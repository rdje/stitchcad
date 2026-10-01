# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

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

## _(2026-10-01)_ — closure counts derive from physical instances and current placements stay canonical

- Stable instance ids survive ordering; nonempty physical pairs derive one typed Count, without a
  separately authored quantity. Component placement reuse and ambiguous supplied ids refuse rather
  than inflate counts or select an arbitrary target. Zipper length is positive authored content or
  symbolic origin; hook/bar sizes retain logical declarations and no vendor/default size.
- Closure borrows current placements and validates every copy/Piece/anchor/direction context.
  Cached placement birth approval cannot certify an interior deletion or missing copy. Fly scope
  refuses before geometry with env_fly and request/gap/G7, preserving the declared envelope.
- Ten contracts, Count-boundary unit and privacy pass; four independent mutations fail red. Strict
  lint found a large nested target error; boxing that evidence keeps all typed refusals readable and
  compact. Restored Rust/WASM/book pass; physical hardware/size resolution remains later work.
- promotion: promoted by `decision_physical-cut-copies-have-stable-identities.md`'s closure instances.

## _(2026-10-01)_ — physical placement validation follows current material without changing identity

- A notion placement names a stable physical copy and retains its original source Piece binding.
  Reordering preserves identity; removal or source reassignment cannot transfer hardware to a peer.
  Reflection is separate from source-frame journal direction and supplies no inferred coordinates.
- Birth requires live uniquely owned anchoring; current validation follows historical references and
  preserves split choices/deletions as CurrentUnresolved evidence. The shared helper maps birth errors
  back to their existing variants, preserving Notch/TurnPoint contracts. A Piece id surviving an edit
  is insufficient: current resolved anchor and whole direction ownership must still be checked.
- Ten contracts + privacy pass. Four independent mutations fail red for copy binding, range ownership,
  historical resolution and anchor ownership; restored strict Rust/WASM/book pass. Raw queries are
  independent of registry/release approval. Hem checklist and verification history relocate unchanged.
- promotion: promoted by `decision_physical-cut-copies-have-stable-identities.md`'s notion placement.

## _(2026-10-01)_ — a faced hem needs current target validation, not cached birth approval

- Hem binds one stable Facing identity without copying its material or source definition. Current
  queries check target identity, served owner and source ranges again; an interior deletion can
  invalidate an old Facing while its original endpoints remain live. Same-id current replacement is
  inspected as current content. Independent edge and target queries do not certify release readiness.
- Depth and fold origins retain logical declarations and no symbolic defaults. Fold type, finishing
  method and allowance corner treatment are separate semantic facts. Recipe/Design owns declaration
  kinds/domains/states; G2/G3 owns physical folds, not the G1 reference validator.
- Ten contracts + privacy pass; disabling identity, current-layer and ownership checks separately
  produces red regressions. Restored strict Rust/WASM/book pass. Gather/layer evidence relocates
  unchanged with an independent committed-payload oracle and staged per-checklist revalidation.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s Hem boundary.

## _(2026-10-01)_ — modelled layer content and executable envelope are separate checks

- Facing/Lining/Interfacing share structural validation but retain distinct kinds. Their recipe offset
  operation owns dimensions; the descriptors retain directed source intent without caching generated
  contours or parameter states. Served identity, whole-source ownership and material reason are local
  invariants; operation/parameter/material registries and physical geometry remain later obligations.
- Feature-matrix rule 3 permits inspecting modelled lining. Its explicit execution check refuses
  env_lining with the served Piece and G7; Facing/Interfacing passing that check grants only scope,
  never geometric or release approval. Design validation must apply it before construction execution.
- Nine contracts and three privacy checks cover all kinds. Disabling lining scope and interval
  ownership independently produces red regressions; restored strict Rust/WASM/book pass. Piece shares
  its original material-reason guard and all existing contracts stay green. Fold evidence relocates
  unchanged and is revalidated by the staged gate.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s layer boundary.

## _(2026-10-01)_ — gather intent belongs to stable material and one canonical ease source

- A Gather names graph/span/side plus its original physical copy. Span-side retargeting to another copy
  is a typed binding change, even for the same Piece/range. Attachment and signed intake/allocation
  are borrowed from the canonical span; a second distribution would drift independently.
- Explicit A-minus-B sign fits the selected side; symbolic sign/value resolution and realized walking
  remain later obligations. Current-plan and current-graph queries are independent and neither grants
  release readiness. Design validation still checks every current registry and source-graph landmark.
- Eleven tests distinguish target changes, both signs, canonical allocations, interval repairs and
  immutable replacement. Disabling copy binding and interval ownership each makes its regression red;
  restored strict Rust/WASM/book pass. All intake kinds now have structural APIs, with physical closure
  and conservation visibly deferred to G2/G3.
- promotion: promoted by `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`'s gather binding.

## _(2026-10-01)_ — shared structural input must preserve distinct tuck and pleat kinds

- Tuck/Pleat wrappers share one immutable content validator but remain distinct semantic types for
  the recipe's typed closing operations. Their nonempty fold lists, intake origin and direction are
  authored intent; reference count alone cannot certify physical pleat shape or conserved intake.
- Duplicate held intervals ignore authored traversal; every line/direction checks complete owned
  current intervals and unique endpoints. Nine contracts exercise both wrappers, including an owned
  endpoints/foreign-middle merge after reversal. Disabling ownership refusal makes that regression
  red. Queries preserve raw split choices, fragment order and interior repairs without mutation.
- Symbolic intake/operation identities preserve registry obligations and provide no defaults.
  Physical fold shape and executed conservation remain G2/G3; strict Rust/WASM and book checks pass.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s tuck/pleat boundary.

## _(2026-10-01)_ — a declared dart intake is not an executed conserved closure

- An internal apex can be anchored to a Piece-owned construction leg without assigning it to the cut
  boundary. Directed legs/references carry intent; G2 checks actual coincidence/straightness, and G2/G3
  compares removed boundary length with declared intake after executing the named closing operation.
- Intake provenance and operation identity stay authored content. Symbolic sources carry no defaults
  or copied states. Nine tests cover birth refusals, duplicate legs, internal apex, split choices,
  current-frame ownership and immutable post-edit evidence. The foreign-middle mutation fails red;
  restored strict Rust/WASM pass. Registries still own operation/parameter existence and kinds.
- The remaining construction scope is split into safe semantic children; examples have a bounded book
  companion. No physical dart result or fixture golden is changed by carrying structural intent.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s dart boundary.

## _(2026-10-01)_ — allowance intent does not resolve a receiver policy or construct an offset

- Width origin and per-edge corner intent are canonical content. Explicit values retain the authored
  parameter identity; formula/profile sources retain declaration ids without copying uncertainty.
  Inclusion always names a logical profile declaration, so target profiles can resolve the same design
  differently. There is no unread-value fallback, global flag or generated contour in the G1 object.
- Eight contracts distinguish authored zero, negative refusal, symbolic width, all corner choices,
  shared width origins, immutable replacement and topology evidence. The foreign-middle merge case
  fails when whole-interval ownership refusal is disabled; restored strict Rust and WASM pass.
- The marks/allowances family is complete structurally, while G2 offsets/error bounds and G4 resolved
  policies remain named obligations. Older object checklists move unchanged to the existing evidence
  sibling before the parent exceeds 1000 lines; staged gates revalidate every moved checklist.
- promotion: promoted by `decision_profile-bindings-stay-symbolic-at-g1.md`'s allowance subsection.

## _(2026-10-01)_ — a directed arrow needs ordered intervals as well as endpoint identity

- A reversed authored range traverses split fragments in reverse order and composes its direction with
  journal reversal. Preserving raw range evidence keeps split choices and lost interiors visible while
  the directed view remains useful. Disabling the reverse iterator makes the order/repair regression red.
- Grain, alignment, stripe and plaid references each require complete owned intervals. The shared sewing
  ownership fold guards merged foreign remainders after reversal. Nine contracts distinguish optional
  fields, explicit bias/antiparallel intent and symbolic angles from actual straightness/angular proof.
- Ontology §10's executable body moves unchanged (231 lines / 18018 bytes, SHA256
  `20c5442203f3a39c33ace68a426ec48e6c8aaec0a6017eb4b998e9c0a511671c`) to its own chapter; the normative
  clauses stay in place. D61 corrects only a diagnostic about the census's global declaration scope;
  its ten probes pass. This is containment with preserved examples and predicates, not a policy change.
- promotion: promoted by `decision_ontology-invariants-structural-g1-geometric-g2.md`'s grain boundary.


# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |
| [`devnotes-part3.md`](docs/history/stitchcad-devnotes-part3.md) | two `2026-09-30` lessons (two tables, one garment; a fixture internally right) | 50 lines, 4723 bytes, `sha256:fcca661d…` |
| [`devnotes-part4.md`](docs/history/stitchcad-devnotes-part4.md) | two `2026-09-30` lessons (a blocked leaf splits; permission is no criterion) | 42 lines, 3706 bytes, `sha256:c2ac5791…` |
| [`devnotes-part5.md`](docs/history/stitchcad-devnotes-part5.md) | two `2026-09-30` lessons (a rule whose only path is "don't"; a digest is about bytes) | 43 lines, 3977 bytes, `sha256:859ce981…` |
| [`devnotes-part6.md`](docs/history/stitchcad-devnotes-part6.md) | two `2026-09-30` lessons (a spec's tables are its test suite; settle it with the artifact) | 55 lines, 5359 bytes, `sha256:129d50d8…` |
| [`devnotes-part7.md`](docs/history/stitchcad-devnotes-part7.md) | three `2026-09-30` lessons (an arm that removes the rule; a synthetic input is a fixture; a RED arm asserts the refusal) | 57 lines, 5120 bytes, `sha256:13fd6c73…` |
| [`devnotes-part8.md`](docs/history/stitchcad-devnotes-part8.md) | two `2026-09-30` lessons (source layout; i18n population) | 35 lines, 3196 bytes, `sha256:04ab285c…` |
| [`devnotes-part9.md`](docs/history/stitchcad-devnotes-part9.md) | the `2026-09-30` certifying-artifact lesson | 15 lines, 1343 bytes, `sha256:bc7fae65…` |
| [`devnotes-part10.md`](docs/history/stitchcad-devnotes-part10.md) | the `2026-09-30` shipped-work reconciliation lesson | 15 lines, 1380 bytes, `sha256:701d33f2…` |
| [`devnotes-part11.md`](docs/history/stitchcad-devnotes-part11.md) | the `2026-09-30` property-test framework lesson | 15 lines, 1384 bytes, `sha256:ae04eadf…` |
| [`devnotes-part12.md`](docs/history/stitchcad-devnotes-part12.md) | ontology slice decomposition | 16 lines, 1570 bytes, `sha256:a2f04e3d…` |
| [`devnotes-part13.md`](docs/history/stitchcad-devnotes-part13.md) | injected identity lesson | 18 lines, 1612 bytes, `sha256:38e83349…` |
| [`devnotes-part14.md`](docs/history/stitchcad-devnotes-part14.md) | persistent-identity lesson | 24 lines, 2230 bytes, `sha256:2b6aebd3…` |
| [`devnotes-part15.md`](docs/history/stitchcad-devnotes-part15.md) | structural-piece lesson | 15 lines, 1334 bytes, `sha256:1b362d26…` |
| [`devnotes-part16.md`](docs/history/stitchcad-devnotes-part16.md) | interval-coverage lesson | 13 lines, 1183 bytes, `sha256:fcf7c475…` |
| [`devnotes-part17.md`](docs/history/stitchcad-devnotes-part17.md) | separate-pair-member lesson | 12 lines, 1049 bytes, `sha256:140c4c41…` |
| [`devnotes-part18.md`](docs/history/stitchcad-devnotes-part18.md) | semantic-anchor/profile-binding lesson | 12 lines, 1102 bytes, `sha256:61a13500…` |
| [`devnotes-part19.md`](docs/history/stitchcad-devnotes-part19.md) | physical-copy identity lesson | 18 lines, 1663 bytes, `sha256:c0e3c442…` |
| [`devnotes-part20.md`](docs/history/stitchcad-devnotes-part20.md) | physical sewing-interval lesson | 15 lines, 1375 bytes, `sha256:34867dc9…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.
