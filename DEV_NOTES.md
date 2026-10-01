# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-10-01)_ — archive capacity and retrieval must be verified together

- D65's 64-file limit blocks ordinary rollover despite a small decoded archive. One immutable
  content-addressed window retains all 64 original full files; bounded manifests/catalog preserve
  logical addresses. Copy/verify/use/source reconstruction precede exact working-copy retirement.
- Python standard-library maintenance tool uses bounded decompression and safe in-memory record
  reads, no tar extraction/network/old-Git dependency. Fresh same-volume target materialization
  refuses symlinks, nested repositories and overwrite. Hook/CI checks committed window immutability.
- Original per-part/aggregate bounds still govern decoded records; compressed resident history and
  finite control/payload collections are counted independently. Compression cannot hide growth.
- Existing ledger probes consume materialized logical records. D68: original coverage mutation
  appended a declaration grep -m1 ignored and passed on unexempted D30. Replace the actual first
  declaration in part2 and require exactly that refusal with D30's exemption retained.
- Calibrated archive refusals, exact source reproof, strict Rust/WASM/book/full probes and staged
  gates validate the transition. HEAD immutability mutation awaits first transition commit;
  .19.2v owns observed remote verdict. No source/physical/release truth is inferred from a digest.
- promotion: promoted by `decision_history-windows-retain-self-contained-bytes.md`.

## _(2026-10-01)_ — documented procedure content lives in one canonical record

- Measurement metadata holds stable name/token/unit/kind and declaration/landmark/procedure ids.
  sc-measure depends on core, never the reverse. Value/state/source borrow the canonical declaration;
  documented procedure text lives once on its immutable referenced record. Nonblank text establishes
  content presence, not physical repeatability or source truth. Caller records invent no standard data.
- Borrowed current inventories reject within-/cross-kind identity collisions before lookup. Body and
  garment references cannot interchange. Repeated girth-level landmarks preserve authored intent;
  no distinct-endpoint or physical-domain rule is guessed. Separate target queries aid inspection,
  while full current validation checks every reference; Design/G4 still owns global source/evidence.
- Sixteen contracts plus three privacy docs pass. Six actual guard mutations fail; restored strict
  Rust executes 325 tests, with three-crate WASM, book and full probes/gates green. CI integration
  required the exceptional push; .4a.2c observed bf29b03 Rust/doctrine jobs and every step successful,
  including the three-crate WASM build. Metadata parent closes; physical/source proof remains deferred.
- D66 corrects README/workspace starter status. Older lessons and token task records move unchanged;
  canonical retrieval pointers retain exact identity. Archive is 64/64 files; the next required seal
  must take D65's owned SPINE.19.2 transition before further product growth.
- Promotion gate initially refused missing fresh questions. D67 traces an earlier false pass to an
  unrelated staged cleanup decline; SPINE.22 owns the scoped verifier, with fresh questions added now.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`'s metadata section.

## _(2026-10-01)_ — identifier syntax and binding authority are separate checks

- Core MachineToken is shared below metadata and recipes. It preserves ASCII lower-snake bytes,
  refusing malformed starts/segments, whitespace, Unicode lookalikes, uppercase and the three grammar
  keywords. Built-in parameter names remain valid references; metadata/recipe owners must separately
  refuse rebinding. Tokens provide no localized display label, text scalar or source-truth claim.
- Six contracts and a private-field doc-test pass. Four independent spelling/keyword mutations fail
  actual regressions; restored strict Rust executes 305 tests with WASM/book green. Grammar and book
  declare the same syntax, including digit-bearing segments; there is no normalization or auto-rename.
- Measurement metadata/runtime integration and observed-CI signoff are separate safe slices. The
  completed length-input contract/checklist moves unchanged to a bounded semantic sibling before
  parent pressure grows; the current checklist remains first. D66 landing-page status is owned by
  the runtime slice; D65 remains scheduled at its required-seal trigger, with history now 62/64 files.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`'s token section.

## _(2026-10-01)_ — numeric availability and source truth are separate contracts

- A shared core LengthDeclaration holds exactly one authored state and source. Known requires a
  nonempty distinct evidence inventory; assumed, unknown, preference and derived carry their distinct
  required record identities. Unknown and derived cannot store numeric values, and their queries name
  the observation or formula they require. Signed lengths and explicit zero remain authored input;
  procedure-specific physical domains belong to measurement/recipe validation, not a generic guard.
- sc-measure can borrow core declarations without copying state or introducing a core→measurement
  dependency cycle. A known claim is not proof that evidence exists or fits a scope. Design/recipe and
  G4 retain those checks; no global export approval is inferred from authored state.
- Eight contracts plus three privacy/state doc-tests pass; disabling empty/duplicate evidence checks
  or returning zero for unknown/derived each makes its regression fail. Strict Rust executes 298
  tests; WASM/book and focused censuses pass. D64 corrects stale ontology coverage prose; completed
  construction contracts and ten checklists partition unchanged with committed-payload comparison.
- promotion: promoted by `decision_length-declarations-retain-state-and-provenance.md`.

## _(2026-10-01)_ — a negative probe must prove it changed the intended contract field

- Full milestone probes found BAD-GATE anchored to an obsolete tuck/pleat explanation. Its sed
  replacement matched nothing, so a valid matrix reached the census and was accepted. Targeting the
  feature's gate cell avoids unrelated prose; independent post-mutation inspection proves exactly one
  invalid gate. Missing/duplicate targets refuse setup, distinct from the census's malformed-gate red.
  A no-op writer mutation now fails loudly as setup failure; the census itself is unchanged.
- Crate-scoped unit execution exposed a live count copied from the original whole workspace: 30
  meant 5 unit + 21 property + 1 doc in sc-units, plus three sc-core smoke tests. No tests disappeared;
  live counts now identify their scope. Historical delivered-workspace records stay unchanged.
- Sixteen §4 objects pass structural review: 149 object/support contracts, 260 sc-core tests including
  18 docs; full workspace 287 including 19 docs. Later geometry/recipe/profile/Design/release owners
  remain explicit. API/MCP workflow control requires observable contracts and independently checked
  artifacts; access alone is no measurement of garment expertise. This restates the canonical roadmap.
- promotion: declined (local fixture repair and re-verification of already canonical contracts).

## _(2026-10-01)_ — Pocket composition names physical copies and guards each source

- Component references pair a physical copy with its expected source Piece, retaining multiple copies
  of one pattern without duplicated geometry. Canonical component metadata is borrowed by copy id;
  current removal/reassignment cannot silently select another target. Same-id Piece replacement is
  inspected as current content. Served-copy membership alone proves no physical shape or recipe rule.
- Position follows shared birth/current anchor contracts; orientation requires complete owned ranges,
  not just live ends. Component contour repairs remain exposed through Piece range evidence and must
  be consumed by Design before execution/release. Opening Declaration/Profile preserves origin but
  reports DeferredToG3, separately from geometry/profile deferrals; no opening catalogue is invented.
- Eleven contracts and privacy pass; three independent guard mutations fail red. Strict Rust rejects
  unchecked fixture indexing, corrected before final checks. Restored WASM/book/censuses/gates pass.
  Director's SOTA/signoff/production-grade ruling remains the review bar, not an API-presence claim.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s Pocket boundary.

## _(2026-10-01)_ — buttonhole length retains a single canonical derivation source

- Button/hole pairs share current placement/count validation. The hole source borrows its owning
  Closure's button-size declaration and recipe operation; no second authored/cached physical length
  can drift. Replacement size/operation changes the observed source while old revision views remain
  unchanged. Current Design queries, not old views, must drive execution and approvals.
- Recipe/Design validates operation kind/dependency; G3 executes physical derivation. The ontology
  specifies that dependency but no physical formula or clearance, so G1 invents neither. DeferredToG3
  keeps that missing execution proof separate from geometric/profile validation and readable sources.
- Five new tests bring Closure contracts to fifteen; independent substitute-source mutations fail
  red. Compile-fail coverage refuses a separate length field; strict Rust/WASM/book pass. Source
  inspection remains available even while current placement repairs block execution.
- promotion: promoted by `decision_physical-cut-copies-have-stable-identities.md`'s button/hole source.

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/window1.md#stitchcad-devnotes-part1md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/window1.md#stitchcad-devnotes-part2md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/window1.md#stitchcad-devnotes-part3md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/window1.md#stitchcad-devnotes-part4md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/window1.md#stitchcad-devnotes-part5md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/window1.md#stitchcad-devnotes-part6md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/window1.md#stitchcad-devnotes-part7md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/window1.md#stitchcad-devnotes-part8md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/window1.md#stitchcad-devnotes-part9md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/window1.md#stitchcad-devnotes-part10md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/window1.md#stitchcad-devnotes-part11md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`devnotes-part12.md`](docs/history/window1.md#stitchcad-devnotes-part12md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`devnotes-part13.md`](docs/history/window1.md#stitchcad-devnotes-part13md) | injected identity lesson | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`devnotes-part14.md`](docs/history/window1.md#stitchcad-devnotes-part14md) | persistent-identity lesson | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`devnotes-part15.md`](docs/history/window1.md#stitchcad-devnotes-part15md) | structural-piece lesson | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`devnotes-part16.md`](docs/history/window1.md#stitchcad-devnotes-part16md) | interval-coverage lesson | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`devnotes-part17.md`](docs/history/window1.md#stitchcad-devnotes-part17md) | separate-pair-member lesson | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`devnotes-part18.md`](docs/history/window1.md#stitchcad-devnotes-part18md) | semantic-anchor/profile-binding lesson | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`devnotes-part19.md`](docs/history/window1.md#stitchcad-devnotes-part19md) | physical-copy identity lesson | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`devnotes-part20.md`](docs/history/window1.md#stitchcad-devnotes-part20md) | physical sewing-interval lesson | 15 lines, 1375 bytes, `sha256:34867dc9…` |

| [`devnotes-part21.md`](docs/history/window1.md#stitchcad-devnotes-part21md) | directed-grainline lesson | 13 lines, 1212 bytes, `sha256:c31c3298…` |

| [`devnotes-part22.md`](docs/history/window1.md#stitchcad-devnotes-part22md) | per-edge allowance lesson | 13 lines, 1192 bytes, `sha256:1807ae98…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

| [`devnotes-part23.md`](docs/history/window1.md#stitchcad-devnotes-part23md) | tuck/pleat and dart lessons | 25 lines, 2172 bytes, `sha256:ba5ee2a7…` |

| [`devnotes-part24.md`](docs/history/window1.md#stitchcad-devnotes-part24md) | canonical gather lesson | 13 lines, 1160 bytes, `sha256:2a10a04e…` |

| [`devnotes-part25.md`](docs/history/window1.md#stitchcad-devnotes-part25md) | Hem and served-layer lessons | 28 lines, 2416 bytes, `sha256:a95d8c77…` |

| [`devnotes-part26.md`](docs/history/stitchcad-devnotes-part26.md) | closure and notion-placement lessons | 27 lines, 2385 bytes, `sha256:b4b58e1b…` |
