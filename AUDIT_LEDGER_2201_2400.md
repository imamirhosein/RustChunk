# RustChunk Starling Audit Ledger — Entries 2201–2400 (175 of 200 recorded)

- Source repository/ref: `starling-client/starling @ 5432370d573642041585c69b40fb045b27133488`
- Audit branch: `starling-1.20.1-port-audit`
- Entry range currently recorded: **2201–2375 (175 unique source files)**.
- Active 200-file stage: **2201–2400 (175/200 complete)**.
- Reporting sections **2201–2250**, **2251–2300** and **2301–2350** are CLOSED (50/50 each); reporting section **2351–2400** is IN PROGRESS (25/50).
- The user has issued a one-time expanded authorization for **2301–2400** (100 files total). Checkpoints **2301–2325**, **2326–2350** and **2351–2375 are COMPLETE**; **2376–2400** remains covered. **STOP at 2400** and request permission before 2401.
- Index order clarification: 70 adjacent transitions relative to the pinned GitHub recursive-tree API order were found among the 2,375 stable Index rows at the time of the original reconciliation: 69 occur in entries 1–2000, and one occurs at entry 2033. No additional raw-tree-position regressions occur from entry 2034 through 2375. This is a historical ordering divergence, not a missing-file, duplicate-path or source-provenance defect: all 2,375 Index paths are unique and present in the pinned tree. The audit IDs are stable historical chronology and must not be renumbered because Ledger, QA and cross-document references depend on them. Do not describe the whole Index as a literal raw recursive-tree traversal. Preserve existing entry numbers; future selection is deterministic lexical order after the latest indexed path.
- Checkpoint cadence: exact 25-file atomic commits, grouped into authorized 50-file sections.
- Every entry contains exactly the 15 required fields from `AUDIT_PROTOCOL.md`. Files over 400 lines must be inspected through EOF in consecutive, gap-free ranges no larger than 400 lines.
- Embedded tests/examples are inspected only; they are not executed.

### 2201. `crates/horizon-theme/icons/pixel/home.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f71cb8c00dc23d618ae4a06ade4c7457a9aa73ff`
- Size: 284 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 284 bytes; no omitted ranges.
- Purpose: Pixelarticons home glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 filled pixel-grid house/home symbol. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: One path using currentColor; static vector only, not a navigation action.
- RustChunk relevance: Optional launcher home/navigation marker; no route or action is included.
- Decision: adaptable; Pixelarticons home glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2202. `crates/horizon-theme/icons/pixel/image.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cf28927704fc5b76583b3fad7ec95cc21ba9ddc7`
- Size: 350 bytes
- Coverage: Complete read through EOF: 4 logical lines; exact pinned-tree size 350 bytes; no omitted ranges.
- Purpose: Pixelarticons image glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 picture/frame symbol composed of two path elements. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Two static paths; no script, embedded image, link or external resource found.
- RustChunk relevance: Optional media/image UI marker; it is not an image decoder or gallery.
- Decision: adaptable; Pixelarticons image glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 4 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2203. `crates/horizon-theme/icons/pixel/info-box.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `04128032b7f76dd9b0e4b54e810185bc107f1b52`
- Size: 203 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 203 bytes; no omitted ranges.
- Purpose: Pixelarticons information glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 information mark inside a square frame. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: One compact currentColor path; no executable or external reference.
- RustChunk relevance: Optional information marker; accessible text must be supplied by the control.
- Decision: adaptable; Pixelarticons information glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2204. `crates/horizon-theme/icons/pixel/message.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c5dc11a7ea529cc5fb485c68b3a7a5d34cd96b4a`
- Size: 190 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 190 bytes; no omitted ranges.
- Purpose: Pixelarticons message glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 rectangular message/chat silhouette. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: One currentColor path; no external reference or message behavior.
- RustChunk relevance: Optional messaging marker; it does not implement chat or networking.
- Decision: adaptable; Pixelarticons message glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2205. `crates/horizon-theme/icons/pixel/monitor.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52edf4d4f7fabdd70c939b4959ad87accbd601d3`
- Size: 203 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 203 bytes; no omitted ranges.
- Purpose: Pixelarticons monitor glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 desktop monitor and stand, expressed as filled pixel rectangles. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static path geometry; no executable or external resource.
- RustChunk relevance: Optional monitor/system marker; it has no device-monitoring behavior.
- Decision: adaptable; Pixelarticons monitor glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2206. `crates/horizon-theme/icons/pixel/notes.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `beb8d97e5bfb8e3aff02f7e3f262de5c540fbdc0`
- Size: 292 bytes
- Coverage: Complete read through EOF: 4 logical lines; exact pinned-tree size 292 bytes; no omitted ranges.
- Purpose: Pixelarticons notes glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 stacked-note/document silhouette composed of two paths. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static currentColor paths; no document storage or external resource.
- RustChunk relevance: Optional notes/document marker; it does not store note content.
- Decision: adaptable; Pixelarticons notes glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 4 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2207. `crates/horizon-theme/icons/pixel/package.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cc6ce07881dd864f8065011ce4956fa0d1cdfa63`
- Size: 350 bytes
- Coverage: Complete read through EOF: 4 logical lines; exact pinned-tree size 350 bytes; no omitted ranges.
- Purpose: Pixelarticons package glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 parcel/package symbol made from two paths. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static vector only; no package-management action or external content.
- RustChunk relevance: Optional package marker; it does not manage mods/packages.
- Decision: adaptable; Pixelarticons package glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 4 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2208. `crates/horizon-theme/icons/pixel/play.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `02e3c17aacfe058e1e8d80ddccddddc9baaad026`
- Size: 228 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 228 bytes; no omitted ranges.
- Purpose: Pixelarticons play glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 pixelated play/forward symbol in one path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: CurrentColor path only; does not launch or control a process.
- RustChunk relevance: Optional play marker; it does not launch Minecraft.
- Decision: adaptable; Pixelarticons play glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2209. `crates/horizon-theme/icons/pixel/plus.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cd35642b9777064cc785293044acf544160876bd`
- Size: 161 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 161 bytes; no omitted ranges.
- Purpose: Pixelarticons plus glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 plus/add symbol using one path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static path; no action, state or external dependency.
- RustChunk relevance: Optional add/create marker; action semantics belong to its parent.
- Decision: adaptable; Pixelarticons plus glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2210. `crates/horizon-theme/icons/pixel/reload.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3458d03f04ce3f7d72a3f903164ca8bc50d03a7a`
- Size: 283 bytes
- Coverage: Complete read through EOF: 5 logical lines; exact pinned-tree size 283 bytes; no omitted ranges.
- Purpose: Pixelarticons reload glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 two-arrow refresh/reload symbol built from three paths. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static vector with currentColor; no network or retry behavior.
- RustChunk relevance: Optional refresh marker; it does not perform retries or downloads.
- Decision: adaptable; Pixelarticons reload glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 5 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2211. `crates/horizon-theme/icons/pixel/search.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7dce1b5ea59a0873d84d232040c590f5e91c241e`
- Size: 291 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 291 bytes; no omitted ranges.
- Purpose: Pixelarticons search glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 pixel-grid magnifying/search symbol. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Single currentColor path; no input or search logic.
- RustChunk relevance: Optional search marker; no search logic is embedded.
- Decision: adaptable; Pixelarticons search glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2212. `crates/horizon-theme/icons/pixel/server.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7979820e718a2dd80c7d0b607527778ddca1866a`
- Size: 211 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 211 bytes; no omitted ranges.
- Purpose: Pixelarticons server glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 rack/server symbol in one path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Only a static symbol; no endpoint or multiplayer behavior.
- RustChunk relevance: Optional server marker; it does not connect to a server.
- Decision: adaptable; Pixelarticons server glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2213. `crates/horizon-theme/icons/pixel/sliders.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c2b4912d02da6e0fac80e2e579d0cb54daaaabcc`
- Size: 215 bytes
- Coverage: Complete read through EOF: 1 logical lines; exact pinned-tree size 215 bytes; no omitted ranges.
- Purpose: Pixelarticons sliders glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 adjustment/sliders symbol in a single-line SVG. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Root declares viewBox and currentColor but omits explicit width and height; parent sizing is required.
- RustChunk relevance: Optional settings/filter marker; it does not expose actual sliders.
- Decision: adaptable; Pixelarticons sliders glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse and render at Android target densities; test intrinsic sizing despite missing width/height, tint and accessibility label; retain Pixelarticons notice.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 1 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2214. `crates/horizon-theme/icons/pixel/star.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a27348730abbb18aad455dbbe35ca66a2ad1c024`
- Size: 318 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 318 bytes; no omitted ranges.
- Purpose: Custom Horizon star/rating glyph
- Dependencies / cross-references: Icon registry in crates/horizon-theme/src/lib.rs (entry 2220); code comment distinguishes this custom star from Pixelarticons.
- Implementation / logic: 24×24 star silhouette made of pixel-like horizontal bands. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: The crate registry labels this as not Pixelarticons and says it was drawn for Horizon ratings. Do not apply the neighboring Pixelarticons notice automatically; standalone asset licensing remains unverified.
- RustChunk relevance: Optional rating visual; custom asset provenance/licensing must be resolved first.
- Decision: undetermined; custom Horizon star glyph; verify its project-level asset license
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Confirm project-level rights for this custom star; test Android rendering, tint/density and accessible rating semantics.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Documented asset provenance/permission, Android rendering screenshots/tests, tint, density and accessibility checks.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2215. `crates/horizon-theme/icons/pixel/terminal.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e000cd10416e3379de9a1e9f4a912e3e09e86111`
- Size: 212 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 212 bytes; no omitted ranges.
- Purpose: Pixelarticons terminal glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 terminal/window symbol with a pixel prompt mark. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static currentColor path only; no terminal execution behavior.
- RustChunk relevance: Optional terminal marker; it does not provide a command console.
- Decision: adaptable; Pixelarticons terminal glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2216. `crates/horizon-theme/icons/pixel/trash.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f67fe4eca7226dfb18b385315d544b0ba3937eda`
- Size: 209 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 209 bytes; no omitted ranges.
- Purpose: Pixelarticons trash glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 waste-bin/delete symbol in one path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static path; second line begins at column zero but is still valid XML. No external reference.
- RustChunk relevance: Optional delete marker; confirmation and destructive behavior belong to the parent.
- Decision: adaptable; Pixelarticons trash glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2217. `crates/horizon-theme/icons/pixel/user.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `13773d116bfe1d91c18172808d8f30e776f711ba`
- Size: 233 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 233 bytes; no omitted ranges.
- Purpose: Pixelarticons user glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 person/profile symbol using one currentColor path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: No account or identity data; static vector only.
- RustChunk relevance: Optional profile marker; it contains no account data.
- Decision: adaptable; Pixelarticons user glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2218. `crates/horizon-theme/icons/pixel/zap.svg`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `93ab75cbf6719f24c30d3f3965c9093d6287db9d`
- Size: 280 bytes
- Coverage: Complete read through EOF: 3 logical lines; exact pinned-tree size 280 bytes; no omitted ranges.
- Purpose: Pixelarticons zap/lightning glyph
- Dependencies / cross-references: Pixelarticons MIT notice at entry 2184; icon registry in crates/horizon-theme/src/lib.rs (entry 2220).
- Implementation / logic: 24×24 angular lightning/energy symbol in one path. Root viewBox is 0 0 24 24 and fill is currentColor.
- Findings: Static geometry only; no performance or power behavior.
- RustChunk relevance: Optional speed/status marker; it does not prove performance.
- Decision: adaptable; Pixelarticons zap glyph; retain its separate MIT notice
- 1.20.1 implications: No Minecraft 1.20.1 or Forge 47.x protocol/rendering behavior; launcher UI asset only.
- Follow-up checks: Parse/render on Android at supported densities and themes; verify tint/clipping and parent accessibility label; retain Pixelarticons MIT notice where applicable.
- RustChunk implementation mapping: Keep in a separately attributed launcher UI icon pack; bind action, focus, hit-target, state and accessibility to parent UI controls.
- Evidence required before adoption: Verified distributed MIT notice where applicable, Android SVG parse/render tests at multiple densities, tint/clipping and accessibility evidence.
- Completion state: COMPLETE — all 3 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2219. `crates/horizon-theme/src/glass.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2b95c14ffadd9926ca1dd659fb589491a7aaec92`
- Size: 10102 bytes
- Coverage: Complete read through EOF: 280 logical lines; exact pinned-tree size 10102 bytes; no omitted ranges.
- Purpose: Glass appearance palette and colour-mixing utilities
- Dependencies / cross-references: Used by theme token generation in crates/horizon-theme/src/lib.rs (entry 2220) and tokens.css (entry 2222).
- Implementation / logic: Exports accent/tint/blur and semantic colours, radii/ink/fill constants, Glass::new, CSS token generation, sRGB↔OKLab and luminance helpers.
- Findings: Glass::new accepts unvalidated tint/blur/accent values; invalid or non-finite values can yield invalid CSS or colour math. on_accent uses a luminance >0.4 heuristic, not a general contrast guarantee. Chrome reference tests are present but unrun.
- RustChunk relevance: Launcher appearance-token reference, not Minecraft rendering or block-state logic.
- Decision: adaptable; palette and OKLab helpers only; validate public numeric inputs
- 1.20.1 implications: No block, item, registry, packet or Forge behavior. Keep launcher/web theme tokens separate from Minecraft's game UI and renderer.
- Follow-up checks: Validate finite tint/blur and 24-bit accent inputs; sweep contrast over many accents; compare CSS output with browser reference.
- RustChunk implementation mapping: Port only a validated palette/mixing module if RustChunk adopts a glass launcher look; do not copy the entire Horizon design system.
- Evidence required before adoption: CSS Color 4/browser comparison, finite-input/property tests and measured contrast evidence.
- Completion state: COMPLETE — all 280 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2220. `crates/horizon-theme/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f15bf05143e85bbcb0b57a2a6e64af08cc02855c`
- Size: 29673 bytes
- Coverage: Complete read to EOF in contiguous, gap-free ranges 1–400, 401–800 and 801–877; 877 logical lines; pinned-tree size 29,673 bytes.
- Purpose: Central Horizon theme crate: fonts, icons, typographic/layout tokens, palettes, motion and CSS output
- Dependencies / cross-references: Includes bundled fonts/icons; declares glass/night submodules; generates the CSS artifact in entry 2222.
- Implementation / logic: Exports font/icon byte registries, Web/Step/Motion/Palette/Blocks types, light/dark palettes, contrast/easing functions and css() generator; includes tests for contrast, icons, bezier and CSS.
- Findings: 877 lines read in 1–400, 401–800 and 801–877. Public float-bearing values and bezier() have no finite/range validation; bisection assumes a valid monotone cubic-bezier x curve. Icon tests only search for viewBox/currentColor substrings rather than parse SVG. CSS-regeneration and WCAG tests exist but were not run. Previously reviewed fonts do not establish Persian/Arabic shaping coverage.
- RustChunk relevance: Strong reference for typed UI tokens and asset registries; launcher/web theme is separate from the Minecraft game UI.
- Decision: adaptable; port selected token/registry patterns only; keep game semantics separate
- 1.20.1 implications: No block, item, registry, packet or Forge behavior. Keep launcher/web theme tokens separate from Minecraft's game UI and renderer.
- Follow-up checks: Test non-finite/range input rejection, SVG parsing, font fallback/shaping, WCAG contrast, bezier behavior and byte-for-byte CSS regeneration.
- RustChunk implementation mapping: Build RustChunk-owned typed UI tokens and asset registry; reuse only selected neutral patterns after font/platform review.
- Evidence required before adoption: Unit/property tests, parser-based SVG validation, exact CSS regeneration, font licensing/fallback and target UI tests.
- Completion state: COMPLETE — all 877 logical lines read through EOF in three consecutive ranges; source SHA and exact pinned-tree byte size match. Tests/builds not run.


### 2221. `crates/horizon-theme/src/night.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3cffde013d9248b8d3bd3cc3992e045acac26863`
- Size: 10113 bytes
- Coverage: Complete read through EOF: 270 logical lines; exact pinned-tree size 10113 bytes; no omitted ranges.
- Purpose: Night/dawn launcher appearance and alpha compositing
- Dependencies / cross-references: Referenced by crates/horizon-theme/src/lib.rs (entry 2220); Night::blocks() supplies the Blocks conversion.
- Implementation / logic: Defines Night palettes, six sky stops, card/stroke alpha, over(), conversion to Blocks and CSS variables; includes sky contrast tests.
- Findings: Public over() does not clamp/validate alpha; public Night fields can carry invalid alpha values, producing extrapolated colours or invalid CSS. Source documentation explicitly says game menus do not use this look. Tests unrun.
- RustChunk relevance: Optional launcher visual theme; not a Minecraft world-sky or game-menu renderer.
- Decision: adaptable; launcher-only theme; validate alpha and keep game-menu separation
- 1.20.1 implications: No block, item, registry, packet or Forge behavior. Keep launcher/web theme tokens separate from Minecraft's game UI and renderer.
- Follow-up checks: Clamp/validate alpha to 0..1; test invalid/non-finite values and compositing/contrast on target displays.
- RustChunk implementation mapping: Optional launcher theme only; never wire its six bands to Minecraft world sky, biome or lighting state.
- Evidence required before adoption: Alpha property tests including invalid inputs, contrast over all sky stops and target-device rendering evidence.
- Completion state: COMPLETE — all 270 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2222. `crates/horizon-theme/tokens.css`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `678c47409daf879df5d2027a943e8795dfe3ec16`
- Size: 7780 bytes
- Coverage: Complete read through EOF: 281 logical lines; exact pinned-tree size 7780 bytes; no omitted ranges.
- Purpose: Generated CSS custom properties for Horizon theme tokens
- Dependencies / cross-references: Generated from crates/horizon-theme/src/lib.rs (entry 2220); comments say do not edit by hand.
- Implementation / logic: Contains light defaults, dark media/attribute overrides, palette/night/glass variables, fonts, type scale, spacing, radii and motion tokens.
- Findings: File identifies itself as generated from the Rust crate and says do not edit. Repeated dark values under media and explicit data-theme selectors are intentional. Regeneration consistency test exists in lib.rs but was not run.
- RustChunk relevance: Web launcher shell reference only; no Minecraft client behavior.
- Decision: adaptable; generated CSS artifact; Rust constants remain authoritative
- 1.20.1 implications: No block, item, registry, packet or Forge behavior. Keep launcher/web theme tokens separate from Minecraft's game UI and renderer.
- Follow-up checks: Regenerate from Rust source and compare byte-for-byte; parse CSS and test media, light, dark and explicit-theme selection.
- RustChunk implementation mapping: Generate web-only CSS from RustChunk token source; native client theme data should remain in Rust/platform UI.
- Evidence required before adoption: Reproducible regeneration, CSS parse/build checks and light/dark/explicit-theme browser snapshots.
- Completion state: COMPLETE — all 281 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2223. `crates/horizon-threads/Cargo.toml`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `346b1c0b9a171f979d52fac0ac8776cbb2ba54b8`
- Size: 1019 bytes
- Coverage: Complete read through EOF: 24 logical lines; exact pinned-tree size 1019 bytes; no omitted ranges.
- Purpose: Build manifest for Horizon thread/allocator support crate
- Dependencies / cross-references: Workspace edition/license/lint settings; mimalloc, libmimalloc-sys, libc and windows-sys.
- Implementation / logic: Declares Rust 1.85, mimalloc/libmimalloc-sys with extended features, libc for Apple/Linux/Android, windows-sys with selected Win32 APIs and optional alloc-census.
- Findings: Allocator/native dependencies are unconditional while OS APIs are target-gated. Android grouping does not prove NDK/ABI compatibility; mimalloc dependency unification and duplicate native-link behavior need checking.
- RustChunk relevance: Possible reference for platform-separated support crates; allocator choices are unproven for RustChunk and target Android.
- Decision: adaptable; manifest layout only; resolve dependencies per target
- 1.20.1 implications: Cargo metadata is not evidence of Android ABI/NDK success or Minecraft/Forge compatibility; validate each target independently.
- Follow-up checks: Resolve Cargo graph and native links per Android ARM64/Windows/Linux; verify NDK and Rust 1.85 workspace compatibility.
- RustChunk implementation mapping: Use target-specific crates/features; keep diagnostic allocator support out of release builds until validated.
- Evidence required before adoption: Per-target Cargo resolution, NDK builds and link map proving one compatible allocator/native library.
- Completion state: COMPLETE — all 24 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2224. `crates/horizon-threads/src/boot_paint.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7effa72209cec2182d781c1eb1592091512a2577`
- Size: 7646 bytes
- Coverage: Complete read through EOF: 195 logical lines; exact pinned-tree size 7646 bytes; no omitted ranges.
- Purpose: Windows GDI boot splash before the wgpu renderer's first frame
- Dependencies / cross-references: windows-sys Win32 GDI/window APIs; caller-supplied Geist font bytes; wgpu first-frame boot transition.
- Implementation / logic: Registers in-memory fonts once, obtains window DC, paints black, centers wordmark/tagline and wraps recent console lines in a 24-row area.
- Findings: Windows GDI only. AddFontMemResourceEx handles/results and GetClientRect/FillRect/DrawTextW/CreateFontW failures are mostly ignored. Font byte lifetime is documented but not enforced by slice signature; log strings have no byte cap and are converted to UTF-16 on each paint. No build/paint test run.
- RustChunk relevance: Boot-placeholder sequencing is useful; this backend cannot run on Android/Linux and needs replacement.
- Decision: rewrite required; keep only boot sequencing idea; replace GDI and harden resource/error handling
- 1.20.1 implications: Directly incompatible with Android/Linux because it depends on Win32/GDI; a separate platform backend is required.
- Follow-up checks: Build/test on Windows; validate API results, font lifetime, GDI handle cleanup, log caps and paint/present concurrency; build separate Android backend.
- RustChunk implementation mapping: Retain boot sequencing concept; implement through Android Surface/native renderer or a platform-neutral interface, not GDI.
- Evidence required before adoption: Windows build and GDI resource/lifetime tests plus separate Android boot-frame/device evidence.
- Completion state: COMPLETE — all 195 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


### 2225. `crates/horizon-threads/src/heap_census.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7fca6225a60ae6d1b775e2f0b603a60b98336e74`
- Size: 10071 bytes
- Coverage: Complete read through EOF: 269 logical lines; exact pinned-tree size 10071 bytes; no omitted ranges.
- Purpose: Feature-oriented allocator wrapper attributing live heap bytes to startup steps and threads
- Dependencies / cross-references: std::alloc GlobalAlloc, mimalloc, TLS and atomics; optional alloc-census diagnostic workflow.
- Implementation / logic: Implements mimalloc-backed GlobalAlloc with an alignment-aware header, tag IDs, TLS step/thread tags, atomic counters, census aggregation and realloc/free tests.
- Findings: Tag is automatically Send although Drop restores thread-local STEP, so cross-thread guard movement can restore the wrong TLS state. layout.size()+header and new_size+header use unchecked arithmetic. census() allocates Vec storage and can perturb its own readings when this allocator is active. Thread naming supports Apple/Linux but not Android, which falls back to unnamed thread. Registry has 512 slots; overflow is folded into untagged. No tests/allocator execution run.
- RustChunk relevance: Profiling reference only; unsafe global allocator requires an independent soundness review before any use.
- Decision: rewrite required; diagnostic-only concept; fix guard mobility, checked sizes, Android tags and self-measurement
- 1.20.1 implications: Allocator instrumentation is below the Minecraft protocol layer and does not implement Forge mod loading or game memory semantics.
- Follow-up checks: Make Tag !Send or bind it to originating thread; checked_add both allocation sizes; exclude census's own allocations; add Android pthread_getname_np; stress allocator invariants with safety tooling.
- RustChunk implementation mapping: If needed, isolate behind a diagnostic-only feature; never replace release allocator without separate soundness review.
- Evidence required before adoption: Allocator soundness review, cross-thread guard/overflow/failure tests, platform thread-name tests and non-perturbing census validation.
- Completion state: COMPLETE — all 269 logical lines read through EOF; source SHA and exact pinned-tree byte size match. No runtime/build test implied.


## Checkpoint 2201–2225 — complete (25/25)

- Entries 2201–2225 were read one at a time through EOF. Exact paths, blob SHAs and byte sizes match the complete pinned tree.
- The 18 pixel SVGs are static currentColor vectors. Pixelarticons' distinct MIT notice is entry 2184; star.svg is explicitly custom Horizon artwork and its independent asset rights remain unresolved.
- Theme code provides Glass/OKLab, Night/Dawn, palettes, font/icon registries, motion and CSS generation. Public floating-point inputs are not range-checked; tests were inspected only.
- The Windows GDI boot painter is not portable to Android. The heap census has unsafe allocator/guard and measurement issues that require a separate soundness review.
- The previously fetched 2201–2207 assets were re-fetched and inspected under the new authorization; they are now valid completed entries.
- Active Ledger: 225 records (2001–2225), exactly 15 fields each. Index: 2,225 sequential unique paths. No app source changed; no tests/builds/APK/device/Forge/server/Aternos/renderer validation.
- Continue at 2226–2250 under the same authorization; preserve 25-file checkpoints and STOP at 2250.

### 2226. `crates/horizon-threads/src/jobs.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b3019b7f708c2285e1c824beb7da91c0f8f3f52e`
- Size: 25495 bytes
- Coverage: Complete read through EOF: 705 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Thread-pool scheduler and dynamic job-source registration
- Dependencies / cross-references: Rust standard library and platform APIs; horizon-threads crate manifest and adjacent worker/memory modules.
- Implementation / logic: Defines worker pool creation, priority/idle scheduling, registration lifecycle, job polling and wake/idle coordination.
- Findings: Pool::new reports at least one worker via started.max(1) even when every OS thread spawn failed, potentially leaving queued work with zero actual workers. A slow/blocking source.next under registration synchronization can delay unregister; dropping a registration from its own job can deadlock. Requires concurrency/race tests around idle/wake and deregistration.
- RustChunk relevance: Potentially useful as isolated platform/threading design or operational test reference; not Minecraft gameplay/protocol code.
- Decision: rewrite required; scheduler ideas only after thread-lifecycle and failure semantics are corrected
- 1.20.1 implications: No Minecraft 1.20.1 registry, chunk, packet or Forge mod-loading implementation.
- Follow-up checks: Inject zero/partial thread-spawn failure; stress register/unregister and idle/wake races; test blocking source.next and self-deregistration without deadlock.
- RustChunk implementation mapping: Keep this as a separate platform/diagnostic layer; implement target-specific interfaces and failure-safe lifecycle in RustChunk rather than importing OS-specific details.
- Evidence required before adoption: Target-specific build/test evidence, injected failure and concurrency tests, Android ARM64/NDK validation, and resource/handle leak checks.
- Completion state: COMPLETE — all 705 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 25495-byte size match. Tests/examples were inspected, not executed.


### 2227. `crates/horizon-threads/src/lib.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `da62d2ba8bd74cd7ee0d99a79a588006fa0223f5`
- Size: 24051 bytes
- Coverage: Complete read through EOF: 590 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Cross-platform thread, process-priority, signal, handle and filesystem helpers
- Dependencies / cross-references: Rust standard library and platform APIs; horizon-threads crate manifest and adjacent worker/memory modules.
- Implementation / logic: Provides OS-specific foreground/idle priority, stop-signal handling, inherited std-handle support, path/junction helpers and allocator/runtime diagnostics.
- Findings: Unix getpriority handling needs errno-aware distinction for a valid -1 priority versus failure. Signal handlers replace prior handlers without restoring them; signal-safety assumptions for atomics and handler work need target verification. Windows junction/path conversions using lossy strings can mangle non-UTF paths. Child std-handle inheritance and Windows process behavior need platform tests.
- RustChunk relevance: Potentially useful as isolated platform/threading design or operational test reference; not Minecraft gameplay/protocol code.
- Decision: adaptable; split pure lifecycle abstraction from OS backends; do not port process-control code wholesale
- 1.20.1 implications: No Minecraft 1.20.1 registry, chunk, packet or Forge mod-loading implementation.
- Follow-up checks: Run per-OS tests for signal install/restore, errno handling, process priority, inherited handles and non-UTF Windows paths; verify Android async-signal safety.
- RustChunk implementation mapping: Keep this as a separate platform/diagnostic layer; implement target-specific interfaces and failure-safe lifecycle in RustChunk rather than importing OS-specific details.
- Evidence required before adoption: Target-specific build/test evidence, injected failure and concurrency tests, Android ARM64/NDK validation, and resource/handle leak checks.
- Completion state: COMPLETE — all 590 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 24051-byte size match. Tests/examples were inspected, not executed.


### 2228. `crates/horizon-threads/src/memory_limit.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `68c8c5b6e930f9f50e1d00ea75964dfb0728ff28`
- Size: 17047 bytes
- Coverage: Complete read through EOF: 449 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Process memory pressure watcher and platform-specific memory-limit controls
- Dependencies / cross-references: Rust standard library and platform APIs; horizon-threads crate manifest and adjacent worker/memory modules.
- Implementation / logic: Tracks configured limits, periodically samples process memory and exposes soft-cap/last-resort shrink signals through platform-specific implementations.
- Findings: Watcher startup uses Once; if initial thread spawn fails, Once is consumed and later attempts cannot restart enforcement. u64::MAX doubles as the no-limit sentinel while Some(u64::MAX) is representable, causing ambiguity. Platform paths and callback semantics need failure-injection tests.
- RustChunk relevance: Potentially useful as isolated platform/threading design or operational test reference; not Minecraft gameplay/protocol code.
- Decision: adaptable; retain only a bounded, restartable memory-pressure interface; reimplement per Android target
- 1.20.1 implications: No Minecraft 1.20.1 registry, chunk, packet or Forge mod-loading implementation.
- Follow-up checks: Inject first watcher spawn failure and verify retry; distinguish no-limit from u64::MAX; test memory sampling, callbacks and shrink lifecycle on Android/Linux/Windows.
- RustChunk implementation mapping: Keep this as a separate platform/diagnostic layer; implement target-specific interfaces and failure-safe lifecycle in RustChunk rather than importing OS-specific details.
- Evidence required before adoption: Target-specific build/test evidence, injected failure and concurrency tests, Android ARM64/NDK validation, and resource/handle leak checks.
- Completion state: COMPLETE — all 449 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 17047-byte size match. Tests/examples were inspected, not executed.


### 2229. `crates/horizon-world/examples/attribute_memory.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b814711ac1dbe873fd8f9f21e32f719323d4999a`
- Size: 1458 bytes
- Coverage: Complete read through EOF: 47 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Held-live attribute allocation workload for external RSS measurement
- Dependencies / cross-references: horizon-world BlockLandingRegistry, collision/movement registries, NBT, entity obstruction and core geometry.
- Implementation / logic: Builds a registry, constructs up to 100,000 zombie attribute containers, optionally ensures four attributes per entity, emits a summary and holds allocations while waiting on stdin.
- Findings: The entity count is bounded, but positional arguments are indexed without arity checks and unknown attribute ids/entities are unwrapped; malformed invocation panics. This is an external profiling workload, not an allocator contract or game runtime.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; profiling example only; do not ship in the client
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Differential-test landing rules, collision shapes, damage sources, sound and event order against Java 1.20.1; validate NBT resources and chunk/entity mutation.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 47 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 1458-byte size match. Tests/examples were inspected, not executed.


### 2230. `crates/horizon-world/examples/behaviour_load_profile.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d9d7a11476d0fbfb31f60b81745b8e27e1bce319`
- Size: 1131 bytes
- Coverage: Complete read through EOF: 32 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Registry and behaviour-resource load-time profiler
- Dependencies / cross-references: horizon-world BlockLandingRegistry, collision/movement registries, NBT, entity obstruction and core geometry.
- Implementation / logic: Loads the client registry, optionally selects a cache directory, loads behaviours three times and reports elapsed milliseconds.
- Findings: Resource path is required with expect; the profile is a one-process timing sample and does not isolate disk cache, warm-up or device variance. It measures Horizon's resource loader, not Minecraft/Forge load time.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; benchmark harness only; repeat with controlled Android/1.20.1 resources before drawing performance conclusions
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Differential-test landing rules, collision shapes, damage sources, sound and event order against Java 1.20.1; validate NBT resources and chunk/entity mutation.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 32 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 1131-byte size match. Tests/examples were inspected, not executed.


### 2231. `crates/horizon-world/examples/chunkgen_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2eb433600d2e75b8efbd75d4268f7b1e0d0a23e0`
- Size: 24990 bytes
- Coverage: Complete read through EOF: 652 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Chunk-generation benchmark and binary output verifier
- Dependencies / cross-references: horizon-world generation/resource registries, NBT/loot systems and CLI argument inputs; depends on prepared client data packs.
- Implementation / logic: Runs seeded generation across configurable work, records NBT-like output/checksums and compares generated candidate output against expected fixture streams.
- Findings: Verifier can accept truncated output: after EOF it may leave still-active candidates marked as matched rather than requiring every expected line to be consumed. This is a correctness flaw in the benchmark's oracle comparison. CLI size/thread/coordinate inputs also lack robust bounds and may cause zero-work, overflow, panic or excessive allocations.
- RustChunk relevance: Benchmark harness only; no shipped runtime value and no verified Android/Minecraft performance measurement.
- Decision: reference; do not trust its pass result until EOF completeness and argument validation are fixed
- 1.20.1 implications: No runtime compatibility claim; these examples consume Horizon-specific resource/registry formats and are not drop-in Forge 1.20.1 tooling.
- Follow-up checks: Add truncated-output and missing-tail regression cases; require every expected line and stream EOF; validate CLI bounds/checked coordinate arithmetic and zero-thread behavior.
- RustChunk implementation mapping: Keep benchmark/oracle code under tools or examples; never include it in the Android runtime dependency graph.
- Evidence required before adoption: Reproducible benchmark corpus, complete output verification, versioned resource hashes, and separate Android device measurements.
- Completion state: COMPLETE — all 652 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 24990-byte size match. Tests/examples were inspected, not executed.


### 2232. `crates/horizon-world/examples/lod_grid_bench.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `4f93737779901b382c93adb73405f37e964fe8a9`
- Size: 4184 bytes
- Coverage: Complete read through EOF: 99 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Far-terrain LOD grid timing and checksum benchmark
- Dependencies / cross-references: horizon-world generation/resource registries, NBT/loot systems and CLI argument inputs; depends on prepared client data packs.
- Implementation / logic: Loads overworld generator resources, warms a worker, samples tiles at selected levels and emits span/shape checksums with optional dump.
- Findings: tiles is not validated: zero leads to division by zero in per-tile metrics; negative values produce misleading/empty work. Level shifts and tile-coordinate multiplication can overflow or panic for invalid levels/large values. The benchmark's 5-column coordinate formula assumes a bounded tile layout.
- RustChunk relevance: Benchmark harness only; no shipped runtime value and no verified Android/Minecraft performance measurement.
- Decision: reference; LOD benchmark idea only; validate parameters and do not treat numbers as Android or Minecraft client performance
- 1.20.1 implications: No runtime compatibility claim; these examples consume Horizon-specific resource/registry formats and are not drop-in Forge 1.20.1 tooling.
- Follow-up checks: Reject tiles<=0 and invalid levels; checked shifts/coordinate math; add deterministic workload size checks and run device-separated benchmarks only after correctness.
- RustChunk implementation mapping: Keep benchmark/oracle code under tools or examples; never include it in the Android runtime dependency graph.
- Evidence required before adoption: Reproducible benchmark corpus, complete output verification, versioned resource hashes, and separate Android device measurements.
- Completion state: COMPLETE — all 99 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 4184-byte size match. Tests/examples were inspected, not executed.


### 2233. `crates/horizon-world/examples/loot_fill.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b1b5ee0d508f95b9fea78953a02e9093bb015c5e`
- Size: 1391 bytes
- Coverage: Complete read through EOF: 36 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Vanilla-format loot-table fill oracle
- Dependencies / cross-references: horizon-world generation/resource registries, NBT/loot systems and CLI argument inputs; depends on prepared client data packs.
- Implementation / logic: Loads loot resources, iterates supplied table names and seeds, fills a 27-slot container and prints item/count/damage/enchantment records.
- Findings: No cap on table/seed argument counts or generated output; missing positional arguments return usage errors but large input lists can create unbounded work/output. It is a CLI oracle, not loot implementation integration evidence.
- RustChunk relevance: Loot-table oracle utility only; no evidence of Forge loot-mod compatibility.
- Decision: reference; fixture/oracle utility only; do not ship
- 1.20.1 implications: No runtime compatibility claim; these examples consume Horizon-specific resource/registry formats and are not drop-in Forge 1.20.1 tooling.
- Follow-up checks: Bound table/seed arguments and output; validate table identifiers and test known Java 1.20.1 loot tables with the correct registry snapshot.
- RustChunk implementation mapping: Keep benchmark/oracle code under tools or examples; never include it in the Android runtime dependency graph.
- Evidence required before adoption: Reproducible benchmark corpus, complete output verification, versioned resource hashes, and separate Android device measurements.
- Completion state: COMPLETE — all 36 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 1391-byte size match. Tests/examples were inspected, not executed.


### 2234. `crates/horizon-world/examples/probe/active_effect_tick_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1e366bf447b2032c345323938b9c54ff936edf62`
- Size: 17616 bytes
- Coverage: Complete read through EOF: 529 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Differential fixture for active-effect ticking, random sources and consume effects
- Dependencies / cross-references: horizon-world active_effects/status_effect hooks; deterministic Java-compatible random sources; adjacent effect/consume fixtures.
- Implementation / logic: Models effect keys/instances, hidden effects, callback events/failures, storage mutation, tick and consume-effect operations; compares deterministic state/event output.
- Findings: Key::tie_break returns Less when identity <= other.identity; if two distinct keys share identity this violates ordering consistency and can corrupt collision-bin ordering assumptions. Extensive unwrap/indexing relies on trusted generated JSON; side is incorporated into output filenames. No fixture corpus or oracle comparison was executed.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; useful behavioral test design only; repair comparator and validate fixture inputs before relying on results
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Make key comparator return Equal for equal identity (or enforce unique identity); fuzz malformed corpus schema; compare callback traces with Java oracle.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 529 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 17616-byte size match. Tests/examples were inspected, not executed.


### 2235. `crates/horizon-world/examples/probe/active_effects_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70395e813bbd4280915c7b910db873de3a4b713e`
- Size: 10216 bytes
- Coverage: Complete read through EOF: 316 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Differential fixture for ActiveEffects map, callbacks and effect identity
- Dependencies / cross-references: horizon-world active_effects/status_effect hooks; deterministic Java-compatible random sources; adjacent effect/consume fixtures.
- Implementation / logic: Serializes effect map, object identity, hidden-chain data and callback events while exercising add/remove/force/blend/get operations.
- Findings: Its JavaKey tie_break uses the same <= comparison; duplicate identity values can return Less in both directions. Input arrays and ids are trusted and indexed/unwrap-heavy; side-based input/output paths are not sandboxed. This is fixture harness behavior, not production collection implementation.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; test-only transport; comparator and input-contract limitations must be recorded
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Make key comparator return Equal for equal identity (or enforce unique identity); fuzz malformed corpus schema; compare callback traces with Java oracle.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 316 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 10216-byte size match. Tests/examples were inspected, not executed.


### 2236. `crates/horizon-world/examples/probe/anchor_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `230b056992e3970ff97329ad53e02c14d026a835`
- Size: 2376 bytes
- Coverage: Complete read through EOF: 71 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Vertical-anchor serialization and resolution comparison fixture
- Dependencies / cross-references: horizon-world worldgen::anchor types and WorldGenerationContext.
- Implementation / logic: Exercises bottom/top/sea-level and absolute/relative anchors against generation contexts, including JSON decoding and resolution outputs.
- Findings: JSON values are unwrapped and narrowed from i64 to i32, so malformed/out-of-range fixture values panic or wrap. No production validation is implemented in this example; actual VerticalAnchor behavior lives in the library.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; use as an oracle shape only; validate numeric ranges in a RustChunk-owned fixture runner
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate i32 conversions/context ranges and JSON shape; differential-test anchor serialization against Java 1.20.1.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 71 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 2376-byte size match. Tests/examples were inspected, not executed.


### 2237. `crates/horizon-world/examples/probe/attribute_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `79cf4e236c12a2201190508bbbe225a7d8ca99c8`
- Size: 7158 bytes
- Coverage: Complete read through EOF: 194 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Attribute modifier state and observer-failure differential fixture
- Dependencies / cross-references: horizon-world::attributes Instance/Definition/Modifier/Observer APIs.
- Implementation / logic: Exercises base values, transient/permanent modifiers, operation grouping, pack/unpack, copying and observer callback errors for two attribute instances.
- Findings: Fixture schema is trusted: it assumes two definitions, valid modifier indices and valid Operation::ALL indices. Numerous unwraps/indexes turn malformed corpus data into panics. No production Attribute implementation is included in this example.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; fixture-only coverage; use library code for behavior and add schema validation to any new oracle
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate fixture schemas, handles and indexes; test callback failure/mutation order against Java reference implementation; keep examples unexecuted until authorized.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 194 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 7158-byte size match. Tests/examples were inspected, not executed.


### 2238. `crates/horizon-world/examples/probe/attribute_health_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `05e779aacbec934e5c980b1aa9e0bac6b9989174`
- Size: 10936 bytes
- Coverage: Complete read through EOF: 309 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Attribute-health and food/health integration fixture
- Dependencies / cross-references: horizon-world attribute_health, live/entity attributes, food and living_health APIs.
- Implementation / logic: Combines entity/live attribute implementations, health synchronization, absorption, food ticking, healing, starvation and injected observer errors.
- Findings: The test adapter assumes catalog keys, two instances, valid operation/difficulty/attribute indexes and parseable bit strings. Invalid fixture data panics; comments explicitly state this fixture does not test cross-attribute hash order.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; useful integration matrix concept only; not a production health subsystem
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate fixture schemas, handles and indexes; test callback failure/mutation order against Java reference implementation; keep examples unexecuted until authorized.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 309 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 10936-byte size match. Tests/examples were inspected, not executed.


### 2239. `crates/horizon-world/examples/probe/attribute_map_queries_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8b7bf110a50f70ba50e13d9403d325b2a49ba0ef`
- Size: 19814 bytes
- Coverage: Complete read through EOF: 504 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Attribute-map query, stored-effect notification and prototype differential fixture
- Dependencies / cross-references: Includes stored_effect_attributes_fixture.rs; uses attribute_map queries, effect_notifications and active_effects.
- Implementation / logic: Includes a shared stored-effect fixture and implements query, lookup, instance edits, modifier notification, refresh and effect integration cases.
- Findings: The fixture relies on generated handle/index invariants and unchecked references into instance/template/supplier arrays. Its key-order/comparator assumptions and input schema are not validated at runtime. The side argument forms fixture filenames directly. No production API behavior is proved by this transport alone.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; fixture coverage only; preserve its modeled call ordering but add bounded and validated inputs
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate fixture schemas, handles and indexes; test callback failure/mutation order against Java reference implementation; keep examples unexecuted until authorized.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 504 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 19814-byte size match. Tests/examples were inspected, not executed.


### 2240. `crates/horizon-world/examples/probe/attribute_updates_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6bf9d815e34d490cc2272a66e4af813768ad0d8f`
- Size: 21523 bytes
- Coverage: Complete read through EOF: 637 logical lines; consecutive, gap-free chunks of at most 400 lines; exact pinned-tree size and SHA verified.
- Purpose: Differential fixture for attribute-update propagation and effect notifications
- Dependencies / cross-references: horizon-world attribute_updates, living_health, effect_notifications, active_effects and ObjectOpenSet.
- Implementation / logic: Models pending update sets, health/absorption state, effect eligibility/add/update/remove, passenger packets and callback failure/mutation sequences.
- Findings: Key::tie_break has the same <= ordering defect under duplicate identities. Many JSON arrays/indexes are trusted; side is interpolated into input/output paths. It tests callback ordering in a mock world, not live network packet compatibility with Minecraft 1.20.1.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; extract ordering scenarios only; validate comparator and fixture schema before oracle use
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Make key comparator return Equal for equal identity (or enforce unique identity); fuzz malformed corpus schema; compare callback traces with Java oracle.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 637 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 21523-byte size match. Tests/examples were inspected, not executed.


### 2241. `crates/horizon-world/examples/probe/basic_feature_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2e6bf9d73822c4d1716e6d50b90356f036ff17a5`
- Size: 6189 bytes
- Coverage: Complete read through EOF: 175 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Basic world-feature placement fixture
- Dependencies / cross-references: horizon-world feature/placement, registry BlockStateId and random sources.
- Implementation / logic: Decodes a basic feature and optional placement modifiers, then places repeatedly against an in-memory mock block world with deterministic pseudo-block lookup and injected read/write failures.
- Findings: The mock height query always returns Unavailable; rem_euclid denominator is fixture-provided and can be zero, and malformed/negative repeat values produce panic or empty-loop behavior. Not a complete terrain/world integration.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; feature placement test harness only; do not treat it as a supported feature implementation
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Reject zero divisor, invalid coordinates and unbounded repeats; compare placement sequence and random draw count with Java oracle.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 175 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 6189-byte size match. Tests/examples were inspected, not executed.


### 2242. `crates/horizon-world/examples/probe/biome_chunk_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e0c7f1686643e1b3b54c3191a8d13f2be647400c`
- Size: 3985 bytes
- Coverage: Complete read through EOF: 104 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: BiomeChunk fill, retrogen and noise-biome query fixture
- Dependencies / cross-references: horizon-world BiomeChunk and RetrogenBiomeResolver; core ChunkPos.
- Implementation / logic: Tests section filling, injected resolver failures, previous-biome retention and coordinate queries using deterministic mock biome ids.
- Findings: Fixture height is narrowed/cast and later used as usize for section loops; negative or implausible dimensions can yield huge iteration. rem_euclid divisor comes from input and can be zero. It is a focused storage/fill fixture, not actual climate biome generation.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; retain section/retrogen scenario coverage only; validate dimensions and divisors
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate min/height/divisors and cap section loops; test partial-fill failure and retrogen behavior across negative world heights.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 104 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 3985-byte size match. Tests/examples were inspected, not executed.


### 2243. `crates/horizon-world/examples/probe/biome_generation_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f5a609a5c3f44289e2f6c9fe93029f9aa2572c8d`
- Size: 5229 bytes
- Coverage: Complete read through EOF: 129 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Seeded climate-router and biome-source generation fixture
- Dependencies / cross-references: horizon-core density graph, climate, biome zoom and horizon-world biome_source/router/biome_chunk.
- Implementation / logic: Loads noise/density/preset registry JSON, compiles a seeded density graph, resolves fixed/checkerboard/end/preset sources, fills biome chunks and queries block-to-quart zoom.
- Findings: Uses min + height - 1 and height conversions without local bounds validation; invalid dimensions could overflow or create excessive loops. Data is loaded from a synthetic fixture bundle, so it is not by itself proof of exact vanilla 1.20.1 registry/codec parity.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; useful generator test design; pin registries and validate bounds before adoption
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Use exact versioned 1.20.1 noise/density/preset registries; validate height math and compare seeded samples to Java generator outputs.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 129 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 5229-byte size match. Tests/examples were inspected, not executed.


### 2244. `crates/horizon-world/examples/probe/biome_placement_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `36a8cb7e6d36a44fe15dce683bf1d075a57552f9`
- Size: 6106 bytes
- Coverage: Complete read through EOF: 175 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Biome-filtered feature-placement and nested callback fixture
- Dependencies / cross-references: horizon-world FeaturePlacer/PlacementModifier and core random generators.
- Implementation / logic: Exercises direct modifier invocation, contextual placement, top-feature identity, query/feature failures, nested placement and legacy/worldgen random streams.
- Findings: When nested=true, the feature callback recursively invokes another biome placement with the same nested flag; if the biome predicate keeps accepting and no injected feature failure terminates it, recursion can grow without bound. Direct mode only invokes modifiers[0], explicitly relying on a unit-biome-filter corpus rather than arbitrary modifier chains.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; use only for controlled oracle scenarios; add recursion depth/cycle guard in any new test harness
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Add nested-depth/cycle limit and exercise direct modifier chains beyond the single-filter corpus; compare random-consumption and biome query order.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 175 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 6106-byte size match. Tests/examples were inspected, not executed.


### 2245. `crates/horizon-world/examples/probe/biome_source_codec_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70347bbc6c385af91de16f2e132303b821a60219`
- Size: 3019 bytes
- Coverage: Complete read through EOF: 79 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Registry-aware biome-source encode/decode fixture
- Dependencies / cross-references: horizon-world BiomeSourceCatalog, tags, climate sampler and density registry.
- Implementation / logic: Builds biome and tag catalogues, decodes and re-encodes sources, lists possible biomes and samples deterministic resolver positions.
- Findings: Normalization converts every JSON number through f64 before comparing bits; integer JSON values above 2^53 lose precision and distinct values may collapse. This is a differential-oracle blind spot if large integer fields enter the schema; `assert!` also assumes no tag names are missing.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; codec oracle only after preserving integer-vs-float JSON representation
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Preserve integer JSON numbers exactly; avoid f64-normalizing all numeric values; add codec round-trip tests for large integers and missing tags.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 79 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 3019-byte size match. Tests/examples were inspected, not executed.


### 2246. `crates/horizon-world/examples/probe/biome_source_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `021e67a0fc2ab3f4d4491342943bac8931e436c7`
- Size: 2794 bytes
- Coverage: Complete read through EOF: 79 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Fixed, checkerboard and End biome-source differential fixture
- Dependencies / cross-references: FixedBiomeSource, CheckerboardBiomeSource, EndBiomeSource and mock DensitySampler.
- Implementation / logic: Samples source implementations at many positions, traces erosion sampler calls and tests sampler failure paths.
- Findings: Fixture JSON and position/biome indexes are unwrapped; checkerboard scale and erosion bit patterns depend on trusted corpus. It isolates source algorithms but not registry codec or full generation parity.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; focused source oracle only; keep separate from full generator validation
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate positions/scales and compare End erosion sampling/cache call order against Java implementation.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 79 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 2794-byte size match. Tests/examples were inspected, not executed.


### 2247. `crates/horizon-world/examples/probe/biome_temperature_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5b367947d4a342100f226eeffd5a9c5299250e1`
- Size: 4590 bytes
- Coverage: Complete read through EOF: 119 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Biome temperature, weather threshold and cache fixture
- Dependencies / cross-references: horizon-world biome_temperature settings/cache and core BlockPos.
- Implementation / logic: Exercises temperature adjustment, precipitation decisions, warm/rain, cold/snow, frozen-iceberg checks, cache fills, cache dumps and thread-local worker recreation.
- Findings: Input JSON/hex strings and operation indexes are unvalidated. Negative fill counts silently run no loop; thread operation creates a fresh worker, so it does not prove cross-thread cache sharing or parallel safety.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; temperature oracle scenarios only; distinguish thread recreation from concurrent worker behavior
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate hex float and operation schema; test cache semantics with genuine concurrent access rather than a fresh worker per nested thread operation.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 119 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 4590-byte size match. Tests/examples were inspected, not executed.


### 2248. `crates/horizon-world/examples/probe/block_collision_scan_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b561a84759bf910aa7251b44fd8410e6a6fb5587`
- Size: 6919 bytes
- Coverage: Complete read through EOF: 199 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Block-collision scanning and ground-support fixture
- Dependencies / cross-references: horizon-world BlockCollisionScan, CollisionContext, VoxelShape and ground-support APIs.
- Implementation / logic: Supplies deterministic mock chunks/blocks/shapes, suffocation flags, injected errors, scan hit iteration and supporting-block search.
- Findings: Palette/state indices and AABB bit inputs are trusted; a malformed fixture may panic. The one-million scan budget bounds the harness but does not establish production-world chunk loading, collision-shape correctness or target-version parity.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; useful collision scenario generator only; validate shape and chunk bounds against target 1.20.1
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Validate palette indexes/AABBs and chunk-missing semantics; compare collision scan order and support search with Java 1.20.1 world collision results.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 199 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 6919-byte size match. Tests/examples were inspected, not executed.


### 2249. `crates/horizon-world/examples/probe/block_entity_storage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `687eed79077b8ba9d78272a65ebe61ee2b71a7f9`
- Size: 6102 bytes
- Coverage: Complete read through EOF: 169 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Prototype block-entity active/pending storage fixture
- Dependencies / cross-references: horizon-world ProtoBlockEntities and NBT Tag; line-oriented test input protocol.
- Implementation / logic: Exercises chunk resets, pending/active positions, marker tags, mutation, save conversion, sorted dump and size counts.
- Findings: Input::tag reads a value token before checking the kind; a `none` kind therefore still needs a placeholder token or it panics. `pending()` panics when marker is absent/not Long. split_whitespace cannot preserve arbitrary strings with spaces. This is a strict fixture line protocol, not a robust NBT reader.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; block-entity scenario harness only; do not reuse its parser for untrusted NBT
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Use a typed parser with explicit token counts and bounded NBT decoding; test malformed marker tags, `none` encoding and arbitrary string values.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 169 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 6102-byte size match. Tests/examples were inspected, not executed.


### 2250. `crates/horizon-world/examples/probe/block_landing_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a6ab6ac33893265dfa165a807bfe8a60b0f14633`
- Size: 10826 bytes
- Coverage: Complete read through EOF: 310 logical lines; full file within one read; exact pinned-tree size and SHA verified.
- Purpose: Block landing, bounce, fall damage, sound and push integration fixture
- Dependencies / cross-references: horizon-world BlockLandingRegistry, collision/movement registries, NBT, entity obstruction and core geometry.
- Implementation / logic: Loads block/movement/collision registries from NBT, builds landing definitions and simulates collision queries, entity AABB pushes, teleportation, sound/damage/event and block mutation with injected failures.
- Findings: The mock `collision_shape` expects registered cached shapes and panics if assumptions fail. Entity/world flags and binary NBT inputs are trusted; this harness is not a full game server nor a target-1.20.1 packet/event test. No example was executed.
- RustChunk relevance: Fixture/oracle only: useful for designing differential tests against Java 1.20.1 semantics, but not itself a production subsystem.
- Decision: reference; integration-oracle design only; independently validate entity collision and block effects against Java 1.20.1
- 1.20.1 implications: Potentially relevant to reconstructing 1.20.1 behavior, but fixture inputs and mock interfaces do not establish vanilla/Forge 47.x parity or Aternos connectivity.
- Follow-up checks: Differential-test landing rules, collision shapes, damage sources, sound and event order against Java 1.20.1; validate NBT resources and chunk/entity mutation.
- RustChunk implementation mapping: Translate the scenario matrix into RustChunk-owned version-pinned differential tests; implement actual behavior from Java 1.20.1/Forge evidence, not by copying the mock fixture.
- Evidence required before adoption: Java 1.20.1 reference outputs, version-pinned registry/packet data, valid/invalid corpus coverage and explicit test results; no Aternos claim without live connection tests.
- Completion state: COMPLETE — all 310 logical lines read through EOF in consecutive ranges no larger than 400 lines; exact pinned-tree SHA and 10826-byte size match. Tests/examples were inspected, not executed.


## Checkpoint 2226–2250 — complete (25/25); section 2201–2250 closed

- Entries 2226–2250 were inspected individually to EOF in serial order. Long files were covered in contiguous, gap-free chunks no larger than 400 lines. Every path, source SHA and byte size matches the complete pinned recursive tree.
- The jobs, memory-limit and cross-platform thread helpers expose worker-start, unregister/wake, sentinel and platform API concerns that need failure/concurrency tests before any reuse.
- The chunkgen benchmark contains an output-verifier EOF flaw that can accept truncated results; benchmark results must not be trusted until corrected.
- The remaining examples are benchmark or differential-fixture transports with trusted-corpus assumptions. They are not shipped runtime features. A fixture key comparator can violate ordering when distinct keys have equal identity; the biome source codec's f64 normalization can lose large integer precision; nested placement mode can recurse without bound if its predicates keep accepting.
- No test/example was executed. No application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, multiplayer-server or Aternos validation was performed.
- Ledger contains **250 records (2001–2250)** with exactly 15 fields each. Index contains **2,250 sequential unique paths** with matching Decisions. Reporting section **2201–2250 is CLOSED (50/50)**.
- The one-time authorization is exhausted. **STOP at 2250** and require fresh explicit permission before entry 2251.

### 2251. `crates/horizon-world/examples/probe/block_properties_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6169041a2fc894023eb44d204e0d7443a0fa37e5`
- Size: 6032 bytes
- Coverage: Complete read through EOF: 160 logical lines; full file within one read; exact pinned-tree size 6032 bytes.
- Purpose: Block-property registry, solidity, render flags and occlusion comparison fixture
- Dependencies / cross-references: horizon-world block-property/support/survival registries; block and collision NBT; core geometry and registry IDs.
- Implementation / logic: Loads synthetic block-state, cached-collision and block-property NBT; checks solid/replaceable/liquid/solid-render values, face occlusion masks, cached bounds, malformed NBT and standalone shape-derived properties.
- Findings: The fixture asserts property/set agreement, but registry and JSON assumptions are untrusted-index prone: bbox indexes six numbers, fact states and names are unwrapped, and values are cast to i32/u32. Negative or malformed inputs panic. It validates a synthetic resource bundle, not the full target-version block-property registry.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate fact schema, numeric ranges and exact state ordering; compare every property and face result against Java 1.20.1; test incomplete/mismatched registry snapshots.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 160 logical lines read through EOF; exact pinned-tree SHA and 6032-byte size match. Example inspected, not executed.


### 2252. `crates/horizon-world/examples/probe/block_push_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `678e0fa241499ded7e45949010d7ad0a39c06f18`
- Size: 3862 bytes
- Coverage: Complete read through EOF: 113 logical lines; full file within one read; exact pinned-tree size 3862 bytes.
- Purpose: Entity push-up behavior when a block's collision shape changes
- Dependencies / cross-references: horizon-world block-property/support/survival registries; block and collision NBT; core geometry and registry IDs.
- Implementation / logic: Models old/new VoxelShapes, entity bounding boxes, ordered entity queries, relative teleports, an injected failure and optional mutation of a later entity during an earlier teleport.
- Findings: Collision state ids directly index a two-element shape array; entity identity directly indexes the entity vector; empty/malformed arrays panic. The mock is a compact push_entities_up harness and does not model entity broad-phase/chunk residency.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate shape/state and entity indexes; compare push ordering, displacement and partial-failure state to Java 1.20.1 Block.pushEntitiesUp.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 113 logical lines read through EOF; exact pinned-tree SHA and 3862-byte size match. Example inspected, not executed.


### 2253. `crates/horizon-world/examples/probe/block_state_codec_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `40d51d4c24afeec4baf31c0e35d42b594c3f290a`
- Size: 1955 bytes
- Coverage: Complete read through EOF: 60 logical lines; full file within one read; exact pinned-tree size 1955 bytes.
- Purpose: Block-state JSON encode/decode differential fixture
- Dependencies / cross-references: horizon-world block-property/support/survival registries; block and collision NBT; core geometry and registry IDs.
- Implementation / logic: Builds a BlockRegistry from a synthetic NBT compound, preserves selected integer properties via a whitelist, encodes all registry states with default/non-default modes and decodes supplied JSON rows.
- Findings: The `integers` whitelist and state facts are generated fixture inputs; arrays and field types are unwrapped. It does not load the canonical 1.20.1 block registry or establish Forge-added state-property compatibility.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Use version-hashed 1.20.1 registry data; test unknown properties, defaults, integer/string representation and malformed JSON against Java's BlockState codec.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 60 logical lines read through EOF; exact pinned-tree SHA and 1955-byte size match. Example inspected, not executed.


### 2254. `crates/horizon-world/examples/probe/block_support_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5d58fe159f233657a5f1abb9010b54f7ded6ddf7`
- Size: 2386 bytes
- Coverage: Complete read through EOF: 69 logical lines; full file within one read; exact pinned-tree size 2386 bytes.
- Purpose: Cached sturdy-face and block-support registry fixture
- Dependencies / cross-references: horizon-world block-property/support/survival registries; block and collision NBT; core geometry and registry IDs.
- Implementation / logic: Loads block registry and cached-support NBT, reports cached/uncached states and Full/Center/Rigid support by face; injects a bad Kind and invalid state id.
- Findings: Hard-coded assertions require exactly 35,723 registry states and 35,524 cached states; this only fits the corresponding pinned data bundle. All per-state queries assume that corpus. It is not a generic registry validator.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Remove corpus-specific cardinality from reusable tests; verify support-cache generation/version, face semantics and uncached fallback against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 69 logical lines read through EOF; exact pinned-tree SHA and 2386-byte size match. Example inspected, not executed.


### 2255. `crates/horizon-world/examples/probe/block_survival_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b357d157795f578362fed1eeff7baa4ce75a2931`
- Size: 10842 bytes
- Coverage: Complete read through EOF: 279 logical lines; full file within one read; exact pinned-tree size 10842 bytes.
- Purpose: Block survival rule evaluation with neighbor, fluid, light and dynamic-entity queries
- Dependencies / cross-references: horizon-world block-property/support/survival registries; block and collision NBT; core geometry and registry IDs.
- Implementation / logic: Builds registries for block properties, support, collision, tags and fluids; decodes declarative SurvivalRule mappings and records neighbor/entity/light/fluid query order with injectable failures.
- Findings: The mock's `neighbors[old % len]` and `fluid_states[old % len]` panic on empty vectors; fixture rules and indexes are unwrapped; unknown rule names panic. A synthetic world returns fixed/mock entities, brightness, fluids and tags, so it does not prove full world survival behavior.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate nonempty bounded fixture arrays, rule schemas and ids; compare all rule families, query order, failure propagation and dynamic support against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 279 logical lines read through EOF; exact pinned-tree SHA and 10842-byte size match. Example inspected, not executed.


### 2256. `crates/horizon-world/examples/probe/block_update_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e0b10ed9c2b24e279a39cad12f019dfd7bf4a04a`
- Size: 18867 bytes
- Coverage: Complete read through EOF: 474 logical lines in consecutive, gap-free ranges 1–400 and 401–474; exact pinned-tree size 18,867 bytes.
- Purpose: Broad differential harness for block shape updates, random/scheduled ticks, neighbor changes and block mutation
- Dependencies / cross-references: horizon-world block update, item durability, blocking rules and combat APIs; serde_json fixture transport and deterministic callbacks.
- Implementation / logic: Loads Behaviours and a map-backed mock level; replays shape/random/block/fluid/neighbor/run operations, block and fluid tick queues, removals, loot/event side effects and RNG continuation.
- Findings: This harness deliberately simplifies the world: `is_loaded` always true, entity counts are zero, block entities are treated as fresh with hard-coded behavior, and a fixed -3..=3 chunk area is initialized. JSON positions/arrays/state ids and operation names are trusted and often unwrap/index directly. It cannot establish full vanilla/Forge world scheduling or multiplayer parity.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Port the scenario matrix as version-pinned differential tests; model real loaded/unloaded chunks, block entities, entities, game rules, event recipients and registry data before claiming parity.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 474 logical lines read through EOF; exact pinned-tree SHA and 18867-byte size match. Example inspected, not executed.


### 2257. `crates/horizon-world/examples/probe/blocking_item_damage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `eb2a6de60505176c35ab068c6b0484bfe887fa1a`
- Size: 4835 bytes
- Coverage: Complete read through EOF: 148 logical lines; full file within one read; exact pinned-tree size 4835 bytes.
- Purpose: Blocking-item durability and callback-flow fixture
- Dependencies / cross-references: horizon-world block update, item durability, blocking rules and combat APIs; serde_json fixture transport and deterministic callbacks.
- Implementation / logic: Combines DurabilityWorld and BlockingItemDamageWorld to exercise player/server/client flags, durability snapshots, break callbacks, injected callback failure and state mutation.
- Findings: `process_enchantments` intentionally returns `unenchanted_change`, so enchantment durability adjustment is not covered. JSON values are unwrapped and narrowed to i32/u8; it does not represent a full inventory/hand/weapon runtime.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Test enchantment-aware damage, stack mutation and client/server branching against Java 1.20.1; validate bit-string inputs and numeric ranges.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 148 logical lines read through EOF; exact pinned-tree SHA and 4835-byte size match. Example inspected, not executed.


### 2258. `crates/horizon-world/examples/probe/blocking_rules_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `19d3bb8d59ee04016b65484b31a20d530aa47187`
- Size: 3324 bytes
- Coverage: Complete read through EOF: 99 logical lines; full file within one read; exact pinned-tree size 3324 bytes.
- Purpose: Blocking damage rules and damage-reduction type matching fixture
- Dependencies / cross-references: horizon-world block update, item durability, blocking rules and combat APIs; serde_json fixture transport and deterministic callbacks.
- Implementation / logic: Exercises delay/disable tick conversion, item damage functions, ordered damage reductions, angle/type matching, source-holder queries and injected failure/mutation.
- Findings: The synthetic TypeMatchWorld supports only set ids 1–4; any other set panics. The mock returns static ids and does not model item tags, full damage sources or runtime enchantments. Inputs are trusted and narrowed.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate rule schema and supported type sets; compare numeric thresholds, reduction order, angle boundaries and server/client decisions with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 99 logical lines read through EOF; exact pinned-tree SHA and 3324-byte size match. Example inspected, not executed.


### 2259. `crates/horizon-world/examples/probe/bulk_section_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a7bcc834a40734b6ff334b85f4379677cbd7f7f9`
- Size: 6811 bytes
- Coverage: Complete read through EOF: 199 logical lines; full file within one read; exact pinned-tree size 6811 bytes.
- Purpose: Multi-chunk terrain bulk access, cached section acquisition and heightmap fixture
- Dependencies / cross-references: horizon-world worldgen TerrainChunk/TerrainRegion/Heightmap, registry/tag and item template/component APIs.
- Implementation / logic: Creates TerrainChunks and a TerrainRegion, wraps TerrainBulkAccess and exercises height/get/world/has/set/prime/reset, acquired-section counts, raw heightmap words and block-section dumps.
- Findings: Fill chooses `states[((x+3*y+5*z+salt) as usize) % states.len()]`, which panics on empty input and can overflow/cast unexpectedly; height/min and registry indexes are unvalidated. Dumps can produce very large output. The region is synthetic and does not model live chunk ticket/loading behavior.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate chunk/section bounds, positive dimensions, nonempty state sets and bounded dump size; compare border behavior, section cache lifetime and heightmaps with Java 1.20.1 chunks.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 199 logical lines read through EOF; exact pinned-tree SHA and 6811-byte size match. Example inspected, not executed.


### 2260. `crates/horizon-world/examples/probe/bundle_weight_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8e077b80763803e32bded242c3f272711e54dfae`
- Size: 5486 bytes
- Coverage: Complete read through EOF: 158 logical lines; full file within one read; exact pinned-tree size 5486 bytes.
- Purpose: Bundle weight fractions, nested content weighting and strict item-template validation fixture
- Dependencies / cross-references: horizon-world worldgen TerrainChunk/TerrainRegion/Heightmap, registry/tag and item template/component APIs.
- Implementation / logic: Tests Fraction creation/checked addition/multiplication, nested BundleWeight, prototype max-stack updates and item-template contents validation, including container/bundle/projectile references.
- Findings: Nested bundle references are assembled only from already-created bundles, so this fixture cannot express forward/cyclic references. Bundle/index/prototype references and counts are trusted and unchecked; signed counts/maxima are narrowed to i32. It is test transport, not item inventory code.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Add malformed/oversized counts, empty/nested/cycle cases and maximum stack rules; compare exact Fraction overflow and bundle capacity outcomes with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 158 logical lines read through EOF; exact pinned-tree SHA and 5486-byte size match. Example inspected, not executed.


### 2261. `crates/horizon-world/examples/probe/canyon_carver_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `76eaad452e3bbc9bdd65db9cb684fe0747c94c0d`
- Size: 3453 bytes
- Coverage: Complete read through EOF: 99 logical lines; full file within one read; exact pinned-tree size 3453 bytes.
- Purpose: Seeded canyon-carver output, ordered carving writes and RNG continuation fixture
- Dependencies / cross-references: horizon-core carving geometry/mask, chunk positions and legacy/xoroshiro random; horizon-world cave/canyon carver definitions.
- Implementation / logic: Decodes CanyonCarver definitions, records start-chunk checks, carve callbacks, mask columns, injected output failure and next/tail random values for LegacyRandom or XoroshiroRandom.
- Findings: World-generation bounds compute `min_y + 1` and `min_y + height - 8` without checked arithmetic; context/target/origin/repeat values are trusted. Mask writes unwrap and can panic if callback coordinates escape bounds. No Java oracle was run.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Test negative world heights, extreme bounds, target/origin reach, callback failure partial masks and exact Java Random consumption against 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 99 logical lines read through EOF; exact pinned-tree SHA and 3453-byte size match. Example inspected, not executed.


### 2262. `crates/horizon-world/examples/probe/carving_geometry_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `52582b45b2e794ef99d61b43f0b771b42d338733`
- Size: 3763 bytes
- Coverage: Complete read through EOF: 113 logical lines; full file within one read; exact pinned-tree size 3763 bytes.
- Purpose: Shared ellipsoid carving geometry and chunk-reach fixture
- Dependencies / cross-references: horizon-core carving geometry/mask, chunk positions and legacy/xoroshiro random; horizon-world cave/canyon carver definitions.
- Implementation / logic: Runs carve_ellipsoid with traced skip predicates and ordered callback failures, emits carving-mask columns and evaluates can_reach cases using bit-preserved f64 parameters.
- Findings: Fixture JSON/hex values and coordinate ranges are trusted; `mask.carve(...).unwrap()` can panic for out-of-range callbacks. Skip mode and synthetic normalized geometry cover only selected predicate variants.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate AABB/ellipsoid extents, negative coordinates, boundaries and partial failures; compare exact carved voxel order and reach decisions to Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 113 logical lines read through EOF; exact pinned-tree SHA and 3763-byte size match. Example inspected, not executed.


### 2263. `crates/horizon-world/examples/probe/carving_mask_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `17c82fd3c80b48b2022c657ec8c7117ca943ba83`
- Size: 3139 bytes
- Coverage: Complete read through EOF: 76 logical lines; full file within one read; exact pinned-tree size 3139 bytes.
- Purpose: CarvingMask traversal, fill and callback mutation fixture
- Dependencies / cross-references: horizon-core carving geometry/mask, chunk positions and legacy/xoroshiro random; horizon-world cave/canyon carver definitions.
- Implementation / logic: Exercises mask construction, point/box carving, visit traversal with in-callback mutation/failure and subsequent column reads/emptiness.
- Findings: Fill loops can expand without a practical size cap and iterate attacker-controlled coordinate ranges; callback tracing uses unwrap and unknown operation paths are unreachable. This is a focused in-memory mask transport.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate height/range and operation quotas; test mutation during visit, duplicate points, empty masks and boundary coordinates against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 76 logical lines read through EOF; exact pinned-tree SHA and 3139-byte size match. Example inspected, not executed.


### 2264. `crates/horizon-world/examples/probe/cave_carver_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `79917c336977b0f7f34a18ae2abea5a75071dc1d`
- Size: 3447 bytes
- Coverage: Complete read through EOF: 99 logical lines; full file within one read; exact pinned-tree size 3447 bytes.
- Purpose: Seeded cave-carver output, ordered carving writes and RNG continuation fixture
- Dependencies / cross-references: horizon-core carving geometry/mask, chunk positions and legacy/xoroshiro random; horizon-world cave/canyon carver definitions.
- Implementation / logic: Decodes CaveCarver definitions and replays start-chunk/carve calls to a mock CarverOutput; records mask columns, injected failures and LegacyRandom/Xoroshiro continuation.
- Findings: Same fixture risks as canyon: unchecked min_y/height arithmetic, unbounded repeats/input, trusted target/origin and `mask.carve(...).unwrap()` on callbacks. The corpus is not a target-version registry/seed parity proof.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Add extreme-height and callback-out-of-range tests; compare cave path geometry, carve order and RNG draws against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 99 logical lines read through EOF; exact pinned-tree SHA and 3447-byte size match. Example inspected, not executed.


### 2265. `crates/horizon-world/examples/probe/chunk_stage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2929de2f117dd0af3fbc063f2e29c342500f90a9`
- Size: 9414 bytes
- Coverage: Complete read through EOF: 271 logical lines; full file within one read; exact pinned-tree size 9414 bytes.
- Purpose: Pre-light TerrainChunk stages, below-zero retrogen, block fill and heightmap transition fixture
- Dependencies / cross-references: horizon-world ChunkStatus, BelowZeroRetrogen and TerrainChunk; core NBT/registry/tag data.
- Implementation / logic: Builds a registry from server facts and tags, sets chunk stages/retrogen masks, runs old-bedrock replacement, fills/sets/gets blocks, primes and queries heightmaps, then dumps raw heightmap words and sections.
- Findings: The fill path divides/modulos by `states.len()` and panics on empty arrays; dimensions, state ids, mask words and `i32`→`u32` casts are trusted. `height / 16` and full section dumps can create large loops/output. It is not full chunk-status scheduling/loading.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate section-aligned bounds, block IDs, masks, nonempty state lists and bounded dumps; compare retrogen/heightmap stage semantics with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 271 logical lines read through EOF; exact pinned-tree SHA and 9414-byte size match. Example inspected, not executed.


### 2266. `crates/horizon-world/examples/probe/chunk_status_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e124517fa5850836035986bc87a89c5ca687004f`
- Size: 2378 bytes
- Coverage: Complete read through EOF: 69 logical lines; full file within one read; exact pinned-tree size 2378 bytes.
- Purpose: ChunkStatus ordering, heightmap metadata and NBT codec fixture
- Dependencies / cross-references: horizon-world ChunkStatus, BelowZeroRetrogen and TerrainChunk; core NBT/registry/tag data.
- Implementation / logic: Prints every ChunkStatus index/name/parent/level/map metadata and pairwise ordering; compares PreLightChunkStage mappings and tests selected NBT tag decode/encode rows.
- Findings: CLI arguments are indexed without an arity guard; JSON row fields are unwrapped and integer inputs narrowed. It queries only fixed synthetic tag variants; this does not validate real chunk files or async chunk scheduling.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Use target-version enumerations and real 1.20.1 NBT samples; validate malformed tag handling, ordering, level mappings and stage-heightmap associations.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 69 logical lines read through EOF; exact pinned-tree SHA and 2378-byte size match. Example inspected, not executed.


### 2267. `crates/horizon-world/examples/probe/climbing_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `48a28b8d345a08861de56a6b626dab56b4066a7c`
- Size: 5168 bytes
- Coverage: Complete read through EOF: 152 logical lines; full file within one read; exact pinned-tree size 5168 bytes.
- Purpose: Climbing predicates, tag binding combinations and position callback fixture
- Dependencies / cross-references: horizon-world ClimbingRegistry and ClimbingWorld; block registry and tag-derived state sets.
- Implementation / logic: Loads block registry, constructs combinations of climbable/glide-through/trapdoor sets, checks invalid tags/state ids and records spectator/position/state/glide/below-block query order plus mutation/failure.
- Findings: Input tag arrays and 0..2 binding selectors are unchecked; registry lookups and JSON coordinates are unwrapped/narrowed. The mock models a small set of world queries rather than real entity movement, step-up or collision resolution.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Compare climbable tags and on_climbable behavior to vanilla 1.20.1; add malformed tag and movement/position transition cases.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 152 logical lines read through EOF; exact pinned-tree SHA and 5168-byte size match. Example inspected, not executed.


### 2268. `crates/horizon-world/examples/probe/collision_context_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f26137feff0e91fc22f665a13bcea7abca1442cb`
- Size: 16908 bytes
- Coverage: Complete read through EOF: 450 logical lines in consecutive, gap-free ranges 1–400 and 401–450; exact pinned-tree size 16,908 bytes.
- Purpose: CollisionContext modes and fluid, powder-snow, scaffolding and dynamic-shape fixture
- Dependencies / cross-references: horizon-world collision contexts, cached/dynamic collision, fluids, powder snow, support and block registry.
- Implementation / logic: Covers empty/position/entity/placement contexts, held-item flags, descending/above checks, liquid shape queries, powder-snow rules, scaffolding and registry-backed dynamic collision shape behavior.
- Findings: 450 lines were read as 1–400 and 401–450. Many JSON enum/index fields are directly indexed/unwrapped; the mock fluid/entity calls use fixed snapshots and some callbacks bypass failure instrumentation. Registry-backed modes require exact fixture NBT. This is not a complete entity/world collision simulation.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Add schema validation, real entity/item/FluidState edge cases, chunk/world failure semantics and Java 1.20.1 differential outputs for every context mode.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 450 logical lines read through EOF; exact pinned-tree SHA and 16908-byte size match. Example inspected, not executed.


### 2269. `crates/horizon-world/examples/probe/collision_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b2601f1374d656202ba9b4cb68cad50a6f7e46d4`
- Size: 7369 bytes
- Coverage: Complete read through EOF: 202 logical lines; full file within one read; exact pinned-tree size 7369 bytes.
- Purpose: Cached collision shapes, canonical shape identity and geometric query fixture
- Dependencies / cross-references: horizon-world CachedCollisionRegistry, BlockSection and NBT; horizon-core AABB/VoxelShape geometry.
- Implementation / logic: Imports cached collision NBT, writes every block state into a BlockSection, inspects cached/uncached shape identity, cube axes, extrema, bounds and boxes, then tests collide/clip/closest-point/find-index operations.
- Findings: The line-based SplitWhitespace parser unwraps token counts and numeric formats then asserts no extra tokens. State IDs and f64 hex are unvalidated; imported cache is synthetic/version-bound. Pointer identity observations are process-local and not a renderer/GPU property.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate parser and state ids; compare geometry tolerances, zero-volume shapes, negative coordinates and Java collision ordering using version-pinned 1.20.1 caches.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 202 logical lines read through EOF; exact pinned-tree SHA and 7369-byte size match. Example inspected, not executed.


### 2270. `crates/horizon-world/examples/probe/combat_rules_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9e523a30dcc4ed68d736d75a877fef9d131031ed`
- Size: 2505 bytes
- Coverage: Complete read through EOF: 81 logical lines; full file within one read; exact pinned-tree size 2505 bytes.
- Purpose: Armor and magic damage calculation/callback fixture
- Dependencies / cross-references: horizon-world combat_rules ArmorWorld; exact f32 bit conversion and callback mock.
- Implementation / logic: Runs after_magic and after_armor with weapon/server-level queries, callback failure and mutation; emits exact f32 bits and call counts.
- Findings: `modify_armor_effectiveness` returns its input unchanged, so no weapon/enchantment modifier is actually modeled. JSON/hex numeric values are trusted. The fixture does not model full damage sources, invulnerability frames or multiplayer combat.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Implement test cases for real modifier hooks, armor/toughness edge values, damage source rules and client/server behavior against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 81 logical lines read through EOF; exact pinned-tree SHA and 2505-byte size match. Example inspected, not executed.


### 2271. `crates/horizon-world/examples/probe/component_durability_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7c5d891118ba6b2cfae9212695bf57f400f39766`
- Size: 6019 bytes
- Coverage: Complete read through EOF: 187 logical lines; full file within one read; exact pinned-tree size 6019 bytes.
- Purpose: Component-based durability patches and break callback fixture
- Dependencies / cross-references: horizon-world component_durability, item_durability, component_patch and item template APIs.
- Implementation / logic: Builds DurabilityPrototype/ComponentDurability, restores patches, exercises hurt/break/set/grow/shrink/query and captures component snapshots, callback failure and mutation.
- Findings: The harness uses `unenchanted_change` as its enchantment processing path, so enchantment-based durability reduction is not covered. Patch/input keys, counts and i32 conversions are trusted; unrecognized patch kinds map to Unbreakable rather than being rejected by the fixture parser.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Add enchanted durability, malformed patch keys, stack-count bounds and break-copy mutation cases; compare Java 1.20.1 component patch semantics.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 187 logical lines read through EOF; exact pinned-tree SHA and 6019-byte size match. Example inspected, not executed.


### 2272. `crates/horizon-world/examples/probe/component_patch_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9bd740061ffb3199b85d6c1e8c990e461cfed84a`
- Size: 4544 bytes
- Coverage: Complete read through EOF: 136 logical lines; full file within one read; exact pinned-tree size 4544 bytes.
- Purpose: Generic component prototype/map/patch serialization and mutation fixture
- Dependencies / cross-references: horizon-world component_patch ComponentMap/Patch/Prototype and JSON/bit-preserving values.
- Implementation / logic: Exercises set/remove/copy/save/apply/restore/clear/build/from operations over integer and f32-bit values; reports map entries, keys, values, flags, patches and equality.
- Findings: Three map/patch arrays are indexed without bounds checks; arbitrary component keys are accepted while the diagnostic values/flags snapshot inspects only keys 0–15. Numeric JSON values are narrowed to i32 and malformed float bits panic.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate duplicate/out-of-range keys and large key spaces; test float NaN/negative-zero round trips and compare patch/equality semantics to Java 1.20.1 data components.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 136 logical lines read through EOF; exact pinned-tree SHA and 4544-byte size match. Example inspected, not executed.


### 2273. `crates/horizon-world/examples/probe/consumable_emission_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6bff122e17795d8f1d128c7c381896ddb7a15170`
- Size: 7543 bytes
- Coverage: Complete read through EOF: 247 logical lines; full file within one read; exact pinned-tree size 7543 bytes.
- Purpose: Consumable emission timing, item particles, sound override and random-source fixture
- Dependencies / cross-references: horizon-world consumable, item_particles, item stack refs and LegacyRandom/XoroshiroRandom.
- Implementation / logic: Models particle callbacks and world pitch/yaw/eye/level queries, random source selection, spawn/sound events, callback failure/mutation and legacy/xoroshiro RNG continuation.
- Findings: The `effects` list on Consumable is always empty, so status-effect application is not covered. Random-source and animation indexes assume valid small ranges; item/count values are narrowed to u8/i32 and callback logs use unwrap. The synthetic world is not a real level.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Test particle/no-particle variants, sound override, particles and status effects together; validate random/animation ranges and exact Java 1.20.1 event/RNG ordering.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 247 logical lines read through EOF; exact pinned-tree SHA and 7543-byte size match. Example inspected, not executed.


### 2274. `crates/horizon-world/examples/probe/consumable_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fccf46129aa440aa798c108a59e7ce765bd6458a`
- Size: 13125 bytes
- Coverage: Complete read through EOF: 413 logical lines in consecutive, gap-free ranges 1–400 and 401–413; exact pinned-tree size 13,125 bytes.
- Purpose: Food/consumable use lifecycle, effects, particles, sound, callbacks and inventory-stack identity fixture
- Dependencies / cross-references: horizon-world consumable, food, food_properties, item_particles, item_template and item_stack_refs; two deterministic RNG types.
- Implementation / logic: Implements a mock World for `ConsumptionWorld`, `FoodConsumptionWorld`, `EmissionWorld` and ItemParticleWorld; covers can-consume, start, on-consume, food properties, listeners, effect application, criterion/stat events, sound and two random streams.
- Findings: 413 lines read in contiguous ranges 1–400 and 401–413. Random-source selectors index a two-element array unchecked; animation uses a fixed 12-entry table; effect count is cast to u8 and can wrap; numerous JSON fields and callback output paths unwrap. The test world has only synthetic food/effect logic and no real inventory/network lifecycle.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate input schema and all enum/count bounds; test creative/server/client, stack shrink/transform, food exhaustion/saturation, effect failure and sound/particle order against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 413 logical lines read through EOF; exact pinned-tree SHA and 13125-byte size match. Example inspected, not executed.


### 2275. `crates/horizon-world/examples/probe/consume_effect_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `85c3b530b767755a333d475211b7a677eee77c31`
- Size: 7028 bytes
- Coverage: Complete read through EOF: 222 logical lines; full file within one read; exact pinned-tree size 7028 bytes.
- Purpose: Consume-effect application fixture for status add/remove/clear and sound effects
- Dependencies / cross-references: horizon-world consume_effect and status_effect; core BlockPos and LegacyRandom/XoroshiroRandom.
- Implementation / logic: Builds three mock effect instances (each with a hidden instance), replays ApplyStatus/RemoveStatus/ClearStatus/PlaySound operations, records add/remove/position/category/sound callbacks, failure/mutation and legacy/xoroshiro RNG states.
- Findings: Input effect indices are narrowed to u8 and directly index the three-item input vector; `returns[i % len]` panics if the return array is empty; operation/list/position arrays are trusted. Callback return/failure behavior is synthetic and no server/network synchronization is modeled.
- RustChunk relevance: Differential-test design reference for reconstructed Minecraft 1.20.1 block/world/item behavior; no direct client runtime feature.
- Decision: adaptable; reference; fixture-only; do not ship
- 1.20.1 implications: These are Starling/Horizon fixture transports, not proof of vanilla Java 1.20.1, Forge 47.x, mod negotiation, chunk protocol or Aternos compatibility. Only version-pinned differential scenarios can be adapted.
- Follow-up checks: Validate operation schemas, effect references and nonempty result sequences; compare probability/RNG draw counts, hidden-effect propagation, remove/clear ordering and sound-category behavior to Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate only the behavioral scenario and Java-oracle assertions using RustChunk-owned types; implement production behavior from version-pinned Java 1.20.1/Forge evidence, not from this mock transport.
- Evidence required before adoption: Java 1.20.1 reference outputs and version-pinned registry/NBT inputs; positive/negative/malformed corpus coverage; reproducible test execution report and documented differences. No Forge/Aternos claim without actual client/runtime integration evidence.
- Completion state: COMPLETE — all 222 logical lines read through EOF; exact pinned-tree SHA and 7028-byte size match. Example inspected, not executed.


## Checkpoint 2251–2275 — complete (25/25); section 2251–2300 remains authorized

- Entries 2251–2275 were read individually through EOF in deterministic lexical order after the full Index. Exact paths, source SHAs and byte sizes match the complete pinned recursive tree.
- The checkpoint is composed of block properties/support/survival, block update/collision/push, chunk/terrain stages, carving, climbing, combat/durability, component patches and consumable/effect fixtures.
- These files are differential-test transports with mock worlds and trusted fixture inputs, not production Minecraft implementations. Common risks include unbounded/empty arrays, unchecked narrowing/indexing, synthetic world assumptions, omitted enchantment effects, fixed registry counts and callback/geometry panics on malformed data.
- Long files `block_update_fixture.rs` (474 lines), `collision_context_fixture.rs` (450) and `consumable_fixture.rs` (413) were read in gap-free chunks no larger than 400 lines.
- Ledger contains **275 records (2001–2275)** with exactly 15 required fields. Index contains **2,275 sequential unique paths** with matching path/Decision rows.
- No tests/examples were executed; no app source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, multiplayer server or Aternos validation.
- Continue directly with **2276–2300** under the existing authorization; do not pause at 2275. Close the 50-file section at 2300 and STOP for fresh permission before 2301.

### 2276. `crates/horizon-world/examples/probe/damage_immunity_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f477c08de3a38fc3433a19f13440338dea3ae80c`
- Size: 4229 bytes
- Coverage: Complete read through EOF: 123 logical lines; exact pinned-tree size 4229 bytes; no omitted ranges.
- Purpose: Damage-immunity decisions and callback order
- Dependencies / cross-references: horizon-world damage, effect, attributes or combat APIs and fixture-owned mock worlds.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: The removed() mock always reads initial state after event mutation; slot inputs are unvalidated. Synthetic policy world, not a real entity/server damage source.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 123 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2277. `crates/horizon-world/examples/probe/damage_reduction_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7310ca43eae32f28deffa9778ecbee0a8c51d47a`
- Size: 4937 bytes
- Coverage: Complete read through EOF: 146 logical lines; exact pinned-tree size 4937 bytes; no omitted ranges.
- Purpose: Damage-reduction and armor/resistance pipeline
- Dependencies / cross-references: horizon-world damage, effect, attributes or combat APIs and fixture-owned mock worlds.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: ArmorWorld::modify_armor_effectiveness returns the input unchanged and protection always returns zero; real enchantment/armor protection is not modeled. Slot values and bit strings are unvalidated.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 146 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2278. `crates/horizon-world/examples/probe/damage_source_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8599e3cacaca4881db638b0693e5cb04247f9e6d`
- Size: 5454 bytes
- Coverage: Complete read through EOF: 164 logical lines; exact pinned-tree size 5454 bytes; no omitted ranges.
- Purpose: Damage-source catalogue, tag membership and source-entity construction
- Dependencies / cross-references: horizon-world damage, effect, attributes or combat APIs and fixture-owned mock worlds.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Entity kinds use hard-coded numeric IDs; entity/position/weapon arrays and constructor values are unvalidated. Snapshot registry data does not prove target 1.20.1 ID/message parity.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 164 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2279. `crates/horizon-world/examples/probe/decoration_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d8e53561a585a708fe886ed5598ae45f30f5f3a`
- Size: 7600 bytes
- Coverage: Complete read through EOF: 211 logical lines; exact pinned-tree size 7600 bytes; no omitted ranges.
- Purpose: Biome-aware feature decoration and random-consumption fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Palette selection indexes a 3×3 array from unvalidated coordinates and repeats synthetic biome sequences; heightmap queries always return Unavailable. No real chunk biome storage.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 211 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2280. `crates/horizon-world/examples/probe/decoration_region_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bad115a50d2133ae2eb07138e8a0e80babf02464`
- Size: 10718 bytes
- Coverage: Complete read through EOF: 303 logical lines; exact pinned-tree size 10718 bytes; no omitted ranges.
- Purpose: Resident 3×3 terrain-region decoration, retrogen, write zones, heightmaps and tick queues
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The fixture uses resident terrain/biome regions, feature placement and write callbacks, then serializes generated state, heightmap/tick or retrogen data.
- Findings: Builds resident TerrainRegion/BiomeRegion; exercises pre-light stages, retrogen, write-zone/height gates, block/fluid tags, heightmaps, scheduled ticks and mutation lifecycle. Uses snapshot DATA_VERSION; state/palette IDs and coordinates unvalidated; not async chunk acquisition.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Rebuild exact 1.20.1 registries/tags; validate coordinates, dimensions and IDs; compare retrogen, write-zone, heightmap, feature and tick outputs, then test actual async chunk scheduler.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Pinned 1.20.1 registry/tag hashes, Java worldgen/retrogen/heightmap/tick reference outputs and actual chunk loading tests.
- Completion state: COMPLETE — all 303 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2281. `crates/horizon-world/examples/probe/dynamic_support_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `84a176ea49c60885b744d33562937fd0f9c669ba`
- Size: 7211 bytes
- Coverage: Complete read through EOF: 189 logical lines; exact pinned-tree size 7211 bytes; no omitted ranges.
- Purpose: Dynamic shulker/piston support and collision shapes
- Dependencies / cross-references: dynamic_support, piston_shape, collision registry/NBT and core VoxelShape.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: State/direction indexes are unchecked; unsupported states panic at expect. Entity/piston states are synthetic JSON, not live ticking block entities.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 189 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2282. `crates/horizon-world/examples/probe/edge_backoff_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `a805cbb1ea490a4fa041eebf0368fb320347565a`
- Size: 3649 bytes
- Coverage: Complete read through EOF: 111 logical lines; exact pinned-tree size 3649 bytes; no omitted ranges.
- Purpose: Edge-backoff movement and collision probes
- Dependencies / cross-references: edge_backoff and horizon-core Aabb/Axis/Vec3.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Missing clear-array entries silently become false; AABB/vector/step inputs are trusted. Scripted collision answers are not a live block/entity collision world.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 111 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2283. `crates/horizon-world/examples/probe/effect_attribute_modifiers_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `160dcdc282c16a3d39b329429182f09d583746bd`
- Size: 11092 bytes
- Coverage: Complete read through EOF: 339 logical lines; exact pinned-tree size 11092 bytes; no omitted ranges.
- Purpose: Status-effect attribute modifier definition and application
- Dependencies / cross-references: effect_attribute_modifiers and attributes Definition/Instance/Modifier/Observer.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Models definitions/templates, instance routing, permanent modifier changes, observer callbacks and in-callback mutation. Keys/templates/routes/indexes are unvalidated; mock routing is not a live entity map.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 339 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2284. `crates/horizon-world/examples/probe/effect_notifications_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `195fa9b4b606ce0b8556cbe4e131214f7da04572`
- Size: 16827 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–506; 506 logical lines; exact pinned-tree size 16827 bytes.
- Purpose: Active-effect eligibility, lifecycle notifications and passenger update trace
- Dependencies / cross-references: active_effects, effect_notifications, status_effect, JavaKey/ObjectOpenSet and stored effect attributes.
- Implementation / logic: The fixture builds effect stores and mock JavaKey maps, then serializes lifecycle, observer, passenger/metadata or synced-value events.
- Findings: JavaKey::tie_break uses <= so equal identities compare Less. Passenger/connection rules are hard-coded; packets are strings, not wire encoding. Input maps/handles unbounded/unwrapped.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Fix JavaKey comparator to return Equal for identical tie-break identities; validate pool/routes/passenger ids; compare metadata and packet semantics with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Java 1.20.1 effect registry, stable identity/order tests, metadata packet encode/decode and client/server traces.
- Completion state: COMPLETE — all 506 logical lines read through EOF; ranges 1–400 and 401–506; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2285. `crates/horizon-world/examples/probe/effect_sync_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c8a2aa09f0b9d90beef8b351c1d7869048c922b`
- Size: 16028 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–455; 455 logical lines; exact pinned-tree size 16028 bytes.
- Purpose: Effect data-value sync, particles, ambient, glowing and invisibility
- Dependencies / cross-references: active_effects, effect_sync, effect_particles, synced_values and metadata schema.
- Implementation / logic: The fixture builds effect stores and mock JavaKey maps, then serializes lifecycle, observer, passenger/metadata or synced-value events.
- Findings: Repeats JavaKey tie_break <= defect; effect/hash/schema IDs trusted. Client/server is a boolean mock; no real metadata packet encoding or client rendering.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Fix JavaKey comparator to return Equal for identical tie-break identities; validate pool/routes/passenger ids; compare metadata and packet semantics with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Java 1.20.1 effect registry, stable identity/order tests, metadata packet encode/decode and client/server traces.
- Completion state: COMPLETE — all 455 logical lines read through EOF; ranges 1–400 and 401–455; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2286. `crates/horizon-world/examples/probe/entity_attribute_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e5fe13b988ac16cf8004ce933b5dbf383eee2cf9`
- Size: 5029 bytes
- Coverage: Complete read through EOF: 125 logical lines; exact pinned-tree size 5029 bytes; no omitted ranges.
- Purpose: Entity attribute catalogue and per-entity modifier map
- Dependencies / cross-references: attributes::entity Catalog and per-entity modifier map.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Attribute/index/operation inputs unvalidated; every operation builds a Modifier even for read-only lookups. No entity lifecycle or packet sync.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 125 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2287. `crates/horizon-world/examples/probe/entity_collision_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d801e7d344fb1202bca2987d3b36f8be3ebef74a`
- Size: 4382 bytes
- Coverage: Complete read through EOF: 128 logical lines; exact pinned-tree size 4382 bytes; no omitted ranges.
- Purpose: Entity collision candidate and unobstructed-shape fixture
- Dependencies / cross-references: entity_collision candidate/identity and core AABB geometry.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Identity indexes are 1-based vector offsets; parent/box indexes unchecked. Mock returns all candidates without query-bound prefilter and has no dynamic removal/building state.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 128 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2288. `crates/horizon-world/examples/probe/entity_fluid_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d897eea1cf92911b89ee5d186c2ee6a2fd395a98`
- Size: 16884 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–477; 477 logical lines; exact pinned-tree size 16884 bytes.
- Purpose: Entity fluid interaction, immersion, flow and fluid-tick fixture
- Dependencies / cross-references: entity_fluid/entity_fluid_tick, fluid registries/tags, block support and dynamic support.
- Implementation / logic: The fixture builds a scripted mock world and drives fluid interaction or entity movement APIs, recording queries and state changes.
- Findings: Synthetic cells/tags/states and optional registries. is_pushed_by_fluid uses values.len()-1 and panics for empty arrays; other state/tag indexes unchecked. Embedded budget assertions were not run.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Reject empty pushed arrays and invalid state/tag indexes; compare fluid tags, flow, eye immersion, budget and tick scheduling with Java.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Java 1.20.1 traces using real loaded chunks, entity state, collision shapes, fluid tags, RNG and packet/event evidence.
- Completion state: COMPLETE — all 477 logical lines read through EOF; ranges 1–400 and 401–477; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2289. `crates/horizon-world/examples/probe/entity_move_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c31e485a9b235eddb0d8640577b9feb9536ff680`
- Size: 12759 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–402; 402 logical lines; exact pinned-tree size 12759 bytes.
- Purpose: Entity movement, collision resolution, piston limiting, edge backoff, fall and history
- Dependencies / cross-references: entity_move, movement_restitution/emission/history and core collision resolution.
- Implementation / logic: The fixture builds a scripted mock world and drives fluid interaction or entity movement APIs, recording queries and state changes.
- Findings: Mock shapes and scripted world queries; passenger() always true and apply_movement_emission intentionally panics if guard missed. Inputs unvalidated; no async chunk collision or network authority.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate shapes and movement inputs; implement real chunk collision and client/server authority; compare movement flags/history/fall effects with Java.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Java 1.20.1 traces using real loaded chunks, entity state, collision shapes, fluid tags, RNG and packet/event evidence.
- Completion state: COMPLETE — all 402 logical lines read through EOF; ranges 1–400 and 401–402; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2290. `crates/horizon-world/examples/probe/entity_obstruction_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d585170bfc9f97a23aa76b1d8b9564bc5561bff`
- Size: 3971 bytes
- Coverage: Complete read through EOF: 115 logical lines; exact pinned-tree size 3971 bytes; no omitted ranges.
- Purpose: Entity obstruction and placement-clearance fixture
- Dependencies / cross-references: entity_obstruction and core Aabb/VoxelShape.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Parent/entity indexes and AABB arrays unvalidated. removed and blocks_building fields are stored but ignored by mock candidate filtering; no live world state.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 115 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2291. `crates/horizon-world/examples/probe/extra_items_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f317e318c542505e7101e05db1c696d49fa6b105`
- Size: 14629 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–446; 446 logical lines; exact pinned-tree size 14629 bytes.
- Purpose: Extra item delivery, inventory pickup, cooldown and after-use remainder fixture
- Dependencies / cross-references: extra_items, item_after_use, item_cooldowns, item_stack_refs/storage, inventory_pickup/player_inventory.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Combines ItemStacks, component patches, inventory, remainder, predicted drops and cooldowns. Inventory hard-coded to 43 slots; inputs unvalidated; component model is not 1.20.1 legacy ItemStack/NBT.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Map component remainder/cooldown data to 1.20.1 ItemStack/NBT; use correct inventory layout; validate slot/count/group bounds and event order.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 446 logical lines read through EOF; ranges 1–400 and 401–446; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2292. `crates/horizon-world/examples/probe/face_support_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `28e6fa81df5b97941e0aa43d7813fb864d1b072d`
- Size: 2995 bytes
- Coverage: Complete read through EOF: 84 logical lines; exact pinned-tree size 2995 bytes; no omitted ranges.
- Purpose: Voxel-shape face extraction, nested face identity and support test
- Dependencies / cross-references: core VoxelShape/Direction/ShapeOp and world SupportType.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Grid coordinates may be empty/inconsistent with occupied bits; cube dimensions use len()-1 and may underflow. No cap on geometry complexity.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 84 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2293. `crates/horizon-world/examples/probe/face_visibility_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3c653f1e1ad39967b6cd9fa66e096c12a9fd6c1a`
- Size: 3351 bytes
- Coverage: Complete read through EOF: 85 logical lines; exact pinned-tree size 3351 bytes; no omitted ranges.
- Purpose: Block face-visibility and occlusion skip rules
- Dependencies / cross-references: block_render FaceVisibility/RenderDefinition, CachedCollisionRegistry and block/occlusion NBT.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Loads snapshot block/occlusion registries; rules include mangrove roots and require version verification. Pair parser validates little beyond split and integer conversion.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 85 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2294. `crates/horizon-world/examples/probe/fall_damage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `573b9c1e9348f7c876c42d834a014280f1874e4b`
- Size: 4156 bytes
- Coverage: Complete read through EOF: 140 logical lines; exact pinned-tree size 4156 bytes; no omitted ranges.
- Purpose: Fall-distance, water, support and fall-on callback fixture
- Dependencies / cross-references: FallDamageWorld/check_fall_damage and core position/Vec3.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: block_state always returns ID 1 and position is fixed; no actual damage source, sound or server event. Values/positions are trusted JSON.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 140 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2295. `crates/horizon-world/examples/probe/feature_catalog_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bbc69ffd3bfcffb705bd466328e81431cc3df785`
- Size: 9260 bytes
- Coverage: Complete read through EOF: 266 logical lines; exact pinned-tree size 9260 bytes; no omitted ranges.
- Purpose: FeatureCatalog resource decode, placed-feature resolution and RNG fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Builds catalog and registry with current DATA_VERSION facts; arrays/repeats/coordinates unvalidated. Heightmap always Unavailable; mock blocks use deterministic hashes rather than resident terrain.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 266 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2296. `crates/horizon-world/examples/probe/feature_graph_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `381972f449ef1ccdd196f3bdd6ad792d2d63ea54`
- Size: 6362 bytes
- Coverage: Complete read through EOF: 180 logical lines; exact pinned-tree size 6362 bytes; no omitted ranges.
- Purpose: FeatureDefinition graph decode and placement/RNG fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Mock height unavailable and block lookups hash-derived; repeats/origin unvalidated. Does not load feature registry, tags, bindings or terrain.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 180 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2297. `crates/horizon-world/examples/probe/feature_placement_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `5310b240f6827b3ffa8c0a4401b7d50d3017a7aa`
- Size: 3554 bytes
- Coverage: Complete read through EOF: 90 logical lines; exact pinned-tree size 3554 bytes; no omitted ranges.
- Purpose: Placement modifier chains and callback RNG-sharing fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Uses synthetic query adapter/result modes and supplied positions; no real terrain or registry. Input modifiers/context are trusted.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 90 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2298. `crates/horizon-world/examples/probe/feature_region_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6a244998d0549f79b3993b67172a1a87c865aa5b`
- Size: 21088 bytes
- Coverage: Complete read through EOF in consecutive, gap-free chunks 1–400 and 401–590; 590 logical lines; exact pinned-tree size 21088 bytes.
- Purpose: Resident terrain-region feature generation, retrogen, heightmaps, write zones and ticks
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The fixture uses resident terrain/biome regions, feature placement and write callbacks, then serializes generated state, heightmap/tick or retrogen data.
- Findings: Builds 3×3 TerrainRegion/BiomeRegion; exercises features, pre-light stage, retrogen, write-zone/height gates, fluid/block tags, heightmaps and tick queues. DATA_VERSION/current snapshot; IDs/arrays unchecked; no async chunk acquisition.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Rebuild exact 1.20.1 registries/tags; validate coordinates, dimensions and IDs; compare retrogen, write-zone, heightmap, feature and tick outputs, then test actual async chunk scheduler.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Pinned 1.20.1 registry/tag hashes, Java worldgen/retrogen/heightmap/tick reference outputs and actual chunk loading tests.
- Completion state: COMPLETE — all 590 logical lines read through EOF; ranges 1–400 and 401–590; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2299. `crates/horizon-world/examples/probe/feature_sort_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `370a1632559dd3e710ba00fdf58dc6d9f214acd5`
- Size: 1944 bytes
- Coverage: Complete read through EOF: 50 logical lines; exact pinned-tree size 1944 bytes; no omitted ranges.
- Purpose: Biome feature dependency graph sorting and cycle fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Converts feature lists and reports topological steps/cycles. IDs cast to u32; fixture does not validate registry resolution or placed-feature dependency data.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 50 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


### 2300. `crates/horizon-world/examples/probe/fill_layer_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e107a63832a760cf4cd543cf26e389fdbfb96c1f`
- Size: 6331 bytes
- Coverage: Complete read through EOF: 185 logical lines; exact pinned-tree size 6331 bytes; no omitted ranges.
- Purpose: FillLayerFeature decode and world-minimum/block replacement fixture
- Dependencies / cross-references: horizon-world worldgen decoration/feature/region/registry/tag/placement APIs and horizon-core random/terrain primitives.
- Implementation / logic: The program loads fixture data, constructs the corresponding Horizon-world model, replays scripted inputs and writes deterministic trace/output files.
- Findings: Loads current DATA_VERSION block registry; mock world hashes positions to seven states, keeps writes in memory and returns synthetic height failures. Repeats/origin/world_min unvalidated.
- RustChunk relevance: Differential test transport only; useful for generating version-pinned scenarios and expected query/event/state order, not a production client subsystem.
- Decision: adaptable; reference
- 1.20.1 implications: No direct proof of Minecraft Java 1.20.1 / Forge 47.x parity. These fixtures use Horizon snapshot APIs/data and must be compared with the target-version Java implementation.
- Follow-up checks: Validate fixture arrays, numeric ranges and registry versions; add edge/failure cases and compare outputs with Java 1.20.1.
- RustChunk implementation mapping: Keep only as a standalone RustChunk differential-test tool. Convert version-confirmed cases into tests; do not include the fixture or mock world in the Android runtime graph.
- Evidence required before adoption: Version-pinned Java 1.20.1 oracle outputs, validated schema, failure cases and explicit executed test results; no Aternos claim without live validation.
- Completion state: COMPLETE — all 185 logical lines read through EOF; single complete read; exact pinned-tree SHA and byte size match. Tests/examples inspected only, not executed.


## Checkpoint 2276–2300 — complete (25/25); section 2251–2300 closed

- Entries 2276–2300 were inspected one at a time through EOF. Long files 2284, 2285, 2288, 2289, 2291 and 2298 were read in consecutive, gap-free ranges of 1–400 and 401–EOF. All paths, source SHAs and byte sizes match the pinned tree.
- This group covers damage immunity/reduction/source attribution, decoration/region generation, dynamic support, edge-backoff, effect attribute modifiers/notifications/sync, entity attributes/collision/fluid/movement/obstruction, extra-item inventory/cooldown, face support/visibility, fall damage and feature catalog/graph/placement/region/sort/fill-layer.
- The effect notification/sync fixtures repeat the JavaKey `<=` comparator defect. Several mock worlds use fixed/synthetic registry/entity/chunk behavior. `entity_fluid_fixture` can panic on empty `pushed` data; component/inventory/shape and movement inputs are largely unvalidated. Feature-region fixtures use current DATA_VERSION and resident mock chunks, not target-version data or async chunk loading.
- Ledger now contains **300 records (2001–2300)** with exactly 15 fields each. Index now has **2,300 sequential unique paths** with matching Decisions.
- Tests/examples were inspected only; none executed. No app source changed; no build/benchmark/APK/device/renderer/Minecraft/Forge/server/Aternos validation.
- Reporting section **2251–2300 is CLOSED (50/50)**. The one-time authorization is exhausted. **STOP at 2300** and require fresh explicit permission before entry 2301.


## Decision taxonomy normalization — entries 2276–2300 — 2026-11-21

- The initial Ledger/Index rows used reference as a standalone Decision. Protocol requires a primary decision; reference is a qualifier only.
- All 25 rows are normalized to adaptable; reference. Supporting Findings and RustChunk relevance retain the fixture-only/no-production-runtime limit.
- Entry IDs, source paths, SHAs, sizes and technical findings are unchanged.

### 2301. `crates/horizon-world/examples/probe/fluid_box_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0b348d7bbc58187bbf4197e846129494df542e41`
- Size: 3871 bytes
- Coverage: Complete read through EOF: 122 logical lines; full file read without omitted ranges; exact pinned-tree size 3871 bytes.
- Purpose: Fluid interaction boxes and vehicle/boat passenger bounds
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Builds actor/boat AABBs and simulates no vehicle, boat and custom passenger-box modifiers; traces query order, injected failures and in-callback world mutation.
- Findings: Vehicle ID narrows u64→u8; unknown vehicle values panic. AABB coordinates and callback fields are trusted JSON; numeric hex parsing unwraps. Only three vehicle cases are modeled, not a full vehicle registry.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate AABBs and vehicle IDs; compare boat passenger/interaction-box edge cases and query/mutation order with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 122 logical lines read through EOF; exact pinned-tree SHA and 3871-byte size match. Example inspected, not executed.


### 2302. `crates/horizon-world/examples/probe/fluid_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1c5ed4a8524274a5439bf925e4090514ffcdb440`
- Size: 4240 bytes
- Coverage: Complete read through EOF: 128 logical lines; full file read without omitted ranges; exact pinned-tree size 4240 bytes.
- Purpose: Fluid-state families, block-state bindings, codec and fluid-height fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Builds a synthetic BlockRegistry from facts NBT; emits FluidKind families, registry-state FluidState/bindings, codec round trips, and height_at/camera_height_at queries with injected failures.
- Findings: State IDs narrow to u32 and positions i64→i32; registry/above-state lookups unwrap. Correctness depends on the private facts-server registry snapshot; no target 1.20.1 fluid registry parity is established.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Pin vanilla 1.20.1 registries; test invalid IDs, source/falling/level codecs, height thresholds and exact query order against Java.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 128 logical lines read through EOF; exact pinned-tree SHA and 4240-byte size match. Example inspected, not executed.


### 2303. `crates/horizon-world/examples/probe/fluid_flow_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec78105099e46476292a1c581fd46465c448789b`
- Size: 3682 bytes
- Coverage: Complete read through EOF: 113 logical lines; full file read without omitted ranges; exact pinned-tree size 3682 bytes.
- Purpose: Fluid flow vectors and solid-face differential fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Mock FluidFlowWorld traces fluid/block/sturdy-face queries, derives flow vectors or solid-face results, injects failures and records Legacy/Xoroshiro state.
- Findings: Cell/state/direction selectors index fixture arrays directly; coordinates narrow to i32. Missing map cells use a trusted default. The mock is not a loaded-chunk/block-tag simulation.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate selectors and empty cells; compare flow vectors, face checks, query order and RNG consumption with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 113 logical lines read through EOF; exact pinned-tree SHA and 3682-byte size match. Example inspected, not executed.


### 2304. `crates/horizon-world/examples/probe/fluid_occupancy_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7fc884f4955091a0eee3c2ab1034705a6e0bfbd3`
- Size: 5978 bytes
- Coverage: Complete read through EOF: 177 logical lines; full file read without omitted ranges; exact pinned-tree size 5978 bytes.
- Purpose: Fluid-presence registry and bounded AABB occupancy scan fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Builds FluidPresence; checks invalid IDs, registry identity, non-finite bounds and query budgets; simulates chunk/section access, block writes and liquid occupancy scans.
- Findings: Fixture dimensions, sections, height and coordinates are unvalidated before indexing/arithmetic; min+height−1 and section conversions can overflow or index out of range. Single synthetic chunk model.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Reject invalid dimensions before allocation; fuzz section bounds and raw IDs; compare negative heights, missing sections and liquid scans with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 177 logical lines read through EOF; exact pinned-tree SHA and 5978-byte size match. Example inspected, not executed.


### 2305. `crates/horizon-world/examples/probe/fluid_shape_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9c14d584963485c90ad94043e7767b1dd291c823`
- Size: 3564 bytes
- Coverage: Complete read through EOF: 97 logical lines; full file read without omitted ranges; exact pinned-tree size 3564 bytes.
- Purpose: Cached fluid voxel-shape identity and height-query fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Exercises FluidShapeCache across state/position operations, records empty/block identity and boxes, and injects fluid-height callback failures.
- Findings: Position/state JSON and operations are trusted and unwrap-heavy. sturdy_down intentionally panics to assert this shape path does not request camera-immersion support; this is a narrow contract fixture.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate state/position inputs; confirm sturdy_down is not required and compare cached shape identity, box geometry and failure behavior with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 97 logical lines read through EOF; exact pinned-tree SHA and 3564-byte size match. Example inspected, not executed.


### 2306. `crates/horizon-world/examples/probe/food_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c12106dfb05d846591ad6719f98a763dcc5e8045`
- Size: 4779 bytes
- Coverage: Complete read through EOF: 139 logical lines; full file read without omitted ranges; exact pinned-tree size 4779 bytes.
- Purpose: FoodData arithmetic, exhaustion and regeneration tick fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Applies eat/eat-properties/exhaustion/direct food-saturation changes and tick; records health, difficulty, regeneration, starvation and callback order with mutation/failure injection.
- Findings: Difficulty indexes a four-item array unchecked; food/timer/nutrition narrow to i32 and bit strings/operation names are trusted. Synthetic world, not a live player or authoritative server.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test overflow/negative food values, all difficulties, regeneration/starvation thresholds and callback failures against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 139 logical lines read through EOF; exact pinned-tree SHA and 4779-byte size match. Example inspected, not executed.


### 2307. `crates/horizon-world/examples/probe/food_properties_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `50b88bcccf9db1bb43d07e6e4f21a8ed23b17cec`
- Size: 5223 bytes
- Coverage: Complete read through EOF: 176 logical lines; full file read without omitted ranges; exact pinned-tree size 5223 bytes.
- Purpose: FoodProperties consumption, sound and random-source fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Models two FoodData slots, two random sources, player/position queries, callback mutation/failure and sound output while running on_consume with Legacy/Xoroshiro.
- Findings: Random source and food index address fixed two-element arrays directly; values narrow to i32/u32 and unwrap. Mock callbacks do not model inventory/network lifecycle.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate selectors/counts; compare creative/survival rules, sound category/position and RNG order with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 176 logical lines read through EOF; exact pinned-tree SHA and 5223-byte size match. Example inspected, not executed.


### 2308. `crates/horizon-world/examples/probe/food_storage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `13e9a52dc669ebf4d55906fd93caa59b377df041`
- Size: 3604 bytes
- Coverage: Complete read through EOF: 108 logical lines; full file read without omitted ranges; exact pinned-tree size 3604 bytes.
- Purpose: FoodData NBT persistence and round-trip fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Reads/writes four food keys, tries multiple tag types, preserves an unrelated field and reports serialized values/round-trip.
- Findings: Tag variants 7–12 are replaced with fixed synthetic values rather than arbitrary payloads; integer narrowing/parsing unwrap. Not a general NBT codec test.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Compare key types/defaults with Java 1.20.1; add real byte/int/float/double payloads and malformed/missing-key cases.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 108 logical lines read through EOF; exact pinned-tree SHA and 3604-byte size match. Example inspected, not executed.


### 2309. `crates/horizon-world/examples/probe/ground_state_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `47611587fd32772d3ea89e6b17e98332fd9c04e1`
- Size: 4034 bytes
- Coverage: Complete read through EOF: 111 logical lines; full file read without omitted ranges; exact pinned-tree size 4034 bytes.
- Purpose: Ground-support state and movement callback fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Runs check_support/set_with_movement for boxes and movement vectors; injects support-query failure and traces on-ground, horizontal-collision and support-block results.
- Findings: Coordinates narrow i64→i32; results are indexed by callback count and mode values are trusted. Support queries are scripted closures, not live chunk collision.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate result-array lengths/modes; compare edge contact, negative coordinates and partial callback failures with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 111 logical lines read through EOF; exact pinned-tree SHA and 4034-byte size match. Example inspected, not executed.


### 2310. `crates/horizon-world/examples/probe/height_provider_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `94b2524f7d8cbb5d36290bf453cedb5a32a9f52e`
- Size: 1860 bytes
- Coverage: Complete read through EOF: 50 logical lines; full file read without omitted ranges; exact pinned-tree size 1860 bytes.
- Purpose: Worldgen height-provider decode, sample and RNG fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Decodes HeightProvider definitions, samples a WorldGenerationContext and records next/tail random values with LegacyRandom or XoroshiroRandom.
- Findings: Context values narrow to i32 and sample count is unbounded input; JSON fields unwrap. Synthetic definition list does not prove 1.20.1 provider registry coverage.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Use version-pinned 1.20.1 definitions; test invalid codecs, min/max distributions and exact RNG draw counts against Java.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 50 logical lines read through EOF; exact pinned-tree SHA and 1860-byte size match. Example inspected, not executed.


### 2311. `crates/horizon-world/examples/probe/heightmap_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `14cebbc597f139902b1f930a531d92a1a9ec8881`
- Size: 8577 bytes
- Coverage: Complete read through EOF: 253 logical lines; full file read without omitted ranges; exact pinned-tree size 8577 bytes.
- Purpose: Heightmap predicates, updates, priming, raw data and query fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Builds block-motion tag definitions; creates a mock chunk with per-section non-air counts; exercises HeightmapTypes, set/update/prime/raw/query/dump and traces reads.
- Findings: height*256 allocation, min+height bounds and section calculations are unchecked; invalid/negative dimensions can overflow or allocate incorrectly. State IDs, raw words, masks and op indexes are trusted. One synthetic chunk only.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate dimensions before allocation; fuzz raw word lengths/IDs; compare all heightmaps, negative heights, priming and updates with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 253 logical lines read through EOF; exact pinned-tree SHA and 8577-byte size match. Example inspected, not executed.


### 2312. `crates/horizon-world/examples/probe/interaction_shape_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c1ca5700b886f76ac7bf129043dd276c774a0481`
- Size: 5924 bytes
- Coverage: Complete read through EOF: 170 logical lines; full file read without omitted ranges; exact pinned-tree size 5924 bytes.
- Purpose: Interaction-shape registry, canonical identity and ray-clip fixture
- Dependencies / cross-references: Uses horizon-world fluid/food/ground/heightmap/interaction-shape modules plus horizon-core geometry, positions, random and NBT; consumes private fixture JSON/NBT registry facts.
- Implementation / logic: Loads block registry and interaction-shape NBT, tests malformed kinds/cache/canonical/state order, dumps shape boxes and clips rays against supplied interaction shapes.
- Findings: Other RayWorld methods deliberately panic because the fixture calls direct interaction clipping. Positions/state IDs/geometry arrays are trusted; not general block/fluid ray tracing.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate ray endpoints/state IDs and malformed NBT; compare hit location/direction/inside and shape priority with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 170 logical lines read through EOF; exact pinned-tree SHA and 5924-byte size match. Example inspected, not executed.


### 2313. `crates/horizon-world/examples/probe/inventory_pickup_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `58170e8719e8d383af964e14557495354269d770`
- Size: 15096 bytes
- Coverage: Complete read through EOF: 410 logical lines in contiguous, gap-free ranges 1–400 and 401–410; exact pinned-tree size 15,096 bytes.
- Purpose: Player inventory pickup, stack merge, selection, equipment and callback mutation fixture
- Dependencies / cross-references: Uses horizon-world player_inventory, item_stack_refs, inventory_tick/use/start, synced_values, equipment and component_patch modules; models player state through mock callbacks.
- Implementation / logic: Models StackRef identity, two PlayerInventory instances, equipment, add/space/remove/pick/replace/copy/component operations and max-stack/infinite-material callbacks.
- Findings: 410 lines read in ranges 1–400 and 401–410. Snapshots assume 43 slots; item/prototype/ref/equipment indexes are trusted and many unwraps can panic on malformed input. Horizon component stacks are not 1.20.1 legacy ItemStack/NBT.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate schema and indexes; compare merge/split, slot limits, equipment aliases and callback mutation with Java 1.20.1; map Forge capabilities separately.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 410 logical lines read through EOF in ranges 1–400 and 401–410; exact pinned-tree SHA and 15096-byte size match. Example inspected, not executed.


### 2314. `crates/horizon-world/examples/probe/inventory_tick_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1dfb9655eb1e3cbfc6d2803dde4d676d894fc93b`
- Size: 7590 bytes
- Coverage: Complete read through EOF: 231 logical lines; full file read without omitted ranges; exact pinned-tree size 7590 bytes.
- Purpose: Inventory and equipment per-tick callback ordering fixture
- Dependencies / cross-references: Uses horizon-world player_inventory, item_stack_refs, inventory_tick/use/start, synced_values, equipment and component_patch modules; models player state through mock callbacks.
- Implementation / logic: Builds a 43-slot inventory/equipment map; runs inventory/equipment/both/stack tick paths while recording stack identity, mutation, level and failures.
- Findings: Fixed slot/equipment indexes are unchecked; levels narrow to u8 and item_tick asserts server level 1. Synthetic refs do not model a full player tick, held-slot sync or packets.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Compare tick iteration order, client/server gates, equipment slot treatment and callback mutation with Java 1.20.1; verify packet mapping separately.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 231 logical lines read through EOF; exact pinned-tree SHA and 7590-byte size match. Example inspected, not executed.


### 2315. `crates/horizon-world/examples/probe/inventory_use_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f00d97c9fe99fffefa5d8740fe6299380b4b2097`
- Size: 10534 bytes
- Coverage: Complete read through EOF: 331 logical lines; full file read without omitted ranges; exact pinned-tree size 10534 bytes.
- Purpose: Active item-use tick/update/complete/release/stop fixture
- Dependencies / cross-references: Uses horizon-world player_inventory, item_stack_refs, inventory_tick/use/start, synced_values, equipment and component_patch modules; models player state through mock callbacks.
- Implementation / logic: Models ActiveStack/UseState, inventory and synced flags; drives callbacks for level/client state, remaining ticks, item ticks, release/finish, vibration, equipment and sync.
- Findings: Uses three mock items, fixed inventory and one synced flag ID 8; callback arrays/item indexes are trusted. Not the full server/client interaction or packet state machine.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test invalid flags/item/level values; compare hand selection, release/finish timing, stop semantics, mutation and dirty-flag packing with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 331 logical lines read through EOF; exact pinned-tree SHA and 10534-byte size match. Example inspected, not executed.


### 2316. `crates/horizon-world/examples/probe/inventory_use_start_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `fc46a58b7dcbe07e5d3d077164c702d2f2e07467`
- Size: 14004 bytes
- Coverage: Complete read through EOF: 436 logical lines in contiguous, gap-free ranges 1–400 and 401–436; exact pinned-tree size 14,004 bytes.
- Purpose: Item-use start, duration, equipment-sync and kinetic/vibration fixture
- Dependencies / cross-references: Uses horizon-world player_inventory, item_stack_refs, inventory_tick/use/start, synced_values, equipment and component_patch modules; models player state through mock callbacks.
- Implementation / logic: Implements UseServices/DurationServices/StartServices; exercises start, notify, flag/assign and lifecycle operations with callbacks, equipment sync and kinetic state.
- Findings: 436 lines read in ranges 1–400 and 401–436. Three item refs, fixed 43-slot inventory and one synced data ID 8; arrays/conversions unchecked. Does not encode actual 1.20.1 packets.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate flag masks/item IDs/duration/inventory bounds; compare use-start hand/equipment sync, kinetic/vibration hooks and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 436 logical lines read through EOF in ranges 1–400 and 401–436; exact pinned-tree SHA and 14004-byte size match. Example inspected, not executed.


### 2317. `crates/horizon-world/examples/probe/item_after_use_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `30c35b87149ac52910fbe8f73cca1588ef0128d7`
- Size: 10772 bytes
- Coverage: Complete read through EOF: 329 logical lines; full file read without omitted ranges; exact pinned-tree size 10772 bytes.
- Purpose: After-use remainder, cooldown and stack replacement fixture
- Dependencies / cross-references: Uses horizon-world item_after_use, cooldowns, blocking_rules, durability, item_template, item_stack_refs/storage and callback observer interfaces.
- Implementation / logic: Models item templates/remainders, stack components, cooldown observer events, refs, finish-item and extra-item callbacks, and mutation/failure.
- Findings: Observer::ended is deliberately unreachable in the fixture; source/finish/prototype indexes and counts are trusted. Only a small three-ref item set exists; no full inventory remainder placement.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test started/ended observer paths, creative/infinite-materials, remainder overflow and cooldown group behavior against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 329 logical lines read through EOF; exact pinned-tree SHA and 10772-byte size match. Example inspected, not executed.


### 2318. `crates/horizon-world/examples/probe/item_blocking_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e3f622a6d4c78f9a0b8b7be15a7493c82c9d0080`
- Size: 7209 bytes
- Coverage: Complete read through EOF: 211 logical lines; full file read without omitted ranges; exact pinned-tree size 7209 bytes.
- Purpose: Blocking item, projectile bypass and damage-reduction callback fixture
- Dependencies / cross-references: Uses horizon-world item_after_use, cooldowns, blocking_rules, durability, item_template, item_stack_refs/storage and callback observer interfaces.
- Implementation / logic: Models component bypass sets, projectile/entity queries, angle/type matching, reductions and player durability/stat callbacks with failure/mutation.
- Findings: TypeMatchWorld supports only test set IDs 1–4; other IDs panic. Entity/projectile kinds are hard-coded; pierce narrows to i8 and component indexes are trusted. Not a full damage/enchantment pipeline.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test unknown sets and angle/type boundaries; compare projectile bypass, piercing, blocking and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 211 logical lines read through EOF; exact pinned-tree SHA and 7209-byte size match. Example inspected, not executed.


### 2319. `crates/horizon-world/examples/probe/item_cooldowns_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e43f1c6bb966a97cbb775caa1c9eed8c97fbde3f`
- Size: 8105 bytes
- Coverage: Complete read through EOF: 238 logical lines; full file read without omitted ranges; exact pinned-tree size 8105 bytes.
- Purpose: Item cooldown grouping, tick expiry and observer-mutation fixture
- Dependencies / cross-references: Uses horizon-world item_after_use, cooldowns, blocking_rules, durability, item_template, item_stack_refs/storage and callback observer interfaces.
- Implementation / logic: Runs add/remove/tick/clock/percent/duration/apply operations using Identifier groups and callbacks that can mutate book or stack.
- Findings: Default group lookup indexes a three-element array by item ID; group/mutation indexes are trusted. Recursive mutation is synthetic; no player cooldown packets are generated.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test fallback group, zero/negative/large durations and mutation during callbacks against Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 238 logical lines read through EOF; exact pinned-tree SHA and 8105-byte size match. Example inspected, not executed.


### 2320. `crates/horizon-world/examples/probe/item_durability_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de0af24c30fabc8c4bbbc6def54fcb1b1b0762f4`
- Size: 4148 bytes
- Coverage: Complete read through EOF: 132 logical lines; full file read without omitted ranges; exact pinned-tree size 4148 bytes.
- Purpose: Item durability damage/break and count/repair fixture
- Dependencies / cross-references: Uses horizon-world item_after_use, cooldowns, blocking_rules, durability, item_template, item_stack_refs/storage and callback observer interfaces.
- Implementation / logic: Exercises hurt/break, hurt-without-breaking, set_damage, grow/shrink and snapshots with infinite-materials, criterion and break callbacks.
- Findings: process_enchantments explicitly uses unenchanted_change, so enchantment durability modifiers are not covered. Numeric values narrow to i32; callback JSON is trusted.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Add enchantment-aware damage and break-copy/infinite-material cases; compare negative/over-max damage, counts and criteria order with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 132 logical lines read through EOF; exact pinned-tree SHA and 4148-byte size match. Example inspected, not executed.


### 2321. `crates/horizon-world/examples/probe/item_particles_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b7375e1755a4f9a8343ac8fb41121aeb39a006a`
- Size: 5563 bytes
- Coverage: Complete read through EOF: 188 logical lines; full file read without omitted ranges; exact pinned-tree size 5563 bytes.
- Purpose: Item particle emission, template, position and RNG fixture
- Dependencies / cross-references: Uses horizon-world item_particles, item_stack_storage, item_template and item_use_catalog/lifecycle; uses core deterministic random sources and component patch data.
- Implementation / logic: Calls item_particles::spawn with item templates, position/orientation/eye/level callbacks, particle output and Legacy/Xoroshiro random-state tracing.
- Findings: Random source indexes a fixed two-element array; particle count is input-controlled without a local cap; item/bit values narrow or unwrap. No packet delivery/rendering is exercised.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate count/random selectors; compare template selection, velocity/orientation and RNG draw sequence with Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 188 logical lines read through EOF; exact pinned-tree SHA and 5563-byte size match. Example inspected, not executed.


### 2322. `crates/horizon-world/examples/probe/item_stack_storage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `31c741b3e307245910bcabb8ff55fef15d535001`
- Size: 7966 bytes
- Coverage: Complete read through EOF: 208 logical lines; full file read without omitted ranges; exact pinned-tree size 7966 bytes.
- Purpose: ItemStackData patches, copy/split/consume and reference semantics fixture
- Dependencies / cross-references: Uses horizon-world item_particles, item_stack_storage, item_template and item_use_catalog/lifecycle; uses core deterministic random sources and component patch data.
- Implementation / logic: Exercises copy/count/clear/split/transmute/component patch/consume/grow/shrink/limit/equality operations over a small stack arena.
- Findings: Fixed three slots; prototype/operation/component indexes and counts are trusted and narrowed. Component-stack model is not proof of 1.20.1 legacy ItemStack NBT or Forge capability behavior.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test empty/large counts and malformed patches; compare split/consume/copy identity and component semantics to Java 1.20.1; map Forge stack/capability rules separately.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 208 logical lines read through EOF; exact pinned-tree SHA and 7966-byte size match. Example inspected, not executed.


### 2323. `crates/horizon-world/examples/probe/item_template_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b6a538b0417f120f97bb3ddd1b789610a2607721`
- Size: 8335 bytes
- Coverage: Complete read through EOF: 237 logical lines; full file read without omitted ranges; exact pinned-tree size 8335 bytes.
- Purpose: ItemTemplate creation, extraction, patch and strict-validation fixture
- Dependencies / cross-references: Uses horizon-world item_particles, item_stack_storage, item_template and item_use_catalog/lifecycle; uses core deterministic random sources and component patch data.
- Implementation / logic: Runs new/from-stack/with-count/create/apply/apply-count/validated-patch and prototype mutation over stack/component maps.
- Findings: Three template slots; prototype/template/stack indexes trusted; keys/counts narrow to u8/i32 and patch fields unwrap. Mock contents validator always succeeds and max-damage/stack rules are simplified.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test invalid IDs, oversized counts and realistic contents; compare patch merge, strict validation and stack creation to Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 237 logical lines read through EOF; exact pinned-tree SHA and 8335-byte size match. Example inspected, not executed.


### 2324. `crates/horizon-world/examples/probe/item_use_catalog_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7d7871720aaff2b1a5b92be630ed139fd23512ee`
- Size: 2714 bytes
- Coverage: Complete read through EOF: 73 logical lines; full file read without omitted ranges; exact pinned-tree size 2714 bytes.
- Purpose: Item-use catalog definition and durability/use-property mutation fixture
- Dependencies / cross-references: Uses horizon-world item_particles, item_stack_storage, item_template and item_use_catalog/lifecycle; uses core deterministic random sources and component patch data.
- Implementation / logic: Loads ItemUseCatalog JSON, resolves a named item, snapshots stack/durability/use properties and applies property mutations.
- Findings: Unknown catalog names unwrap and panic; float hex/integer ranges are not validated. Definitions belong to this snapshot and are not a 1.20.1 registry or packet contract.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Test missing IDs, invalid numeric ranges and defaults; compare use duration/release/max-stack/durability metadata to Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 73 logical lines read through EOF; exact pinned-tree SHA and 2714-byte size match. Example inspected, not executed.


### 2325. `crates/horizon-world/examples/probe/item_use_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `8d88aae7acde4c96a0a4129d4714d7263bc75cd9`
- Size: 4453 bytes
- Coverage: Complete read through EOF: 139 logical lines; full file read without omitted ranges; exact pinned-tree size 4453 bytes.
- Purpose: Basic item-use eligibility, blocking delay and remaining-use duration fixture
- Dependencies / cross-references: Uses horizon-world item_particles, item_stack_storage, item_template and item_use_catalog/lifecycle; uses core deterministic random sources and component patch data.
- Implementation / logic: Builds synthetic item blocking components; runs blocking_item, is_blocking, ticks_using_item and partial ticks with callback failure/mutation.
- Findings: Two-item component/delay array is fixed; selectors/durations are trusted. Uses Horizon BlockingRules and does not implement full 1.20.1 hand-use animation, release packets or Forge events.
- RustChunk relevance: Useful as a narrow differential-test scenario for reconstructing Minecraft Java 1.20.1 world/item behavior; synthetic Starling fixture, not production client code.
- Decision: adaptable; reference
- 1.20.1 implications: No fixture establishes target 1.20.1 registry, NBT, packet, inventory, hand-use, item-component, Forge 47.x mod compatibility or Aternos connectivity. Versioned Java-oracle outputs are required before adopting behavior.
- Follow-up checks: Validate item selectors and fractional duration boundaries; compare use eligibility, blocking delay, partial ticks and client/server behavior to Java 1.20.1.
- RustChunk implementation mapping: Keep under RustChunk developer tests/tools, outside the Android release dependency graph. Recreate the scenario using RustChunk-owned 1.20.1 types and Java oracle outputs; do not copy mock adapters or later-version registry assumptions into production.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/ItemStack/NBT data; valid and malformed fixture corpus; executed differential tests and Android/runtime integration evidence. Forge/Aternos compatibility requires separate live integration tests.
- Completion state: COMPLETE — all 139 logical lines read through EOF; exact pinned-tree SHA and 4453-byte size match. Example inspected, not executed.


## Checkpoint 2301–2325 — COMPLETE (25/25)

- Entries 2301–2325 were inspected one at a time through EOF in deterministic lexical path order after the full Index. Exact source paths, SHAs and byte sizes match the pinned, untruncated recursive tree.
- This batch covers fluid state/flow/occupancy/shape, food and persistence, ground support, height providers/maps, interaction shapes, player inventory/ticking/use, cooldowns, durability, particles, stack storage/templates and item-use fixtures.
- These are synthetic differential-test transports, not production game systems. Common risks are trusted JSON/NBT inputs, unchecked array/registry indexes, numeric narrowing, unbounded sample/particle counts and fixed mock inventory/item sets. `item_durability_fixture.rs` explicitly uses `unenchanted_change`, so enchantment durability changes are not covered.
- Long files: `inventory_pickup_fixture.rs` 1–400 and 401–410; `inventory_use_start_fixture.rs` 1–400 and 401–436.
- Active Ledger: **125 records (2201–2325)** with exactly 15 fields per entry. Index: **2,325 sequential unique paths** with matching path/Decision rows.
- No examples/tests were executed; no RustChunk app source changed; no build, benchmark, APK, Android device, renderer, Minecraft, Forge, multiplayer server or Aternos validation.
- One-time user authorization covers **2301–2400**. Continue through 2326–2350 without pausing at 2325; proceed through 2351–2375 and 2376–2400; **STOP at 2400**.

### 2326. `crates/horizon-world/examples/probe/item_use_lifecycle_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `f7d3df48a8f762caddacfc2954e739d0974cc85d`
- Size: 7588 bytes
- Coverage: Complete read through EOF: 234 logical lines; exact pinned-tree size 7588 bytes; no omitted ranges.
- Purpose: Active item-use lifecycle and hand-stack transitions
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Exercises inventory-backed active item use, hands, duration/tick completion, release/stop and remainder handling with mock item references.
- Findings: Small fixed item/hand catalogue; JSON indexes, modes and references are trusted, narrowed or indexed directly. Not a live client/server hand-use packet state machine.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Add malformed hand/item indexes, creative/server/client and release/finish edge cases; compare state and packets with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 234 logical lines read through EOF; exact pinned-tree SHA and 7588-byte size match. Example inspected, not executed.


### 2327. `crates/horizon-world/examples/probe/item_use_start_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `be927e42efb99a295d18f28ae88d9f70d9ae6c5b`
- Size: 8049 bytes
- Coverage: Complete read through EOF: 247 logical lines; exact pinned-tree size 8049 bytes; no omitted ranges.
- Purpose: Item-use start and duration-service fixture
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Covers start-use decisions, duration queries, hand/equipment synchronization and callback traces using mock item/player services.
- Findings: Selectors and durations are trusted or narrowed; fixed mock references do not establish real inventory sync, packet contents or Forge event ordering.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate item/hand/duration ranges; compare hand choice, equipment sync and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 247 logical lines read through EOF; exact pinned-tree SHA and 8049-byte size match. Example inspected, not executed.


### 2328. `crates/horizon-world/examples/probe/java_hash_map_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `de441a488d50c23c6a82b5c5c9a95bf4194e3874`
- Size: 3636 bytes
- Coverage: Complete read through EOF: 111 logical lines; exact pinned-tree size 3636 bytes; no omitted ranges.
- Purpose: Java-compatible hash-map and cursor fixture
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Exercises put/remove/contains/get/clear, cursor traversal, next/delete, entry snapshots and errors with synthetic keys and configurable hashes.
- Findings: IDs/hash narrow to i32/usize; key indices and operation fields are unvalidated. Container contract only, not production gameplay or cross-thread safety proof.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Test hash collisions, equal IDs, replacement order, cursor invalidation and callback mutation against Java map behavior.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 111 logical lines read through EOF; exact pinned-tree SHA and 3636-byte size match. Example inspected, not executed.


### 2329. `crates/horizon-world/examples/probe/java_identity_map_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `700b5b32939d571398508178c2ccfdad969b2d93`
- Size: 5043 bytes
- Coverage: Complete read through EOF: 147 logical lines; exact pinned-tree size 5043 bytes; no omitted ranges.
- Purpose: Identity-keyed Java map and cursor fixture
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Models identity-map insertion, lookup, replacement, cursor traversal and removal using synthetic keys.
- Findings: `tie_break` returns Less for `identity <= other.identity`; equal identities compare Less both ways, invalid if distinct keys share an identity. Input indexes/conversions are unchecked.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Correct equal-identity comparator behavior; add duplicate/equal identity, hash collision and cursor mutation tests before using as an oracle.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 147 logical lines read through EOF; exact pinned-tree SHA and 5043-byte size match. Example inspected, not executed.


### 2330. `crates/horizon-world/examples/probe/level_place_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0b9b4f88342465a686a53857efb99be5f42c5d13`
- Size: 3850 bytes
- Coverage: Complete read through EOF: 112 logical lines; exact pinned-tree size 3850 bytes; no omitted ranges.
- Purpose: Block placement-state and unobstructed-neighbour fixture
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Calls placement-state logic with a synthetic 5×5 neighbourhood, fixed loaded-chunk answers, deterministic random and block lookup traces.
- Findings: `is_loaded` always true and `set_raw` always None; neighbour/face/state indices and coordinates are trusted. No real chunk availability or server placement packets.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate direction/state/position inputs; add unloaded-neighbour and border cases against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 112 logical lines read through EOF; exact pinned-tree SHA and 3850-byte size match. Example inspected, not executed.


### 2331. `crates/horizon-world/examples/probe/level_shape_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `696ac62ab094353ff88d4439e3f9c3103ff42b1b`
- Size: 5042 bytes
- Coverage: Complete read through EOF: 147 logical lines; exact pinned-tree size 5042 bytes; no omitted ranges.
- Purpose: Block shape-update, survival and scheduled-tick fixture
- Dependencies / cross-references: horizon-world item-use, Java-map, level placement/shape and registry APIs; core Java-compatible key/hash and block-position abstractions.
- Implementation / logic: Runs update_shape/can_survive and emits block/fluid ticks using a synthetic neighbourhood and mock LevelAccess.
- Findings: `is_loaded` is always true; cell arrays, directions, positions and tick IDs are trusted/cast. Not full neighbour scheduling or unloaded-chunk behavior.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate fixture schema and tick IDs; compare shape update, survival, scheduled ticks and RNG calls with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 147 logical lines read through EOF; exact pinned-tree SHA and 5042-byte size match. Example inspected, not executed.


### 2332. `crates/horizon-world/examples/probe/live_attribute_updates_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b053c2c82baac8dbe1223cfb71f7fe2e0ef16b55`
- Size: 22741 bytes
- Coverage: 598 logical lines, read in contiguous ranges 1–400 and 401–598; exact pinned-tree size 22,741 bytes.
- Purpose: Integrated live-attribute, health, effect-notification and update-order fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Combines two LiveAttributes maps, SyncedFloat health, attribute updates, effect notifications and callback failure/mutation.
- Findings: 598 lines read in 1–400 and 401–598. EffectKey::tie_break always Less; only consistent with the fixture's single logical key. Attribute/map indexes, operations and arrays are trusted. Passengers empty, server-player false, connection and send_packet are no-ops.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Add distinct effect-key ordering, malformed indexes and real server-player/passenger/connection/packet scenarios against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 598 logical lines read through EOF in ranges 1–400 and 401–598; exact pinned-tree SHA and 22741-byte size match. Example inspected, not executed.


### 2333. `crates/horizon-world/examples/probe/live_attributes_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1c4a868f51780770b51ba8c7723454f0b2c8159d`
- Size: 7898 bytes
- Coverage: Complete read through EOF: 212 logical lines; exact pinned-tree size 7898 bytes; no omitted ranges.
- Purpose: Entity-catalog live attribute and modifier fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Loads Catalog and exercises value/base/has/modifier/ensure/reset, update/sync acknowledgement and effect-modifier operations.
- Findings: Instance-hash iterator uses next().unwrap(); operation/attribute indexes directly access arrays. Hex/amplifier values are trusted; no real entity lifecycle or network sync.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate entity/catalog keys and hash-list lengths; compare modifier persistence, cache invalidation and dirty/update/sync semantics with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 212 logical lines read through EOF; exact pinned-tree SHA and 7898-byte size match. Example inspected, not executed.


### 2334. `crates/horizon-world/examples/probe/living_air_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `882c5a8f3d748257a3c73d62cd5f3cddcf346f45`
- Size: 7865 bytes
- Coverage: Complete read through EOF: 250 logical lines; exact pinned-tree size 7865 bytes; no omitted ranges.
- Purpose: Living air movement, climbing, friction and input-vector fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Runs air/relative movement, climb handling, speed/friction and input-vector paths with query traces, failures and mutation.
- Findings: Sequence indexes `a[min(n,len-1)]`; empty arrays underflow/panic. JSON flags/bit strings are trusted; fixed support coordinates and synthetic environment.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Reject empty sequences and invalid numeric inputs; compare movement, friction, climb, yaw and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 250 logical lines read through EOF; exact pinned-tree SHA and 7865-byte size match. Example inspected, not executed.


### 2335. `crates/horizon-world/examples/probe/living_damage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d8097bde96c8509fe9d12b0133fd1be8280c633f`
- Size: 5973 bytes
- Coverage: Complete read through EOF: 186 logical lines; exact pinned-tree size 5973 bytes; no omitted ranges.
- Purpose: Living damage, armor/magic, absorption, exhaustion and health fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Exercises actually_hurt with invulnerability, armor/magic, absorption, stats, exhaustion, tracker, health and callback failures.
- Findings: State/bit inputs trusted; conversions and event counts unbounded. Attacker is a unit mock; incomplete DamageSource/death/entity-attribute/network model.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Add malformed bits, extreme damage and real attacker/source cases; compare damage stages, absorption, stats and event order.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 186 logical lines read through EOF; exact pinned-tree SHA and 5973-byte size match. Example inspected, not executed.


### 2336. `crates/horizon-world/examples/probe/living_fall_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `2d65929ff87c37482b86ce11688cd28bb10bf61a`
- Size: 6160 bytes
- Coverage: Complete read through EOF: 191 logical lines; exact pinned-tree size 6160 bytes; no omitted ranges.
- Purpose: Fall-damage and impulse-grace fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Covers calculate/cause fall damage, immunity sequence, passenger propagation, sound and impulse reset/grace/ignore operations.
- Findings: Mode/array indexes are trusted; empty immunity array panics and grace ticks narrow to i32. Passenger/server are synthetic.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate sequence/grace bounds; compare safe-fall, damage multiplier, passenger propagation, sound and grace semantics with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 191 logical lines read through EOF; exact pinned-tree SHA and 6160-byte size match. Example inspected, not executed.


### 2337. `crates/horizon-world/examples/probe/living_fluid_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `bd37049658e096ecd1c4febe4ffcc825f8df8b58`
- Size: 10708 bytes
- Coverage: Complete read through EOF: 331 logical lines; exact pinned-tree size 10708 bytes; no omitted ranges.
- Purpose: Living water/lava movement and air-motion integration fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Combines LivingAirWorld/LivingFluidWorld for fluid travel, gravity, jump-out, float-while-ridden, air and climb paths.
- Findings: Ground/climb/sprint sequences are trusted; empty arrays panic. Numeric fields/selectors are unchecked; chunk, vehicle, fluid and free-space queries are scripted mocks.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate arrays/selectors; compare water/lava movement, swimming efficiency, jump thresholds, float tags and query/RNG order.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 331 logical lines read through EOF; exact pinned-tree SHA and 10708-byte size match. Example inspected, not executed.


### 2338. `crates/horizon-world/examples/probe/living_health_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `53d00b0763049b431094c08b07746738d5a54a69`
- Size: 5191 bytes
- Coverage: Complete read through EOF: 152 logical lines; exact pinned-tree size 5191 bytes; no omitted ranges.
- Purpose: Synced health/absorption, heal/hurt/dead and dirty-packing fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Maintains two SyncedFloat stores and exercises heal/set/base/absorption/hurt/dead/sync/pack with callback failure and active-slot changes.
- Findings: Active data indexes fixed two-element arrays unchecked; JSON and hex floats trusted. Dead/attribute data come from a synthetic context.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Test invalid active slots, NaN/infinite/negative health and mutations; compare clamp, dirty flags, pack timing and heal/hurt with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 152 logical lines read through EOF; exact pinned-tree SHA and 5191-byte size match. Example inspected, not executed.


### 2339. `crates/horizon-world/examples/probe/living_hurt_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b5ab3e890bac72ac13d23f3a5c2805af95eace42`
- Size: 8264 bytes
- Coverage: Complete read through EOF: 243 logical lines; exact pinned-tree size 8264 bytes; no omitted ranges.
- Purpose: Server hurt-state, blocking, immunity, death-protection and criteria fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Models hurt_server through invulnerability/fire/freeze/sleep, blocking, helmet, damage, knockback, death protection, effects and statistics.
- Findings: Effect count unbounded; cooldown/duration/amplifier/counts narrow to i32 and JSON is trusted. Mock source/blocking/sound/player attribution; no live entity packets or Forge hooks.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Bound effect counts and numeric fields; compare cooldown, hurt animation, blocking, death protection and callback failure order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 243 logical lines read through EOF; exact pinned-tree SHA and 8264-byte size match. Example inspected, not executed.


### 2340. `crates/horizon-world/examples/probe/living_landing_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3c3e333d7131cf34b44e2be9aff9c3ff3c2015ba`
- Size: 6643 bytes
- Coverage: Complete read through EOF: 209 logical lines; exact pinned-tree size 6643 bytes; no omitted ranges.
- Purpose: Living fall landing, fluid update, block interaction and particle fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Exercises check_living_fall_damage with mock fluid/ground/block queries, fall_on mutation, particles, climbable reset and callback failure.
- Findings: Support/landing coordinates narrow to i32 and block_position unwraps a required value; block state is fixed u8, no live registry collision shapes.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate position bounds and absent-support cases; compare fluid ordering, fall reset, block callbacks and particles with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 209 logical lines read through EOF; exact pinned-tree SHA and 6643-byte size match. Example inspected, not executed.


### 2341. `crates/horizon-world/examples/probe/living_motion_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `96c0792ac18fdfe16fbe6a4ce3759bda4a125bac`
- Size: 5454 bytes
- Coverage: Complete read through EOF: 164 logical lines; exact pinned-tree size 5454 bytes; no omitted ranges.
- Purpose: Jump power/boost, gravity, swimming, flying speed and delta-movement fixture
- Dependencies / cross-references: horizon-world live attributes, effect notifications, living health/damage/fall/fluid/hurt/landing/motion; core geometry and deterministic random.
- Implementation / logic: Runs living_motion operations against a mock velocity/attribute world; traces queries, errors, velocity mutation and sync flags.
- Findings: Amplifier narrows to i32; bit strings and operation names trusted. Synthetic callbacks do not validate entity physics, movement packets or interpolation.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Test extreme gravity/amplifiers, NaN/infinite motion and sync transitions; compare calculations and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 164 logical lines read through EOF; exact pinned-tree SHA and 5454-byte size match. Example inspected, not executed.


### 2342. `crates/horizon-world/examples/probe/material_condition_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `86017d1eb2a79fbba2636fef93cba78b03f20ffa`
- Size: 26564 bytes
- Coverage: 735 logical lines, read in contiguous ranges 1–400 and 401–735; exact pinned-tree size 26,564 bytes.
- Purpose: Material condition registry/codec, density/biome query and rule evaluation fixture
- Dependencies / cross-references: horizon-world worldgen material conditions/extensions/stage/surface, density/noise/biome/carver/aquifer/terrain/registry/NBT modules and fixture resource snapshots.
- Implementation / logic: Loads condition/biome/tag/noise registries; encodes/decodes conditions, samples density/biomes, compiles sequence/if/ore rules and evaluates them with deterministic random.
- Findings: 735 lines read in ranges 1–400 and 401–735. Condition/rule schemas, anchors and indexes unwrap/direct-index; context/IDs narrow to i32/u32; ore channels index a fixed three-slot array. Numeric JSON normalization uses f64, so preserve integer-vs-double representation when extending the schema.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate dimensions, registry refs, conditions, anchors and ore-channel bounds; test large integer round trips, missing tags and deterministic density/RNG output against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 735 logical lines read through EOF in ranges 1–400 and 401–735; exact pinned-tree SHA and 26564-byte size match. Example inspected, not executed.


### 2343. `crates/horizon-world/examples/probe/material_extension_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d7170b6dc21082436c229f6d7f65ee6cdb39749`
- Size: 6789 bytes
- Coverage: Complete read through EOF: 208 logical lines; exact pinned-tree size 6789 bytes; no omitted ranges.
- Purpose: Badlands erosion and frozen-ocean material-extension fixture
- Dependencies / cross-references: horizon-world worldgen material conditions/extensions/stage/surface, density/noise/biome/carver/aquifer/terrain/registry/NBT modules and fixture resource snapshots.
- Implementation / logic: Builds MaterialExtensions from six positional noise sources; applies eroded_badlands/frozen_ocean to a mock column and traces noise, reads/writes and injected read limits.
- Findings: Column is synthetic; `remaining==0` injects failure while negative means unbounded reads. Block IDs, biome/noise names, heights and coordinates are trusted/cast; same_block has a hard-coded alias map.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Bound read limits and coordinates; compare badlands/iceberg block selection, noise forks and read/write ordering against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 208 logical lines read through EOF; exact pinned-tree SHA and 6789-byte size match. Example inspected, not executed.


### 2344. `crates/horizon-world/examples/probe/material_stage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `31a563f7c6ae4935ce131af94ffdd4476164c5a9`
- Size: 28545 bytes
- Coverage: 828 logical lines, read in contiguous ranges 1–400, 401–800 and 801–828; exact pinned-tree size 28,545 bytes.
- Purpose: Integrated terrain material stage, chunk fill, biome neighbourhood, aquifer, carving and heightmap fixture
- Dependencies / cross-references: horizon-world worldgen material conditions/extensions/stage/surface, density/noise/biome/carver/aquifer/terrain/registry/NBT modules and fixture resource snapshots.
- Implementation / logic: Loads block/biome/noise/density/router/aquifer/carver/rule registries; builds TerrainChunk, fills noise, applies surface/top material and carving, then dumps sections, heightmaps and postprocessing.
- Findings: 828 lines read in 1–400, 401–800 and 801–828. Height/min/dimensions and min+height−1/min+height−8 arithmetic are not fully checked; usize casts can amplify invalid inputs. State arrays/properties/registry lookups unwrap. Results depend on private registry snapshots and mock policies, not canonical vanilla 1.20.1/Forge data.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate height alignment, bounded sections, non-empty biome choices and registry completeness; compare seeded blocks, biome quart samples, aquifers, surface rules, carvers, heightmaps and postprocessing byte-for-byte with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 828 logical lines read through EOF in ranges 1–400, 401–800 and 801–828; exact pinned-tree SHA and 28545-byte size match. Example inspected, not executed.


### 2345. `crates/horizon-world/examples/probe/material_surface_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3b3373f8650af3280c54db9fc997248a04d3e545`
- Size: 4375 bytes
- Coverage: Complete read through EOF: 137 logical lines; exact pinned-tree size 4375 bytes; no omitted ranges.
- Purpose: Clay-band generation, positional-noise and surface-query fixture
- Dependencies / cross-references: horizon-world worldgen material conditions/extensions/stage/surface, density/noise/biome/carver/aquifer/terrain/registry/NBT modules and fixture resource snapshots.
- Implementation / logic: Generates ClayBands with Legacy/Xoroshiro, creates MaterialSurface with positional noise and tests band/depth/secondary queries.
- Findings: Point coordinates narrow to i32; override arrays assume three values; missing noise or invalid hex unwrap. ConstantNoise asserts a restricted query dimension; not a full material stage.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate point/range inputs and override length; compare band generation, noise forks and surface depth/secondary values with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 137 logical lines read through EOF; exact pinned-tree SHA and 4375-byte size match. Example inspected, not executed.


### 2346. `crates/horizon-world/examples/probe/movement_emission_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `70ef1b32b87fa1a6d562e02e5ed86da52b61b6e7`
- Size: 6661 bytes
- Coverage: Complete read through EOF: 209 logical lines; exact pinned-tree size 6661 bytes; no omitted ranges.
- Purpose: Movement sound/event emission and distance mutation fixture
- Dependencies / cross-references: horizon-world movement emission/properties/support and object_open_map/object_open_set; NBT block registry, tags, geometry and callback-based mock worlds.
- Implementation / logic: Runs block movement effects and apply_movement_emission for sound/event modes; traces walking/swimming/flap callbacks and distance mutation.
- Findings: MovementEmission directly indexes a four-item array from unvalidated input; callback state indexes and JSON arrays are trusted. Synthetic sound/event hooks do not deliver packets or play audio.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate emission/state selectors; compare sound/event order, mutation timing and distance thresholds with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 209 logical lines read through EOF; exact pinned-tree SHA and 6661-byte size match. Example inspected, not executed.


### 2347. `crates/horizon-world/examples/probe/movement_registry_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `28a934ee5efad582e98777a3c8fab9d0ea978601`
- Size: 6241 bytes
- Coverage: Complete read through EOF: 178 logical lines; exact pinned-tree size 6241 bytes; no omitted ranges.
- Purpose: NBT movement properties, vanilla tags and movement-support binding fixture
- Dependencies / cross-references: horizon-world movement emission/properties/support and object_open_map/object_open_set; NBT block registry, tags, geometry and callback-based mock worlds.
- Implementation / logic: Loads block registry/movement-properties NBT, builds tags, binds default/empty/all tag sets, dumps coefficients and tests support/jump/speed queries.
- Findings: State provider indexes `states[call % states.len()]`; empty array panics. Required tags and registry facts assert/unwrap; corpus is tied to a resource snapshot.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Test empty/malformed registry/tag data and versioned coefficients; compare support offsets and per-state values with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 178 logical lines read through EOF; exact pinned-tree SHA and 6241-byte size match. Example inspected, not executed.


### 2348. `crates/horizon-world/examples/probe/movement_support_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1206aa0dc2e0a03402505303be5c916c8efae874`
- Size: 4889 bytes
- Coverage: Complete read through EOF: 143 logical lines; exact pinned-tree size 4889 bytes; no omitted ranges.
- Purpose: Support position, fence/wall/water/bubble and movement-factor fixture
- Dependencies / cross-references: horizon-world movement emission/properties/support and object_open_map/object_open_set; NBT block registry, tags, geometry and callback-based mock worlds.
- Implementation / logic: Runs support_position with custom/legacy/on-support/movement offsets and block jump/speed factors against synthetic MovementBlockWorld.
- Findings: State selection uses `reads % states.len()` and panics on empty inputs; mode/state selectors and factors are trusted. No real collision-shape/chunk query.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Validate state lists and mode values; compare fence-gate, wall/fence, bubble and water behavior with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 143 logical lines read through EOF; exact pinned-tree SHA and 4889-byte size match. Example inspected, not executed.


### 2349. `crates/horizon-world/examples/probe/object_open_map_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ae03169ddcdf4ce2a279bbb22720a098b59c60a9`
- Size: 6317 bytes
- Coverage: Complete read through EOF: 169 logical lines; exact pinned-tree size 6317 bytes; no omitted ranges.
- Purpose: ObjectOpenMap identity, cursor and mutation-during-iteration fixture
- Dependencies / cross-references: horizon-world movement emission/properties/support and object_open_map/object_open_set; NBT block registry, tags, geometry and callback-based mock worlds.
- Implementation / logic: Exercises put/remove/get/contains/clear, cursor next/delete, entry values and for_each callbacks that mutate/fail while emitting snapshots.
- Findings: Key/entry indexes directly access arrays; values narrow to i32 and malformed operation/action arrays panic. Key equality is ID-only while object/hash are independent.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Add equal-ID/different-object, hash-collision, remove-current, clear/reinsert and callback-mutation tests against Java map behavior.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 169 logical lines read through EOF; exact pinned-tree SHA and 6317-byte size match. Example inspected, not executed.


### 2350. `crates/horizon-world/examples/probe/object_open_set_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c624d336d64c3480467934eda0053d3daf629424`
- Size: 3384 bytes
- Coverage: Complete read through EOF: 96 logical lines; exact pinned-tree size 3384 bytes; no omitted ranges.
- Purpose: ObjectOpenSet equality, cursor, removal and membership fixture
- Dependencies / cross-references: horizon-world movement emission/properties/support and object_open_map/object_open_set; NBT block registry, tags, geometry and callback-based mock worlds.
- Implementation / logic: Exercises add/remove/contains/get/clear and cursor next/force-next/delete with synthetic object keys and configurable hashes.
- Findings: Hash/ID arrays and key indexes are unchecked; operation values narrow and malformed modes panic. Container contract fixture only; no concurrency proof.
- RustChunk relevance: Focused differential-test design/reference for Minecraft Java 1.20.1 semantics; synthetic Starling fixture, not production gameplay/client code.
- Decision: adaptable; reference
- 1.20.1 implications: Does not establish canonical vanilla 1.20.1 registry/data, packet, entity, movement or Forge 47.x compatibility. Compare with version-pinned Java oracle outputs; no Aternos claim follows.
- Follow-up checks: Test equal IDs/different objects, hash collisions, cursor invalidation and mutation-after-delete against Java collection behavior.
- RustChunk implementation mapping: Keep in developer-side fixtures/tools outside the Android release graph. Recreate scenarios with RustChunk-owned version-pinned types and Java oracle outputs; do not import synthetic mock worlds or later-version registry assumptions into production.
- Evidence required before adoption: Exact Java 1.20.1 reference output and registry/NBT resources, malformed/edge-case corpus, executed differential tests and separate Android/Forge integration evidence.
- Completion state: COMPLETE — 96 logical lines read through EOF; exact pinned-tree SHA and 3384-byte size match. Example inspected, not executed.


## Checkpoint 2326–2350 — COMPLETE (25/25); reporting section 2301–2350 CLOSED

- Entries 2326–2350 were inspected individually through EOF in deterministic lexical path order after excluding the full Index. Exact paths, blob SHAs and sizes match the complete pinned tree.
- Batch contents: item-use lifecycle, Java hash/identity maps, level placement/shape, live attributes/effect notifications, living air/damage/fall/fluid/health/hurt/landing/motion, material conditions/extensions/stage/surface, movement emission/registry/support and object-open maps/sets.
- These are test/fixture transports with synthetic worlds, fixed registries/arrays, trusted JSON/NBT and unchecked indexing/numeric narrowing; not production implementations or proof of Java 1.20.1/Forge parity. Two JavaKey implementations return invalid Less ordering for equal identities (unit EffectKey in 2332; identity comparator in 2329). Empty sequence arrays can panic in living-air/fluid fixtures. Material-stage is a broad harness but depends on private registry snapshots and unchecked chunk-height/section bounds.
- Long-file coverage: `live_attribute_updates_fixture.rs` 1–400 and 401–598; `material_condition_fixture.rs` 1–400 and 401–735; `material_stage_fixture.rs` 1–400, 401–800 and 801–828.
- Active Ledger: **150 records (2201–2350)**, exactly 15 fields each. Index: **2,350 sequential unique paths** with matching path/Decision rows.
- No tests/examples were executed; no RustChunk application source changed; no build, benchmark, APK, device, renderer, Minecraft, Forge, server or Aternos validation.
- The one-time authorization covers **2301–2400**. Section **2301–2350 is CLOSED (50/50)**. Continue checkpoints **2351–2375** and **2376–2400** without asking; **STOP at 2400** and request fresh permission before 2401.

### 2351. `crates/horizon-world/examples/probe/piston_movement_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `804ccec2e7365f22297e0abde655cf855ee6f4da`
- Size: 2112 bytes
- Coverage: Complete read through EOF: 60 logical lines; exact pinned-tree size 2112 bytes.
- Purpose: Piston movement delta limiting and game-time callback fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Builds PistonMovement from initial game time and XYZ deltas; calls limit for scripted movement vectors while capturing callback count, injected time failure, resulting movement, stored deltas and tick.
- Findings: Hex-float and operation fields are trusted JSON and parsed with unwrap; the fixture covers the state helper and callback sequencing, not a full moving-piston block entity or entity push lifecycle.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate NaN/infinite and negative delta/time cases; compare tick-expiry boundary, callback failure and all movement axes with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 60 logical lines read through EOF; exact pinned-tree SHA and 2112-byte size match. Example/utility inspected, not executed.


### 2352. `crates/horizon-world/examples/probe/piston_push_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1d40cd610e1389127971a9a0cf6cec13e2523f09`
- Size: 5628 bytes
- Coverage: Complete read through EOF: 163 logical lines; exact pinned-tree size 5628 bytes.
- Purpose: Piston push geometry, collision movement area and sticky criteria fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Computes extended progress, moved-block origin, swept movement area, per-face push distance, collided-entity displacement, search/sticky boxes and push-reaction matching over supplied AABBs.
- Findings: AABB coordinates are read by unchecked six-element indexing; positions narrow i64 to i32; direction/reaction are normalized with rem_euclid, but box lists and hex numbers remain trusted. No live entity broad-phase, chunk or block-entity state.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test degenerate/inverted AABBs, negative coordinates, all push reactions and delta cap boundaries; compare entity ordering and sticky criteria with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 163 logical lines read through EOF; exact pinned-tree SHA and 5628-byte size match. Example/utility inspected, not executed.


### 2353. `crates/horizon-world/examples/probe/piston_shape_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `ec75a95028a79bc9168c3062cb589b9d058304cd`
- Size: 4754 bytes
- Coverage: Complete read through EOF: 127 logical lines; exact pinned-tree size 4754 bytes.
- Purpose: Moving-piston state interpolation, related-state and collision-shape fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Loads block registry and cached collision NBT; computes piston movement direction/progress/offset, related state and dynamic collision shape with an injected block-entity query failure.
- Findings: Direction directly indexes Direction::ALL; state indexes cached/dynamic registry arrays; nonnegative noclip directly indexes Direction::ALL. Dynamic-support entries are expected to exist and use expect; fixture NBT/corpus is trusted.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate direction/state/noclip ranges and missing dynamic-support values; compare interpolation, noclip, cached/dynamic shape and callback errors with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 127 logical lines read through EOF; exact pinned-tree SHA and 4754-byte size match. Example/utility inspected, not executed.


### 2354. `crates/horizon-world/examples/probe/placement_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6214e630eae1284c56b584b7111053a3a2cb9fb8`
- Size: 3413 bytes
- Coverage: Complete read through EOF: 83 logical lines; exact pinned-tree size 3413 bytes.
- Purpose: Worldgen placement-modifier decode, position emission and random-state fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Decodes PlacementModifier JSON; runs count-only repetitions or position modification through mock placement queries, injected consumer failure and optional RNG draws; records next/tail random values for Legacy/Xoroshiro.
- Findings: World-generation context and positions narrow i64 to i32; JSON fields and operation modes are trusted. Query semantics are delegated to a fixture transport, not a live chunk/worldgen scheduler.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test invalid definitions, negative/extreme coordinates, repetition bounds, partial consumer failure and exact RNG draw order against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 83 logical lines read through EOF; exact pinned-tree SHA and 3413-byte size match. Example/utility inspected, not executed.


### 2355. `crates/horizon-world/examples/probe/player_equipment_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0a356e31dedd8f5663ff791e86fb3688e6d0d912`
- Size: 9745 bytes
- Coverage: Complete read through EOF: 273 logical lines; exact pinned-tree size 9745 bytes.
- Purpose: Player equipment slots, hand access and inventory-callback mutation fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Builds PlayerInventory/EntityEquipment and reference stacks; exercises get/set hand, equipment get/set, clear, selected slot and callback failure/mutation while snapshotting aliases and equipment views.
- Findings: Snapshot loops hard-code 43 inventory slots; EquipmentSlot::ALL and stack-reference vectors are indexed without bounds checks; numeric fields narrow to u8/i32. Mock callbacks can mutate selection, slots and stack counts, not actual entity synchronization.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate slot/ref selectors and callback partial-mutation semantics; compare hand/armor/body/saddle slots, shared equipment aliases and change notification with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 273 logical lines read through EOF; exact pinned-tree SHA and 9745-byte size match. Example/utility inspected, not executed.


### 2356. `crates/horizon-world/examples/probe/player_food_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `186ec63327c40aec1c42aa8ea26408a2c20426b6`
- Size: 4565 bytes
- Coverage: Complete read through EOF: 139 logical lines; exact pinned-tree size 4565 bytes.
- Purpose: Player food exhaustion, can-eat and manoeuvre eligibility fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Exercises PlayerFoodWorld operations for exhaustion, food checks and enough-food-for-manoeuvres using two FoodData slots, stored/direct state paths and callback failure/mutation.
- Findings: Direct/virtual indexes select fixed two-slot food arrays without range checks; bit values and fields unwrap. StoredPlayerFood branch unwraps run() so an injected callback error can panic instead of being emitted as a normal fixture error.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test both food slots, invalid indexes, stored/direct equivalence and callback-error behavior against Java 1.20.1 player food semantics.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 139 logical lines read through EOF; exact pinned-tree SHA and 4565-byte size match. Example/utility inspected, not executed.


### 2357. `crates/horizon-world/examples/probe/player_health_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `24163bb759deb3a785a58cab3c23c07256c4c935`
- Size: 8209 bytes
- Coverage: Complete read through EOF: 236 logical lines; exact pinned-tree size 8209 bytes.
- Purpose: Player attribute health/absorption and SyncedFloats callback fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Binds PlayerAttributeHealth to entity attributes and synced health/absorption floats; exercises set/heal/absorption/get/hurt/dead, base/modifier mutations, assign, dirty packing and observer failures.
- Findings: Health/absorption IDs narrow to u8; attribute selector directly indexes a two-key array; Operation::ALL is directly indexed. JSON/hex values and assigned field selectors are trusted; no live entity or network sync.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate IDs, operation/field selectors, NaN/infinite and negative values; compare max-health clamp, absorption, dirty flags and observer ordering with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 236 logical lines read through EOF; exact pinned-tree SHA and 8209-byte size match. Example/utility inspected, not executed.


### 2358. `crates/horizon-world/examples/probe/player_hurt_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `6e77abf71d1137c3a0316f76de8957cdb764cd4a`
- Size: 10509 bytes
- Coverage: Complete read through EOF: 307 logical lines; exact pinned-tree size 10509 bytes.
- Purpose: Player hurt and server-player hurt state-machine fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Exercises HurtState cooldown/invulnerability, tags, blocking, helmet damage, actual hurt, knockback, death protection, sounds, criteria and attacker/player/arrow-owner decisions with failure injection.
- Findings: Difficulty selection uses an unchecked fixed enum index; attacker mode uses a small hard-coded match and panics outside 0–3. Durations/counts narrow to i32 and JSON/hex inputs unwrap; sound/entity/network adapters are synthetic.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate attacker/difficulty ranges and extreme cooldown/damage values; compare client/server paths, death protection, blocking and event order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 307 logical lines read through EOF; exact pinned-tree SHA and 10509-byte size match. Example/utility inspected, not executed.


### 2359. `crates/horizon-world/examples/probe/player_inventory_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `418fb9a3ea7dec1e399ef11a743be5ad6e6170f1`
- Size: 11317 bytes
- Coverage: Complete read through EOF: 303 logical lines; exact pinned-tree size 11317 bytes.
- Purpose: Player inventory, crafting/picking, equipment sharing and two-inventory mutation fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Models two PlayerInventory values, ItemStacks/StackRefs, shared or separate EntityEquipment and operations for get/set/remove/pick/add/replace/equipment/copy/alias/component changes.
- Findings: Snapshots hard-code 43 slots; inventory target, refs, prototype, equipment order and component indexes are trusted and directly indexed; item/count values narrow to u8/i32. It models Horizon component stacks, not 1.20.1 legacy ItemStack NBT or Forge capabilities.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test shared-equipment aliasing, replacement across inventories, malformed slots/prototypes, stack limits and callback-free mutation; map behavior separately to Java 1.20.1/Forge 47.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 303 logical lines read through EOF; exact pinned-tree SHA and 11317-byte size match. Example/utility inspected, not executed.


### 2360. `crates/horizon-world/examples/probe/player_travel_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `e4d4936625e61bde6b652d9e56de9629071d9b12`
- Size: 4814 bytes
- Coverage: Complete read through EOF: 155 logical lines; exact pinned-tree size 4814 bytes.
- Purpose: Player travel, fluid-affect, flying speed and velocity mutation fixture
- Dependencies / cross-references: horizon-world piston, placement, player equipment/food/health/hurt/inventory/travel modules; core AABB/vector/random and item-stack abstractions.
- Implementation / logic: Runs travel, flying_speed and affected_by_fluids with passenger/swimming/look/fluid/ability/sprint callbacks; captures velocity/position/flying mutation and injected query failure.
- Findings: `living_travel` is intentionally scripted with a hard-coded fluid query at (17,-64,-33), not a real world/block lookup. Bit fields and modes unwrap; state is mock, so it cannot establish actual fluid/chunk/player-travel behavior.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate travel modes and extreme vectors; compare flying speed, passenger/swimming branches and actual fluid-position queries with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 155 logical lines read through EOF; exact pinned-tree SHA and 4814-byte size match. Example/utility inspected, not executed.


### 2361. `crates/horizon-world/examples/probe/portal_travel_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `cda3ae2487170dac062eacf917b3bac16ab164d5`
- Size: 8901 bytes
- Coverage: Complete read through EOF: 272 logical lines; exact pinned-tree size 8901 bytes.
- Purpose: Portal travel decision, dimension transfer and entity placement fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Models source/destination dimensions and portal travel decisions, position/velocity updates and callback traces using a mock multi-world environment.
- Findings: World/chunk availability is scripted and treated as loaded; dimension/position/velocity values are fixture inputs. Does not model real chunk tickets, portal search or cross-dimension network entity synchronization.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Compare portal search, destination chunk loading, safe landing, velocity rotation and player/entity packet ordering with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 272 logical lines read through EOF; exact pinned-tree SHA and 8901-byte size match. Example/utility inspected, not executed.


### 2362. `crates/horizon-world/examples/probe/registered_player_combat_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c7088116516dbaeb39299aefce244ec45a8dd4d4`
- Size: 20760 bytes
- Coverage: 609 lines read in contiguous ranges 1–400 and 401–609; exact pinned-tree size 20,760 bytes.
- Purpose: Registered-player combat and attribute-store integration fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Compares combat hooks using entity-attribute and live-attribute stores, exercising registered player values, modifiers and callback order with synthetic player/weapon/source adapters.
- Findings: 609 lines read in contiguous ranges 1–400 and 401–609. Several unsupported hooks deliberately panic to assert the chosen path; input selectors/catalog keys/operation indices are trusted. Mock player/source callbacks are not live combat or packet delivery.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test every intentionally unsupported path separately; compare modifier calculations, attacker ownership, server/client decisions and callback traces with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 609 logical lines read through EOF in contiguous chunks of at most 400 lines; exact pinned-tree SHA and 20760-byte size match. Example/utility inspected, not executed.


### 2363. `crates/horizon-world/examples/probe/registry_collision_scan_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `b66812142b54e86c2fca6a59c5386a3e3d9cfcf6`
- Size: 20676 bytes
- Coverage: 591 lines read in contiguous ranges 1–400 and 401–591; exact pinned-tree size 20,676 bytes.
- Purpose: Registry-wide collision-shape scan, block-state geometry and query-budget fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Loads registry/cached collision data and scans supplied block-state/shape combinations, recording box/collision results, query order and bounded scan behavior.
- Findings: 591 lines read in contiguous ranges 1–400 and 401–591. Scan/corpus bounds are fixture-defined; registry state indexes and shape data rely on private NBT snapshots. It does not prove live chunk collision or renderer/GPU behavior.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate registry version/hash and scan limits; compare all canonical 1.20.1 collision shapes, dynamic shapes, query counts and state ordering with Java.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 591 logical lines read through EOF in contiguous chunks of at most 400 lines; exact pinned-tree SHA and 20676-byte size match. Example/utility inspected, not executed.


### 2364. `crates/horizon-world/examples/probe/registry_fall_ray_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c4f61644ca549d6cc5dd956fc09140c3cc3960b0`
- Size: 6331 bytes
- Coverage: Complete read through EOF: 193 logical lines; exact pinned-tree size 6331 bytes.
- Purpose: Registry-bound fall/ray collision and shape-query fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Uses registry collision/shape data to exercise fall-ray queries and records shape/hit behavior against a synthetic context.
- Findings: State/position indexes and geometric bit inputs are trusted; registry corpus is pinned to private data. No real entity movement, unloaded chunk or server-side ray packet path.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Compare hit ordering, boundary/tie cases, negative coordinates and fall/collision semantics with Java 1.20.1 and canonical shape resources.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 193 logical lines read through EOF; exact pinned-tree SHA and 6331-byte size match. Example/utility inspected, not executed.


### 2365. `crates/horizon-world/examples/probe/registry_fluid_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `45cd272b0cbfe6fd05025af701f3c3982786faf8`
- Size: 6219 bytes
- Coverage: Complete read through EOF: 172 logical lines; exact pinned-tree size 6219 bytes.
- Purpose: Block-registry fluid binding and fluid-state lookup fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Loads block registry/fluid facts and exercises state-to-fluid bindings, source/falling/level lookup and registry-derived fluid classifications.
- Findings: Bindings depend on fixture NBT/registry snapshots; state IDs and tag/name inputs are trusted. This does not independently verify vanilla 1.20.1 fluid tags or full fluid physics.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Compare all state/fluid bindings and tag membership against version-pinned Java 1.20.1 registries; include unknown and missing states.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 172 logical lines read through EOF; exact pinned-tree SHA and 6219-byte size match. Example/utility inspected, not executed.


### 2366. `crates/horizon-world/examples/probe/replace_block_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `d0a742f99da0f2327dc9c40d8aedac4b27d678f9`
- Size: 6832 bytes
- Coverage: Complete read through EOF: 194 logical lines; exact pinned-tree size 6832 bytes.
- Purpose: Block replacement rules and placement-context decision fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Exercises replaceability/placement checks, state and context queries, callback failure and block mutation over a synthetic placement world.
- Findings: Mock `height()` deliberately returns Unavailable, so paths requiring a heightmap are not covered; block/state/context indexes and JSON values are trusted. The synthetic world does not model real heightmaps or chunk availability.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Add available/missing/edge heightmap cases; compare replacement and placement predicates with Java 1.20.1 for all relevant blocks.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 194 logical lines read through EOF; exact pinned-tree SHA and 6832-byte size match. Example/utility inspected, not executed.


### 2367. `crates/horizon-world/examples/probe/restitution_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `0d630274e931c0d6318e406907639a84049dd88c`
- Size: 5144 bytes
- Coverage: Complete read through EOF: 156 logical lines; exact pinned-tree size 5144 bytes.
- Purpose: Velocity restitution, collision impulse and response fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Exercises restitution calculations and movement/velocity response using supplied vectors and scalar coefficients, recording exact float outputs.
- Findings: Vectors/scalars arrive as hex float strings and are unwrapped without finite/range validation; fixture does not execute a real collision solver or entity physics loop.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test negative/zero/extreme restitution and non-finite vectors; compare collision response with Java 1.20.1 physics.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 156 logical lines read through EOF; exact pinned-tree SHA and 5144-byte size match. Example/utility inspected, not executed.


### 2368. `crates/horizon-world/examples/probe/retrogen_data_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `3980d12349e6a9d36edd58f28da4b53f20100f5c`
- Size: 3743 bytes
- Coverage: Complete read through EOF: 112 logical lines; exact pinned-tree size 3743 bytes.
- Purpose: Below-zero retrogen metadata, mask and NBT persistence fixture
- Dependencies / cross-references: horizon-world portal, registered combat, collision/shape/fluid/replace/retrogen modules; pinned NBT registries and core geometry/worldgen.
- Implementation / logic: Loads/serializes retrogen fields and masks, exercises bit/section metadata and round-trip output for synthetic chunk retrogen data.
- Findings: NBT/JSON field shapes and mask sizes are trusted; height/section conversions and bit operations are not a general hostile-input validator. Does not run actual chunk retrogen scheduling.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Compare exact 1.20.1 retrogen keys, section masks, below-zero status transitions and serialization against Java NBT fixtures.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 112 logical lines read through EOF; exact pinned-tree SHA and 3743-byte size match. Example/utility inspected, not executed.


### 2369. `crates/horizon-world/examples/probe/rule_test_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `42a25532c46e39a50f2fe70cb5ead77a86746ef6`
- Size: 3305 bytes
- Coverage: Complete read through EOF: 85 logical lines; exact pinned-tree size 3305 bytes.
- Purpose: Worldgen rule decoding and deterministic test fixture
- Dependencies / cross-references: horizon-world worldgen rule and feature/scattered-ore/state-provider registries; core random and placement APIs.
- Implementation / logic: Decodes rule definitions and evaluates them against supplied world/context facts, capturing decisions and query traces.
- Findings: Height/state arrays are indexed with modulo and can panic on empty input; JSON schemas, IDs and repeat counts are trusted.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate empty/oversized arrays and rule IDs; compare rule decisions and query order against Java 1.20.1 worldgen rules.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 85 logical lines read through EOF; exact pinned-tree SHA and 3305-byte size match. Example/utility inspected, not executed.


### 2370. `crates/horizon-world/examples/probe/scattered_ore_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `91b6821b73c2465c27d772ee05c324cfdbc3c15d`
- Size: 7601 bytes
- Coverage: Complete read through EOF: 218 logical lines; exact pinned-tree size 7601 bytes.
- Purpose: Scattered ore feature placement and block-write fixture
- Dependencies / cross-references: horizon-world worldgen rule and feature/scattered-ore/state-provider registries; core random and placement APIs.
- Implementation / logic: Runs ScatteredOreFeature against a mock block world with configured movement offsets, block selection, placement and write decisions.
- Findings: Mode values index a fixed six-offset array without full bounds validation; `height()` is Unavailable in the mock, so height-dependent branches are not covered. Registry/state inputs are fixture-specific.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test every movement mode and available heightmap, random draw count and failed block writes against Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 218 logical lines read through EOF; exact pinned-tree SHA and 7601-byte size match. Example/utility inspected, not executed.


### 2371. `crates/horizon-world/examples/probe/server_player_combat_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `9607dcec21ff57b4d241e72764c13531f9692247`
- Size: 21702 bytes
- Coverage: 629 lines read in contiguous ranges 1–400 and 401–629; exact pinned-tree size 21,702 bytes.
- Purpose: Server-player combat integration across entity and live attribute stores
- Dependencies / cross-references: horizon-world ServerPlayerCombat/ServerPlayerDamage, attributes, synced floats, food, damage stats and combat hooks.
- Implementation / logic: Runs the ServerPlayerCombat pipeline with two alternative attribute stores and traces player/weapon/source/combat callbacks and state.
- Findings: 629 lines read in ranges 1–400 and 401–629. Several callbacks intentionally panic because only the registered player path is expected; player, weapon and source adapters are mocks, not real packets/server entities.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Exercise unsupported attacker/source paths separately; compare registered player damage, attribute changes and callback order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 629 logical lines read through EOF in contiguous chunks of at most 400 lines; exact pinned-tree SHA and 21702-byte size match. Example/utility inspected, not executed.


### 2372. `crates/horizon-world/examples/probe/server_player_damage_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `1242e95a2e569d3e9048a74815abfed6a41eafeb`
- Size: 9087 bytes
- Coverage: Complete read through EOF: 272 logical lines; exact pinned-tree size 9087 bytes.
- Purpose: Server-player damage pipeline, bypass flags, armor/resistance and synced health fixture
- Dependencies / cross-references: horizon-world ServerPlayerCombat/ServerPlayerDamage, attributes, synced floats, food, damage stats and combat hooks.
- Implementation / logic: Builds player attributes, SyncedFloats and FoodData; executes ServerPlayerDamage::apply with armor/effect/resistance bypass, callback failure, mutation, stat and damage tracking traces.
- Findings: `modify_armor_effectiveness` returns its input unchanged, so armor-modifier hooks are not actually applied in this fixture. Attacker-related methods deliberately panic for the player-only path; JSON attribute/operation/slot indexes are trusted.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Add armor modifier and attacker path coverage; test resistance/bypass ordering and health/absorption changes with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 272 logical lines read through EOF; exact pinned-tree SHA and 9087-byte size match. Example/utility inspected, not executed.


### 2373. `crates/horizon-world/examples/probe/simple_block_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `232e56f9463532d4ef5d66d71d52340d8fadb149`
- Size: 28547 bytes
- Coverage: 773 lines read in contiguous ranges 1–400 and 401–773; exact pinned-tree size 28,547 bytes.
- Purpose: Integrated simple-block and placed-feature behavior, survival/support/collision, biome weather and world side effects fixture
- Dependencies / cross-references: horizon-world block registry/properties/support/survival/collision/fluid, state provider, simple block feature/feature graph/placement, biome temperature/weather and worldgen modules.
- Implementation / logic: Loads block/fluid/property/support/collision registries and feature/state-provider/catalog bindings; replays simple-block, feature graph and placed-feature operations against a mutable mock world with block writes, loot-table metadata, gateways, biome snow/freeze, postprocessing, ticks and legacy/Xoroshiro random traces.
- Findings: 773 lines read in ranges 1–400 and 401–773. World height/generation is hard-coded (-64..319); state/position/registry/feature inputs unwrap or cast unchecked. Chunk load state and real server/entity/network behavior are not modeled. The fixture consumes private registry/NBT/catalog snapshots and is not proof of canonical 1.20.1/Forge data parity.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Validate registry hashes, world-height boundaries, feature graph/placed-feature errors, biome weather and block-write failures; compare seeded output, loot/gateway side effects and RNG draw order with Java 1.20.1.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 773 logical lines read through EOF in ranges 1–400 and 401–773; exact pinned-tree SHA and 28547-byte size match. Example/utility inspected, not executed.


### 2374. `crates/horizon-world/examples/probe/simple_container_fixture.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `c6aef9c2a26c7d06c6744d2c31bfe03497144829`
- Size: 8842 bytes
- Coverage: Complete read through EOF: 253 logical lines; exact pinned-tree size 8842 bytes.
- Purpose: SimpleContainer stack refs, component patches, slot mutation and callback behavior fixture
- Dependencies / cross-references: horizon-world SimpleContainer, ItemStacks/StackRef and component_patch Prototype/Patch; JSON line transport.
- Implementation / logic: Builds ItemStacks/StackRefs and SimpleContainer; covers set/add/remove/remove-type/remove-all/clear/get/can-add, alias/copy/component mutations and callback failure/mutation snapshots.
- Findings: Prototype/ref/slot/component indexes and numeric values are trusted and narrowed; many snapshot and callback writes unwrap. The mock uses small item/count queries (0–2) and does not model real player inventory packets or Forge capabilities.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Test duplicate/aliased stacks, max-stack changes, malformed refs and partial callback failure; compare SimpleContainer merge/split semantics with Java 1.20.1 and map Forge capabilities separately.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 253 logical lines read through EOF; exact pinned-tree SHA and 8842-byte size match. Example/utility inspected, not executed.


### 2375. `crates/horizon-world/examples/probe/spawn_origin.rs`
- Source repository/ref: starling-client/starling @ 5432370d573642041585c69b40fb045b27133488
- Source SHA: `7162a00a8a5e066ff8559c8da37ef74e12fe9f61`
- Size: 1701 bytes
- Coverage: Complete read through EOF: 44 logical lines; exact pinned-tree size 1701 bytes.
- Purpose: Overworld generator spawn-position and origin parity command-line utility
- Dependencies / cross-references: horizon-world OverworldGenerator/GeneratorData, BlockRegistry, worldgen resources and core chunk positions.
- Implementation / logic: Loads GeneratorData and a BlockRegistry from client worldgen/facts resources; for each seed computes spawn position and NoiseBasedChunkGenerator origin coordinates/chunk.
- Findings: The registry is intentionally leaked with Box::leak for a process-lifetime reference; seed list is unbounded CLI input. The utility depends on supplied client resource snapshot and DATA_VERSION and emits parity lines only.
- RustChunk relevance: Focused differential-test or tooling reference for reconstructing Minecraft Java 1.20.1 behavior; not production gameplay/client implementation.
- Decision: adaptable; reference
- 1.20.1 implications: These are Starling test/fixture utilities and mocks. They do not by themselves establish vanilla Java 1.20.1 or Forge 47.x behavior, packet compatibility, mod loading or Aternos connectivity. Version-pinned Java oracle data and runtime integration are required.
- Follow-up checks: Run with version-hashed vanilla 1.20.1 resources; compare spawn column/origin over fixed seed corpus with Java NoiseBasedChunkGenerator.
- RustChunk implementation mapping: Keep in developer-side test/fixture tools, not the Android release dependency graph. Rebuild scenario coverage with RustChunk-owned 1.20.1 types and canonical Java oracle outputs; do not ship mock worlds or private test-resource assumptions as production code.
- Evidence required before adoption: Pinned Java 1.20.1 reference outputs and registry/NBT resources; positive/negative/malformed fixture corpus; executed differential tests and separate Android/Forge integration results. Aternos compatibility requires a live connection test.
- Completion state: COMPLETE — all 44 logical lines read through EOF; exact pinned-tree SHA and 1701-byte size match. Example/utility inspected, not executed.


## Checkpoint 2351–2375 — COMPLETE (25/25)

- Entries 2351–2375 were inspected individually through EOF in deterministic lexical order after the complete Index boundary. Exact source paths, blob SHAs and byte sizes match the pinned recursive tree.
- Coverage includes piston movement/push/shape, placement modifiers, player equipment/food/health/hurt/inventory/travel, portal travel, registered/server player combat and damage, registry collision/fall-ray/fluid, block replacement/restitution/retrogen, rule testing, scattered ore, integrated simple-block/placed-feature behavior, SimpleContainer and spawn-origin tooling.
- These are test/fixture utilities with synthetic worlds, fixed registry snapshots, trusted JSON/NBT and unchecked indexes/casts; they are not production client features. Notable limits: PlayerFood's stored branch unwraps callback errors; PlayerTravel's parent dispatch uses a scripted hard-coded fluid position; ServerPlayerDamage leaves armor effectiveness unchanged; ReplaceBlock and ScatteredOre mocks return Unavailable for height; SimpleBlock hard-codes the 1.20.1 world-height context; SpawnOrigin is a CLI parity tool based on supplied resources.
- Long-file coverage: `registered_player_combat_fixture.rs` 1–400/401–609; `registry_collision_scan_fixture.rs` 1–400/401–591; `server_player_combat_fixture.rs` 1–400/401–629; `simple_block_fixture.rs` 1–400/401–773.
- Active Ledger contains **175 records (2201–2375)** with exactly 15 fields per entry. Index contains **2,375 sequential unique paths** with matching path/Decision rows.
- No examples/tests were executed; no application source changed; no build, benchmark, APK, Android device, renderer, Minecraft, Forge, multiplayer server or Aternos validation.
- The one-time authorization covers **2301–2400**. The 50-file section **2351–2400** is now 25/50 complete; the remaining checkpoint **2376–2400** is authorized. **STOP at 2400** and request fresh permission before 2401.
