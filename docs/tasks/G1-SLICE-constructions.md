# G1-SLICE — completed construction contracts and acceptance evidence

The bounded construction sibling of [G1-SLICE](G1-SLICE.md). Completed child contracts and
checklists retain their committed text unchanged; current measurement work remains in the parent.
Historical verification/commit/changelog tables stay in [the evidence sibling](G1-SLICE-evidence.md).

## Completed construction child contracts

- ID: `G1-SLICE.3c.4a`
  Status: `done`
  Goal: semantic intake constructions, owned anchors/direction and explicit closing operation.
  Children: `.3c.4a.1` (dart), `.3c.4a.2` (tuck/pleat/gather).

  Verification: all four semantic kinds have immutable structural APIs and deferred closure checks;
  dart/fold/gather tests + privacy, Rust/WASM/book and focused gates pass in child commits.
  Commit: children `STITCHCAD-G1-0014` … `STITCHCAD-G1-0016`.

- ID: `G1-SLICE.3c.4a.1`
  Status: `done`
  Goal: immutable Dart with intake origin, apex, two directed legs, declared direction and closing
  operation identity; actual closure/intake conservation remains G2/G3.
  Acceptance: required references are born-valid and owned; identical held leg intervals refused;
  nonnegative explicit intake retains its parameter origin, symbols supply no default; edits expose
  raw repairs/choices; no intake conservation or operation execution claim. Privacy and WASM pass.
  Design before code: apex is an EdgeAnchor on owned internal construction geometry, so an interior
  apex is expressible without pretending it lies on the cut boundary. G2 proves geometric leg/apex
  coincidence; two directed leg ranges and an explicit directed reference carry geometric intent.
  Intake has explicit parameter/value or formula/profile declarations; closing operation is held
  identity whose existence/kind/dependency is `.5`/`.6`'s obligation. Common reusable intake source
  stays distinct from allowance width. Extend the structural/geometric boundary record before code.
  Book containment: add a separate construction implementation companion before growing the current
  24135-byte executable chapter beyond its 24576-byte health target; normative clauses stay put.
  Verification: nine dart contracts + privacy; ownership mutation red; strict Rust, wasm, book,
  fixture/feature/glossary/tree censuses, ledger and staged doctrines green.
  Commit: `STITCHCAD-G1-0014`

- ID: `G1-SLICE.3c.4a.2`
  Status: `done`
  Goal: immutable Tuck/Pleat/Gather with explicit intake, owned fold/attachment geometry, direction
  and closing operation; type-specific semantic content and deferred physical conservation.
  Acceptance: validate all required scope references and intake domains, preserve repairs and defaults
  prohibition; G2/G3 must prove the actual closing operations and gather distribution.
  Verification: Tuck/Pleat/Gather contracts + privacy, Rust/WASM/book and focused gates pass.
  Commit: child slices `STITCHCAD-G1-0015`, `STITCHCAD-G1-0016`

  Children: `.3c.4a.2a` (tucks/pleats), `.3c.4a.2b` (gathers linked to a sewing span).

- ID: `G1-SLICE.3c.4a.2a`
  Status: `done`
  Goal: immutable, distinctly typed Tuck and Pleat with nonempty directed fold-line geometry,
  intake origin, explicit folding direction and closing-operation identity.
  Acceptance: all lines/direction are owned complete ranges with unique endpoints; empty or duplicate
  held fold intervals and negative explicit intake refused; symbolic intake supplies no default;
  queries preserve direction/repairs; physical fold shape/conserved intake remains G2/G3.
  Design before code: shared FoldDefinition input/validation, separate immutable Tuck/Pleat wrappers
  preserve semantic kind. A nonempty list supports one or multiple authored fold lines without
  inventing a physical pleat-count rule. Exact duplicate held intervals ignore traversal direction.
  Reuse IntakeAmount and whole-interval ownership; folding operation existence/kind is `.5`/`.6`.
  Broader actual geometry tests remain G2/G3, including coincidence between distinct held names.
  Extend the existing structural/geometric record before code. Live-window rollover is part of
  this slice's doc sync if the next entry crosses CHANGELOG/DEV_NOTES health targets.
  Verification: nine shared tuck/pleat contracts + two privacy doctests; ownership mutation red;
  strict Rust, wasm, book, focused censuses, ledger and staged doctrine gate green.
  Commit: `STITCHCAD-G1-0015`

- ID: `G1-SLICE.3c.4a.2b`
  Status: `done`
  Goal: Gather with intake origin, owned direction/attachment and explicit graph/span/side identity,
  using declared sewing ease allocation without a separate hidden distribution or physical stretch.
  Acceptance: named span/copy side exists and belongs to the Piece; all anchors/intervals resolve;
  no copied ease/profile state or claimed executed conservation; immutable queries expose removals.
  Verification: eleven gather contracts + privacy; copy-binding and ownership mutations red;
  strict Rust, wasm, book, focused censuses, ledger and staged doctrines green.
  Commit: `STITCHCAD-G1-0016`

  Design before code: Gather names graph/span/side and the held physical-copy id. Attachment range,
  signed intake source and distribution are borrowed from that span, not independently authored
  or cached. A-side gathering interprets A-minus-B directly; B-side gathering interprets its negation.
  An explicit differential with the wrong sign is refused; symbols remain G2/G3/G4 obligations.
  The raw signed source is exposed with its side, preserving provenance without invented values.
  Direction and closing operation are explicit; complete owned attachment/direction ranges required.
  Target queries report missing graph/span/copy or a changed side-copy binding without retargeting.
  Extend the existing sewing decision before code; retain graph distribution as the single source.
  Containment ownership: move recent completed mark/dart checklists unchanged to the existing evidence
  sibling before this checklist crosses 1000 lines; revalidate every staged checklist. Live-window
  rollover remains part of synchronized docs when health targets require it.

- ID: `G1-SLICE.3c.4b`
  Status: `done`
  Goal: Hem and Facing/Lining/Interfacing descriptors with served edges/Pieces, depth/fold method,
  offset relationship and material assignment; deferred/out-of-envelope constructions typed refusals.
  Acceptance: owned live references, positive composition links, explicit symbolic parameters and no
  generated offset claim; lining refuses its named deferred-envelope diagnostic in executable scope.
  Verification: served-layer + Hem contracts green; current composition and ownership mutations red.
  Commit: `STITCHCAD-G1-0018`

  Children: `.3c.4b.1` (served layers), `.3c.4b.2` (hem including faced-hem links).

- ID: `G1-SLICE.3c.4b.1`
  Status: `done`
  Goal: distinct immutable Facing/Lining/Interfacing descriptors with served Piece, explicit
  recipe offset relationship and material assignment. Modelled lining must refuse execution in v1.
  Acceptance: served identity matches the provided Piece, source ranges are nonempty/owned/complete
  with unique endpoints, unresolved material has a nonblank reason, raw repairs remain visible.
  Offset relationship carries operation identity + directed source ranges; recipe owns dimensions
  and resolves operation inputs, without duplicate uncertainty/default values here. Geometry is G2.
  Design before code: the canonical recipe is authoritative (ontology principle 1), so offset
  dimensions remain on the named recipe operation. Distinct layer types share structural validation;
  no separately generated contour or duplicated output-Piece material state is cached. Lining can be
  modelled structurally (feature matrix rule 3) but `require_in_scope()` returns `env_lining`, served
  Piece and proving gate G7. Facing/Interfacing pass only this envelope check, not geometry approval.
  `.6` must apply that check before requested construction execution, not before inspecting content.
  Share Piece's existing material-reason invariant and prove the original Piece contracts unchanged.
  Seal the oldest DEV_NOTES lesson unchanged if the synchronized entry crosses its health target.
  Extend the existing structural/geometric decision before code. Move completed fold evidence to the
  sibling before the parent's next checklist approaches 1000 lines; preserve and revalidate it.
  Verification: 9 layer contracts + 3 privacy tests; two refusal mutations red; restored checks green.
  Commit: `STITCHCAD-G1-0017`

- ID: `G1-SLICE.3c.4b.2`
  Status: `done`
  Goal: immutable Hem with owned finish edge, depth origin, explicit fold-type declaration and
  turned/faced method; faced hems bind an existing Facing serving the same Piece.
  Acceptance: all required references resolve, missing/wrong facing links are typed refusals,
  symbolic dimensions/fold type have no defaults; raw repairs stay visible; physical folding G2/G3.
  Design before code: depth is Explicit(parameter/value), Formula or Profile; explicit nonnegative
  zero is authored, never a fallback. Fold type names a logical declaration or Profile binding;
  recipe/Design owns its domain/state, so G1 invents no physical fold vocabulary. Method is Turned
  or Faced with a stable Facing id. Validate its id, same served Piece and current source ranges;
  later queries refuse removed/reassigned/invalid targets without retargeting. Whole finish-edge
  coverage and unique endpoints are required; raw interval repairs remain queryable. Registries
  validate parameter/fold kinds; G2/G3 prove physical folding. Extend the existing decision first.
  Move completed gather/layer checklists unchanged to the evidence sibling for bounded continuity;
  seal oldest live ledger/lesson entries unchanged if the synchronized docs cross health targets.
  Verification: served-layer + Hem contracts green; current composition and ownership mutations red.
  Commit: `STITCHCAD-G1-0018`

- ID: `G1-SLICE.3c.4c`
  Status: `done`
  Goal: Closure descriptors with button/buttonhole derivation, centred zipper and hook/bar placement,
  count and sizes; unsupported fly refusal. Derived buttonhole length cannot be independently entered.
  Acceptance: typed supported/deferred variants, required owned anchors and composition/count domains;
  no physical placement or resolved buttonhole-size claim before G2/G3.
  Verification: all closure child contracts green; canonical source mutations red; geometry remains deferred.
  Commit: `STITCHCAD-G1-0021`

  Children: `.3c.4c.1a` (physical notion placements), `.3c.4c.1b` (centred zipper/hook-bar/fly),
  `.3c.4c.2` (button/buttonhole pairs and one canonical length derivation).

- ID: `G1-SLICE.3c.4c.1a`
  Status: `done`
  Goal: immutable semantic placement of a notion on a stable physical CutCopy, with an owned anchor
  and directed orientation range, reusable by all closure components without duplicated geometry.
  Acceptance: missing copy, wrong/reassigned source Piece, invalid anchors and incomplete/foreign
  orientation ranges are typed refusals. Current queries never retarget to another copy.
  Design before code: stable placement id + physical-copy id + anchor/direction are authored input;
  private born Piece identity guards copy reassignment, without caching geometry or reflected coordinates.
  Birth anchors require live edges; current validation accepts uniquely resolved historical anchors,
  while retaining split choices/repairs. Extract the shared current-anchor validator behind the
  existing born-live guard so original Notch/TurnPoint behavior stays unchanged. Independent raw
  anchor/direction queries remain visible; Design validates all current copy/Piece registries.
  Extend the physical-copy decision before code; G2/V1 consumes reflection. Hardware/count/size
  descriptors follow in `.1b`/`.2`. Move completed Hem evidence unchanged to the existing sibling;
  bounded closure book examples and live ledger/lesson rollovers are owned synchronized docs.
  Move the completed verification-log table unchanged to the evidence sibling before the new checklist
  crosses 1000 lines; retain a direct retrieval pointer and fresh current verification log here.
  Verification: 10 placement contracts + privacy; four mutations red; restored focused checks green.
  Commit: `STITCHCAD-G1-0019`

- ID: `G1-SLICE.3c.4c.1b`
  Status: `done`
  Goal: centred zipper and hook/bar descriptors with physical notion-placement pairs, explicit sizes,
  counts derived from nonempty placements and named env_fly refusal. No physical placement claim.
  Acceptance: all placements revalidate current contexts; supported kinds stay distinct; empty/duplicate
  physical instances refused; no fallback sizes; fly refusal names requested closure/trousers gap/G7.
  Design before code: each ClosureInstance has its own stable id and two distinct current
  NotionPlacement ids. Count is derived as sc_units::Count from nonempty instances, never authored
  twice; overflow is typed. Reused instance/placement identities are refused; geometric coincidence
  remains G2. Centred zipper length is Explicit(parameter/positive Length), Formula or Profile;
  hook/bar sizes each name a logical recipe declaration or Profile binding, with no vendor default.
  Closure borrows canonical current placements and validates their Piece/plan/ledger contexts;
  duplicate context ids are typed refusals rather than first-match choices. Fly scope is checked
  before geometry and returns env_fly with requested Closure, declared trousers gap and gate G7.
  Extend the existing physical-copy decision first. Move completed placement evidence unchanged to
  the sibling before this checklist exceeds 1000 lines; live ledger/lesson rollovers owned here.
  Preserve the completed commit-log table unchanged in the sibling, with a direct retrieval pointer
  and a fresh current-slice commit table, so history growth does not displace active contracts.
  Verification: 10 closure contracts + Count domain + privacy; four mutations red; focused checks green.
  Commit: `STITCHCAD-G1-0020`

- ID: `G1-SLICE.3c.4c.2`
  Status: `done`
  Goal: button/buttonhole pairs with owned placements and button sizes, single canonical derived hole
  length source; no separately authored or cached hole length. Explicit profile/recipe provenance.
  Acceptance: required pairs/counts/targets validate; changing button size changes the source observed
  by derived holes; physical length resolution/derivation and placements remain G2/G3 obligations.
  Design before code: ButtonAndButtonhole is a distinct supported ClosureKind, using existing
  stable physical instance/placement pairs (first button, second hole). Button size retains a
  required NotionSize logical recipe/Profile declaration. ButtonholeDerivation names the recipe
  operation; there is no independently authored/cached hole length. buttonhole_length_source()
  borrows that same canonical size and derivation with Closure id; replacement size/operation
  changes the observed source. Recipe/Design must validate that typed operation consumes this size;
  G3 executes it and checks positive physical length, not an invented G1 formula or clearance.
  Separate DeferredToG3 derivation state from geometric DeferredToG2 and profile DeferredToG4.
  Extend the existing closure decision first; use existing current target/count validation for
  button/hole instances. Move completed zipper/hook-bar evidence and its completed log rows unchanged
  to the existing sibling before growing beyond 1000 lines; live ledger/lesson rollovers owned here.
  Move the completed task changelog unchanged to the sibling with a direct pointer, preserving
  current logs here and avoiding a repeatedly displaced active contract.
  Verification: all closure child contracts green; canonical source mutations red; geometry remains deferred.
  Commit: `STITCHCAD-G1-0021`

- ID: `G1-SLICE.3c.4d`
  Status: `done`
  Goal: Pocket position, orientation, opening and component Piece references; complete construction
  family signoff and close `.3c` after all child types have structural evidence and book coverage.
  Acceptance: every component exists, composition required, owned placement/direction references,
  symbolic opening resolution and execution scope remain G3 obligations; named envelope diagnostics
  must be consumed before execution, not replaced by approximation. All §4 families accounted for.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`

  Children: `.3c.4d.1` (Pocket metadata/composition), `.3c.4d.2` (construction/object-family signoff).

- ID: `G1-SLICE.3c.4d.1`
  Status: `done`
  Goal: immutable Pocket with served physical copy/source Piece, owned position and directed orientation,
  required logical opening binding and nonempty explicit component copy/Piece references.
  Acceptance: current targets are unambiguous/existing; copy/source mismatch, duplicate components,
  unresolved/foreign anchors and whole orientation intervals are typed refusals. No substituted pieces.
  Design before code: PocketPieceRef explicitly pairs copy id with source Piece id for served target
  and every component; multiple copies of one pattern Piece remain distinct. Source reassignment
  cannot silently satisfy the old reference. The served copy may also be an explicit component;
  G1 infers no recipe-cycle or physical-shape rule from that relationship. Recipe dependency cycles
  and physical pocket construction remain `.5`/G3. Opening is a logical recipe/Profile declaration,
  with DeferredToG3 resolution/scope rather than invented opening vocabulary or support approval.
  Birth/current anchor validation follows existing shared contracts; orientation requires complete
  owned intervals/unique endpoints. Component queries borrow current Piece metadata; Design/G2 must
  inspect all component contour repairs and geometry before execution/release. Extend the existing
  structural/geometric decision first. Bounded Pocket examples/API vocabulary and ledger/lesson
  rollovers are owned synchronized docs; completed evidence moves if containment requires it.
  Verification: eleven Pocket contracts + privacy; three mutations red; strict Rust/WASM/book,
  fixture/feature/glossary/tree, ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0022`

- ID: `G1-SLICE.3c.4d.2`
  Status: `done`
  Goal: re-derive all geometry-bearing object families against roadmap/ontology contracts, tests,
  immutable interfaces, current repair behavior, deferred obligations and mdBook implementation index.
  Acceptance: every child structurally complete with owned later obligations; full milestone checks/
  probes green; no geometric or unsupported-envelope support claim inferred from modelled metadata.
  Close `.4d`, `.3c.4`, `.3c` and top-level `.3` only after all structural criteria are verified.
  Director reaffirmed SOTA, signoff and production-grade as the bar on 2026-10-01. Structural
  completion is not production certification: geometry/interoperability/reliability and independent
  review gates retain their owned proofs; measured evidence, not API presence, earns signoff.
  Review plan before changes: inspect every ontology §4 object against immutable public interfaces,
  focused test inventory and bounded book examples; derive test populations from cargo output.
  Run full Rust/WASM/book and all 22 probe suites. Preserve completed acceptance evidence unchanged
  in the existing sibling before parent growth; review-only docs/module status and containment
  rollovers are owned here. Later recipe/profile/Design checks remain explicit release obligations.
  Signoff defects owned now: D62 — LIVE_STATUS attributes workspace-level 30 tests to sc-units
  (current 5 unit + 21 property + 1 doc = 27; original sc-core had three smoke tests). D63 —
  feature-matrix BAD-GATE mutation matches old tuck/pleat explanation, so it no longer changes the
  fixture and full probes report 11 pass / 1 fail. Fix row/cell targeting and assert fixture mutation
  before accepting red evidence; census predicate stays unchanged. Both are immediate priority.
  Director 2026-10-01: external agents must control an instance through MCP over the API and gain
  stitching competence through observable contracts/validation. Workflow discovery and full parity
  belong .6/.9 and G5; MCP control alone cannot certify expertise or production artifact correctness.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`

## Completed construction acceptance checklists

### `G1-SLICE.3c.4a.1` — semantic dart intent with visible closure obligations

- [x] **REPRODUCE / ISSUE** — ontology §4.3 and the reference skirt require semantic darts;
  `0c65d75` has no construction object. Intake declaration, geometric apex/legs and the executed
  closing operation must remain distinguishable so G1 cannot silently certify conservation.
- [x] **ROOT CAUSE (WHY + WHERE)** — a valid endpoint or declared intake is insufficient physical
  evidence. `cargo test -p sc-core --test dart_contract
  foreign_middle_of_merged_direction_is_refused_despite_owned_endpoints_after_reversal` → `1 passed`,
  `rc=0`: complete ledger coverage/live owned ends conceal a foreign middle, refused before/after
  reversal. `interior_apex_owned_legs_intake_origin_and_operation_identity_are_immutable_content`
  → `1 passed`, `rc=0`: an internal apex is valid structural intent with visible conservation deferral.
- [x] **FIX** — immutable Dart with intake origin, interior edge-anchored apex, two directed legs,
  explicit directed closing/folding reference and required closing-operation identity. Validate
  nonnegative authored intake, distinct held leg intervals, apex scope and all positive interval
  ownership/coverage/unique endpoints. Preserve raw topology evidence and operation/parameter
  registry obligations; expose G2 geometry and G2/G3 executed-conservation deferrals.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test dart_contract` → `9 passed`, `rc=0`;
  intake/symbolic values, opposite duplicate legs, each field's unknown/foreign scope, internal apex,
  split choice, foreign-middle merge/reversal, directed deletion evidence and immutable replacement
  discriminate. Disable ownership refusal → merged-middle test fails, `rc=101`; restored strict
  `make check` passes. The privacy doctest is green; queries never rewrite the original definition.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Existing marks/allowance/sewing contracts remain green and numerical goldens do not change.
- [x] **LOCKSTEP** — construction family decomposed into owned semantic children before code;
  structural/geometric decision extended, Rust/subsystem status, bounded construction book companion
  + SUMMARY/ontology index and feature coverage, frontier/evidence/logs, TASK_TREE/MEMORY/LIVE_STATUS,
  CHANGELOG and promoted DEV_NOTES. Three of four object families remain done; next `.3c.4a.2`.

### `G1-SLICE.3c.4a.2a` — distinct tuck/pleat intent with shared structural checks

- [x] **REPRODUCE / ISSUE** — ontology §4.3 requires semantic tucks and pleats, not line art;
  `8753290` has only dart intent. A fold-reference list and a declared intake cannot certify a
  physical fold shape, type-specific closing operation or conservation proof.
- [x] **ROOT CAUSE (WHY + WHERE)** — current interval ownership matters independently of endpoints.
  `cargo test -p sc-core --test fold_contract
  foreign_middle_of_a_merged_fold_line_is_refused_despite_owned_endpoints_after_reversal` → `1 passed`,
  `rc=0`: both Tuck/Pleat refuse the owned-ends/foreign-middle merge before/after reversal.
  Shared structural inputs must not collapse the two semantic kinds into one untyped recipe operation.
- [x] **FIX** — separate immutable wrappers share validated fold content: intake provenance,
  nonempty directed ranges, explicit direction and required closing-operation identity. Reject negative
  explicit intake, empty/duplicate held intervals, unresolved endpoints and unowned current portions.
  Query authored line order + directed raw evidence; retain typed physical/registry obligations.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test fold_contract` → `9 passed`, `rc=0`;
  both types discriminate empty/duplicate opposite lines, explicit/symbolic intake, all reference roles,
  split choices, hidden foreign interiors, directed repairs, partial ranges and immutable replacement.
  Ownership refusal disabled → merged-middle regression fails, `rc=101`; restored strict check passes.
  Two privacy doctests are green; no physical fold-count/shape or executed-conservation claim.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Earlier dart/mark/sewing contracts stay green; no numerical golden changes.
- [x] **LOCKSTEP** — `.3c.4a.2` decomposed before code; existing structural/geometric decision,
  Rust/subsystem status, bounded construction examples/API vocabulary/ontology index and feature row,
  task evidence/frontier/logs, TASK_TREE/MEMORY/LIVE_STATUS/CHANGELOG and promoted DEV_NOTES.
  Oldest changelog/dev-note entries seal to part18/part14. Next `.3c.4a.2b` gathers.

### `G1-SLICE.3c.4b.1` — served layers retain recipe and material intent with explicit lining scope

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires distinct served layers; feature-matrix rule 3
  separates modelled content from executable support. The prior commit has no layer descriptors.
- [x] **ROOT CAUSE (WHY + WHERE)** — a structurally valid Lining is still outside the v1 envelope.
  `cargo test -p sc-core --test layer_contract
  modelled_lining_refuses_v1_execution_with_named_diagnostic_piece_and_proving_gate` → `1 passed`,
  `rc=0`: inspection succeeds while execution returns env_lining, served Piece and G7.
  `foreign_middle_of_merged_offset_source_is_refused_despite_owned_ends_after_reversal` → `1 passed`,
  `rc=0`: live endpoints alone cannot establish whole-source ownership.
- [x] **FIX** — separate immutable Facing/Lining/Interfacing wrappers share owned-source and material
  validation. Offset operation/source intent stays authored; recipe owns dimensions and operation
  inputs. Explicit envelope checks distinguish inspection from execution; physical geometry is G2.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test layer_contract` → `9 passed`, `rc=0`:
  all kinds, materials, duplicates, wrong served Piece, split choices, hidden interiors, repair order
  and immutability. Three privacy doctests pass. Disabling lining scope and interval ownership
  separately makes each regression red, `rc=101`; restored checks pass. Moved fold checklist compares
  unchanged with `git show HEAD:docs/tasks/G1-SLICE.md`; existing Piece contracts remain green.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Fixture goldens stay unchanged.
- [x] **LOCKSTEP** — pre-code decision, `.6` envelope obligation, module/subsystem status, construction
  examples/API vocabulary, feature reasons, evidence relocation, frontier/logs/index and live memory
  are synchronized. The oldest interval lesson seals unchanged to devnotes-part16 with a verified
  digest. No geometric or release approval is implied. Next `.3c.4b.2` implements Hem.

### `G1-SLICE.3c.4a.2b` — gathers bind physical span sides and borrow canonical ease

- [x] **REPRODUCE / ISSUE** — feature matrix §5 defines gathering through seam ease distribution;
  `0cb97e4` has no Gather. An independent intake/allocation would duplicate the span's source, while
  binding only a span side would silently transfer gathering to a replacement physical copy.
- [x] **ROOT CAUSE (WHY + WHERE)** — span side and physical-copy identity are separate authored facts.
  `cargo test -p sc-core --test gather_contract
  wrong_graph_missing_span_and_changed_side_copy_are_refused_without_retargeting` → `1 passed`,
  `rc=0`: a changed A-side copy sharing the same Piece/range returns CopyBindingChanged.
  `uniform_weighted_and_between_notch_allocation_have_one_canonical_graph_source` → `1 passed`,
  `rc=0`: pointer equality proves each allocation view borrows the graph's existing declaration.
- [x] **FIX** — immutable graph/span/side/copy binding, explicit direction and closing operation;
  borrow attachment and signed intake/allocation from the canonical span. Validate explicit ease
  sign, copy owner and complete owned attachment/direction with unique endpoints. Target queries
  refuse missing/changed bindings; raw evidence and independent current-plan checks remain visible.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test gather_contract` → `11 passed`,
  `rc=0`: both gathered sides, symbolic sources, canonical allocations, changed/missing targets,
  copy reassignment, hidden merged interiors, split choices, repairs and immutable replacement.
  Copy-binding and ownership refusals disabled separately → their regressions fail, `rc=101`;
  restored `make check` passes. Privacy doctest green; three moved mark/dart checklists compare
  unchanged to committed originals. Four public intake structs re-derived from dart/fold/gather sources.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `0 mismatch(es)`;
  feature/glossary → `0 failure(s)`; tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger → `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Earlier sewing/intake/mark contracts stay green; no fixture golden changes.
- [x] **LOCKSTEP** — pre-code sewing decision extended, module/subsystem status, bounded construction
  examples/API vocabulary/ontology index and feature row, intake parents closed, evidence relocation,
  frontier/logs/index, MEMORY/LIVE_STATUS/CHANGELOG and promoted DEV_NOTES. Oldest lesson seals to
  devnotes-part15. Three of four object families remain done; next `.3c.4b` hem/layer descriptors.

### `G1-SLICE.3c.4b.2` — Hem retains depth/fold intent and validates current Facing targets

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires Hem depth, fold type, finish edge and turned/faced
  method; the prior commit has served layers but no Hem. Birth approval cannot certify later edits.
- [x] **ROOT CAUSE (WHY + WHERE)** — a Facing can lose interior source material while its original
  endpoints stay live. `cargo test -p sc-core --test hem_contract
  old_facing_is_revalidated_against_current_ledger_instead_of_reusing_birth_approval` → `1 passed`,
  `rc=0`: current queries and Hem birth both return InvalidFacing with the raw interior repair.
  `foreign_middle_of_finish_edge_is_refused_despite_owned_ends_after_reversal` → `1 passed`, `rc=0`:
  full live coverage still fails whole-source ownership, including after reversal.
- [x] **FIX** — immutable finish edge, explicit/symbolic depth, required logical fold binding and
  authored Turned/Faced method. Faced composition borrows the original current Facing after checking
  identity, served Piece and source validation. No duplicated layer data, solved values or defaults.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test hem_contract` → `10 passed`, `rc=0`:
  authored/symbolic depth and fold types, both methods, canonical target borrowing, missing/replaced/
  reassigned targets, stale interiors, foreign finish material, raw directions/repairs and immutability.
  Identity, current-layer and interval-ownership refusals disabled independently → each regression red,
  `rc=101`; restored strict checks and privacy pass. Gather/layer checklist relocation compares
  unchanged to `git show HEAD:docs/tasks/G1-SLICE.md`, with per-checklist staged revalidation.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Earlier layer/intake contracts pass.
- [x] **LOCKSTEP** — pre-code decision, Design composition/declaration obligations, module/map status,
  construction examples/API vocabulary, feature reason, `.4b` parent closure, frontier/logs/index and
  live docs align. Oldest entries seal unchanged to changelog-part19/devnotes-part17, with verified
  digests. Three of four
  object families remain done; physical folding remains G2/G3. Next `.3c.4c` closures.

### `G1-SLICE.3c.4c.1a` — physical notion placements preserve stable copy bindings

- [x] **REPRODUCE / ISSUE** — closure components attach to physical material; Piece-only positions
  cannot distinguish copies. The prior commit has CutCopy identities but no notion-placement API.
- [x] **ROOT CAUSE (WHY + WHERE)** — birth and current anchor validation have different contracts.
  `cargo test -p sc-core --test notion_contract
  current_validation_follows_historical_anchors_but_reports_current_choice_and_repairs` → `1 passed`,
  `rc=0`: historical anchors resolve while split choices/deletions remain current repair evidence.
  `copy_source_reassignment_and_wrong_provided_piece_are_typed_refusals` → `1 passed`, `rc=0`:
  the original copy id cannot silently adopt another source Piece.
- [x] **FIX** — immutable placement id/copy/anchor/direction plus original source identity guard.
  Born-live anchoring keeps existing errors; shared current validation follows journal ownership.
  Current copy/Piece checks and whole direction intervals refuse retargeting, lost/foreign material.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test notion_contract` → `10 passed`, `rc=0`:
  distinct copies, ordering/removal/reassignment, historical anchors, current ownership, choices,
  interior repairs, directed order and immutable input. Four mutations (copy binding, range ownership,
  historical resolution and current anchor ownership) independently fail red, `rc=101`; restored
  privacy/checks pass. Hem checklist and completed verification table compare unchanged to HEAD.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Notch/TurnPoint contracts unchanged.
- [x] **LOCKSTEP** — pre-code copy decision, closure children, current Design obligations, module/map,
  bounded closure chapter/API vocabulary and feature reasons, evidence/log retrieval, frontier/index
  and live docs align. The oldest lesson seals unchanged to devnotes-part18. No closure kind/count/size or
  physical transform claim yet; next `.3c.4c.1b`. G1 remains 3/4 object families.

### `G1-SLICE.3c.4c.1b` — zipper/hook-bar instances borrow current placements and refuse fly scope

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires closure kinds, placement, count and sizes;
  fixture §9 names a zipper and hook/bar. The prior commit only supplies notion placements.
- [x] **ROOT CAUSE (WHY + WHERE)** — current placements can lose interior material after birth.
  `cargo test -p sc-core --test closure_contract
  current_placement_repairs_are_revalidated_at_birth_all_target_checks_and_borrowed_queries` →
  `1 passed`, `rc=0`: birth/all-target/borrowed queries return the same current interval evidence.
  `fly_scope_refusal_precedes_geometry_and_names_requested_closure_gap_and_gate` → `1 passed`,
  `rc=0`: env_fly carries Closure/trousers gap/G7 even before any geometry context exists.
- [x] **FIX** — immutable stable instances and canonical placement ids; Count derives from nonempty
  instances. Distinct zipper/hook-bar size origins stay symbolic; no defaults. Current targets and
  unambiguous registry ids validate; fly scope refuses before geometry. Nested evidence is boxed.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test closure_contract` → `10 passed`, `rc=0`:
  kinds/sizes/counts/reordering, identity reuse, ambiguous/missing contexts, current repairs/removals,
  scope and immutability. Count boundary unit + privacy pass. Scope, reuse, length and current-target
  mutations each fail red, `rc=101`; restored strict checks pass. Placement checklist/commit table
  compare unchanged with `git show HEAD:docs/tasks/G1-SLICE.md`, with staged revalidation.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Earlier placement contracts pass.
- [x] **LOCKSTEP** — pre-code copy/instance decision, Design obligations, modules/map, closure examples/
  API vocabulary and feature reasons, bounded evidence/commit retrieval, frontier/index and live docs
  align. Oldest entries seal unchanged to changelog-part20/devnotes-part19. Hardware remains G2/G3; buttons
  follow `.3c.4c.2`. G1 remains 4/18 top-level leaves and 3/4 object families.

### `G1-SLICE.3c.4c.2` — buttonhole length has one canonical button/operation source

- [x] **REPRODUCE / ISSUE** — ontology §4.7 forbids entering buttonhole length twice. The prior Closure
  supports zipper/hook-bar, but has no button pair or canonical derived-hole source.
- [x] **ROOT CAUSE (WHY + WHERE)** — an operation id alone leaves the queried size source unspecified.
  `cargo test -p sc-core --test closure_contract
  buttonhole_length_source_borrows_the_canonical_button_size_and_derivation_without_defaulting` →
  `1 passed`, `rc=0`: pointer equality proves both fields borrow canonical authored declarations.
  `replacing_button_size_or_operation_changes_the_single_observed_hole_source` → `1 passed`, `rc=0`:
  replacement changes the observed source without mutating the old revision's view.
- [x] **FIX** — distinct ButtonAndButtonhole kind shares stable instance/count/target validation;
  required button-size binding and derivation operation have no separate hole-length input/cache.
  Read-only source view names Closure and borrows both inputs; actual derivation reports DeferredToG3.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test closure_contract` → `15 passed`, `rc=0`:
  five new contracts cover recipe/profile size origins, canonical borrowing/replacement, non-button
  absence, existing count/reuse/missing-target rules and current repair blocking. Button-size and
  operation sources substituted independently → regressions red, `rc=101`; restored strict/privacy
  checks pass. A compile-fail example proves a second length field is unavailable. Moved closure
  checklist and task changelog compare unchanged to HEAD, with staged revalidation.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; `make wasm` →
  green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree census → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`;
  staged `make gate` → `=== all doctrines green ===`, all `rc=0`. Existing closure kinds stay green.
- [x] **LOCKSTEP** — pre-code closure decision, typed recipe dependency obligation, module/map,
  closure examples/API vocabulary and feature row, parent closure, evidence/history retrieval and
  live frontier/index/docs align. Oldest entries seal unchanged to changelog-part21/devnotes-part20. No formula
  or default is invented; G3-GRADING.5 still owes executed derivation proof. Next `.3c.4d` pockets.

### `G1-SLICE.3c.4d.1` — Pocket retains physical composition and owned placement intent

- [x] **REPRODUCE / ISSUE** — ontology §4.7 requires Pocket placement, opening and component Pieces;
  the prior object families have no Pocket API. Explicit physical-copy identity applies to composition.
- [x] **ROOT CAUSE (WHY + WHERE)** — pattern-only references cannot distinguish multiple physical
  components or refuse reassignment. `cargo test -p sc-core --test pocket_contract
  physical_components_borrow_canonical_pattern_metadata_and_retain_all_deferred_proofs` → `1 passed`,
  `rc=0`: distinct copy references borrow the same canonical Piece. `orientation_cannot_hide_foreign_middle_geometry_behind_owned_live_endpoints`
  → `1 passed`, `rc=0`: valid ends cannot prove whole-range ownership, even after reversal.
- [x] **FIX** — immutable Pocket holds served copy/source guard, position/directed orientation,
  logical opening and nonempty unique component copy/source references. Current validation repeats
  target/placement checks; component queries borrow canonical metadata without copying geometry.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test pocket_contract` → `11 passed`, `rc=0`:
  composition, ambiguity/missing/reassigned targets, canonical metadata, historical point choices,
  interior loss/foreign geometry and updated same-id Piece ownership covered. Disabling copy-source,
  orientation-ownership and nonempty-component guards independently yields actual test failures,
  `rc=101`; restored suite and compile-fail privacy pass. Component repairs stay visible separately.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust + privacy green; restored `make wasm`
  → green; `make book` → warning-free; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`;
  tree → `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`; staged `make gate`
  → `=== all doctrines green ===`, all `rc=0`. Existing structural object families remain green.
- [x] **LOCKSTEP** — structural/geometric decision extended before code; recipe/Design opening and
  component-contour obligations named. API/module/map, Pocket examples/local vocabulary, feature
  row, tree/frontier/index/live docs align. Oldest changelog/lesson seal unchanged to part22/part21.
  Director's quality bar remains explicit in `.3c.4d.2`; physical scope/geometry are not approved here.

### `G1-SLICE.3c.4d.2` — structural object families reviewed against the production-grade bar

- [x] **REPRODUCE / ISSUE** — all individual §4 objects are delivered; parents still await a complete
  review. Full probes expose D63 (feature `11 pass / 1 fail`), and crate-scoped test counts expose D62.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core -- --list` → `260` tests including `18`
  docs, `rc=0`; fourteen object/support suites contain `149` contracts. Source/API review accounts
  for sixteen §4 objects in four families. `cargo test -p sc-units` → `5 + 21 + 1`, `rc=0`;
  `git show eb83f01:crates/sc-core/src/lib.rs` → three original smoke tests, explaining workspace 30.
  D63's literal old explanation matches no current tuck/pleat row; fixture unchanged, census green.
- [x] **FIX** — close structural object parents after review; retain all geometric, recipe/profile,
  current-registry and release obligations. Correct crate-scoped live counts (D62). D63 targets the
  feature's gate cell and independently proves one malformed gate; absent/duplicate targets refuse
  setup. Director's API/MCP control requirement belongs existing .6/.9/G5 parity/evaluation contracts.
- [x] **ADDRESSED (verified)** — `run_feature_matrix_probes.sh` → `14 pass / 0 fail`, `rc=0`:
  malformed gate refused by unchanged census; missing/duplicate target setup refused. No-op mutation
  of gate writer → `13 pass / 1 fail`, `rc=1`, naming fixture setup failure; restored suite passes.
  `make check` executes `287` tests including `19` docs, `rc=0`; all 149 object/support contracts pass.
  Completed Pocket/button checklists compare unchanged to HEAD and staged gates revalidate them.
- [x] **NO REGRESSION** — strict `make check`, `make wasm`, warning-free `make book`, full `make probes`
  → `22 suite(s) green`; fixture → `0 mismatch(es)`; feature/glossary → `0 failure(s)`; tree →
  `0 unowned / 0 orphan(s) / 0 dead link(s)`; ledger → `9 pass / 0 fail`; staged `make gate` →
  `=== all doctrines green ===`, all `rc=0`. No census predicate, diagnostic or envelope was weakened.
- [x] **LOCKSTEP** — object status/module/map, bounded book review and current-query limits,
  command/MCP requirement, parent closures, fresh frontier/index/live records align. D62/D63 close
  in the sealed defect ledger; 8 open / 54 sealed. G1 → 5/18 leaves, all four structural families;
  next .4 measurements/ease/sizes. G0 stays unapproved and production declaration remains G7.
  Review examples have a bounded ontology-review chapter; oldest allowance lesson seals unchanged
  to devnotes-part22. Completed historical checklists retain independent HEAD comparison evidence.
  promotion: declined (review re-verifies existing canonical decisions; D63 is a local fixture repair).

## Completed recipe byte records — preserved from d7a421e

## Complete recipe byte contract protocol

- ID: `G1-SLICE.5a.3f.1a`
  Status: `done`
  Goal: repair D109 exact-byte specification before complete recipe identity implementation.
  Clean predecessor eee15a8; no jobs/user edits. Read grammar4, contract, worked examples, existing
  D84/D95/D103 expression rules, statement/ordered APIs and governance6.1 delegated authority.
  Decide engineering bytes under the director's standing delegation; engineer authors/applies same
  decision and cannot approve its evidence. No new human/release/gate closure claimed.
  Retain existing (bind name kind expression); select flat (assert name tolerance left right),
  explicit (recipe statement...) envelope, empty (recipe), one space/no final newline. Names,
  declared kind/symbolic tolerance and authored operand/statement order remain identity-bearing.
  No sorting, duplicate merging, annotation inference, tolerance resolution or algebraic rewrite.
  Typed identity domains remain distinct; bytes do not claim project schema, hash namespace or
  canonical-input reader. Later store .7 must frame its typed fields/digest domains explicitly.
  Independently author statement and empty/whole ordered recipe byte fixtures; actual recursive
  book-reference parsing supplies every operand without infer/evaluate. Author-chunk recipe
  coverage remains distinct from an independent whole-recipe parser. Check all six bindable kinds/
  five tolerances plus aliases/raw angles/signs/calls/branches/order/empty/whitespace examples.
  Actual renderer opcode/name/annotation/operand/envelope/order/newline/empty faults must fail
  authored exact-byte assertions; exclusive finally restore exact tracked producer source.
  Standing structural suite watches producer/anchors/classifier; no product serializer proof claim.
  Book grammar + statement annex/index and a one-record technical decision expose exact bytes,
  implementation status, independent reviewer/unapproved evidence and principal's reversal path.
  Preserve closed subtree/closure/oldest ledger payloads exactly; shorten orientation input only
  as needed for new record within unchanged map ceiling. Scoped syntax/book/recording gates.
  Verification: actual reference16 statements/six kinds/five tolerances; nine whole sources/
  twelve ordered chunks/all four displayed examples; nine interpreter authored-byte assertion reds/
  exact restore. Focused syntax20/structural/language16/publication9 pass0. Product Rust source/test
  bytes stay exact eee15a8; original subtree/closure/oldest ledgers/report payloads preserved.
  Book51 chapters/21 APIs/1060 source/1640 rendered links; technical evidence unapproved.
  Commit: `STITCHCAD-G1-0068`.

### Complete recipe contract recording receipts

Ledger9/13 pointer controls, archive28/184 CLI controls/175 logical reads and retention175 records/
50 working Markdown/8712 decoded lines/673893 decoded bytes/330215 resident bytes pass0. Fresh
reconstruction yields10open/99unique sealed/zero overlap0. Tree10lanes/13trees/nine siblings/zero gaps;
glossary310/nine/158; feature105/29; uncertainty133/16; fixture20/four/five/zero mismatch all0.
Original D109 report and oldest G1-0046 changelog/G1-0066 lesson payloads compare exact eee15a8.
All scoped jobs observed terminal; final staged doctrine gate is required before commit.
Two actual reference witnesses serialize bind(width, length, 25 mm) identically to its binding
statement and recipe(bind(n, count, 1)) identically to its one-statement recipe. Typed field/digest
framing is therefore a concrete .7 acceptance obligation, not a new keyword or untyped hash claim.
Producer and watched structural rerun observed terminal rc=0 after all nine actual renderer reds;
source restoration remains exact. Product implementations are unchanged.
Staged make gate → === all doctrines green ===, rc=0; every scoped verification job observed terminal.

## Complete recipe byte closure — preserved during G1-0069

### `G1-SLICE.5a.3f.1a` — exact complete recipe byte contract

- [x] **REPRODUCE / ISSUE** — D109 exact assertion/empty/envelope bytes absent from grammar4;
  original report preserved byte-identically in sealed part40. Existing binding bytes stay fixed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `git diff -- docs/book/src/spec/formula-language/grammar.md`
  shows predecessor's explicit missing contract versus exact new4.1 templates/examples, rc=0.
- [x] **FIX** — retained bind, flat assert payload, ordered/empty recipe wrapper/single spaces/no
  final newline; decision discloses same-party author/applier, review/reversal and future typed .7 owner.
- [x] **ADDRESSED (verified)** — `recipe_byte_contract.py` actual reference syntax matches16 authored
  statements/nine whole sources/twelve chunks/all four published examples; `recipe_byte_mutations.py`
  nine actual authored-byte assertion reds/exact restoration, rc=0. No compiled product claim.
- [x] **NO REGRESSION** — focused `cargo test` recipe11/statement9:20 passed, rc=0; structural/
  language16/publication9 all rc=0. Every Rust source/test byte equals eee15a8; no behavior changed.
- [x] **LOCKSTEP** — normative grammar/annex/index and decision/ADR/current live records align;
  old subtree/closure/three sealed payloads exact; D109 technical gap closed, .1b normalization next.

## Verification Log

Pre-change protocol precedes instruments/book edits. Actual recursive reference syntax independently
renders16 authored statement rows (six kinds/five tolerances), nine complete sources/twelve chunks and
all four actual book/decision examples. Inference/evaluation trapped; no whole-reference-recipe parser
or compiled product serializer claim. Two actual ordinary-call byte collisions confirm the typed
identity boundary assigned to .7. Nine actual interpreter authored-byte assertion reds/exact
source restore and normal rerun observed terminal0; classifier anchors/noise controls watched.
Focused syntax20 (recipe11/statement9), structural suite/language16 and publication9 pass0:
51 chapters/21 APIs/1060 source/1640 rendered links. Rust source/test bytes compare exact eee15a8.
Ledger9 arms/13 pointers, archive28 arms/184 CLI controls/175 logical reads and retention175 records/
50 working Markdown/8712 decoded lines/673893 decoded bytes/330215 resident bytes pass0.
Fresh reconstructed defects10open/99unique sealed/zero overlap; tree10 lanes/13trees/nine siblings/
zero gaps, glossary310/nine/158, feature105/29, uncertainty133/16, fixture20/four/five pass0.
Predecessor complete syntax subtree/closure and three sealed payloads compare byte-identically0.
Map orientation shortened within unchanged ceiling; no unique maintained facts or ceilings removed.
Staged make gate → === all doctrines green ===, rc=0; independent evidence approval unapproved.

## Commit Log

| Leaf | Commit subject | Verification |
| --- | --- | --- |
| `.5a.3f.1a` | `STITCHCAD-G1-0068 (leaf G1-SLICE.5a.3f.1a): specify exact statement and ordered recipe identity bytes` | 16 statements/nine sources/nine actual reds |


Retention: complete predecessor payload preserved by .5b.2a.

## Completed product metadata receipts — preserved from cfd0748

## Product semantic vocabulary receipts — .5b.2a,2026-10-03 (UTC)

- New explicit metadata-only FormulaKind/Origin/ReservedName/ReservedContext; all8/six/9/8/four
  populations checked against independently authored rows and actual canonical tables. Tolerance
  roles reuse existing syntax variants. No provider, value/state availability or numeric conversion API.
- Five public contracts pass0; ten actual compiled mapping/population/context/role/lookup faults
  fail in Rust assertion bodies, rc101, and restore exact source. Standing structural suite verifies
  unique anchors and rejects compiler/expect-only/passing-name noise; normal restored tests pass0.
- Strict fmt/clippy/native make check:596 passes/49 result groups/0 failed, terminal0; WASM all
  three libraries compile0. Initial test-fixture tuple triggered type_complexity; owned/fixed here
  with named ReservedCase fields rather than a lint suppression. Final strict rerun passes.
- Full reference structural and language16/publication9 terminal0;55 chapters/29 scoped API rows/
  1135 source links/1762 rendered links, warning-free. No grammar or runtime result changed.
- D129 scoped current orientation repaired; former .1b/.1c frontier promises absent. Original
  seven-line/630B report retained exact in part52, digest645bed69; no history audit claimed.
- Exact predecessor task91lines/7051B/6cae1199 moved from recipes to constructions,41lines/3295B/
  24e36f3e checklists to measurements. Oldest ledger15lines/1243B/ffd1e428 and lesson12lines/1049B/
  de02f486 match d7a421e predecessor; existing archived content unchanged, no ceiling raised.
- Publication/tree/archive/ledger and staged gate recording receipts follow before commit.
  G1 stays5/18 In Progress. Next .5b.2b final sourced-declaration protocol before implementation.
- Final tree10lanes/13trees/10siblings/zero gaps and archive218 logical/32 resident records pass0.
  Fresh materialization yields10open/118sealed disjoint defect IDs; exact task/ledger predecessor
  comparisons pass0. Ledger9/0fail terminal0; final diff/fmt checks0. Staged doctrine gate follows.
- Final staged make gate:13 checks/all green, rc=0; the initial gate refused unformatted rc0
  evidence signatures. Actual output now uses rc=0/rc=101 inside all three required fresh boxes;
  no gate or signature policy changed. Diff check0; pre-commit hook repeats the staged gate.

Retention: complete predecessor payload preserved by .5b.2b.

## Completed product metadata checklist — preserved from cfd0748

### G1-SLICE.5b.2a — closed product semantic metadata

- [x] **REPRODUCE / ISSUE** — prior product API stops at syntax annotations; scoped declaration
  metadata foundation is next .5b.2. D129 stale orientation reproduced by exact current prose.
- [x] **ROOT CAUSE (WHY + WHERE)** — kind/binding, semantic origin/value provider and tolerance/
  size roles need distinct closed metadata. formula_semantic_contract →5 tests/0 failed, rc=0,
  compares independent8/six/9/8 populations with actual canonical table rows in both directions.
- [x] **FIX** — four metadata types preserve all normative roles without provider/value access.
  Reuse existing symbolic tolerances and preserve MachineToken/D124 current keyword set.
- [x] **ADDRESSED (verified)** — semantic_mutations.py →10 actual compiled body assertion reds,
  rc=101 per faulty build, producer restored exactly; script rc=0. Normal restored5 tests pass0.
  Every context/role mapping, geometry non-bindability, exact lookup and closed population checked.
- [x] **NO REGRESSION** — make check →596 native passes/49 groups, strict fmt/clippy rc=0; WASM3
  compile0; full structural/reference/language16/publication9 terminal rc=0. No runtime or grammar claim.
- [x] **LOCKSTEP** — metadata annex/examples/API map/status/navigation and README/live/task/
  resume agree;55 chapters/29 APIs/1135 source/1762 rendered links verified0. D129 fixed;
  exact predecessor task/ledger/lesson payloads retained in bounded parts, no caps raised.
  promotion: declined (existing kinds, origins, context and no-value-access principles).


Retention: complete predecessor payload preserved by .5b.2b.

## Completed product metadata protocol — preserved from cfd0748

- ID: `G1-SLICE.5b.2a`
  Status: `done`
  Work unit: `STITCHCAD-G1-0083`; clean predecessor d7a421e, handoff0/no jobs/user changes.
  Goal: closed product kind/origin/reserved-name/context vocabulary before sourced declarations.
  Read: roadmap ADR-0003/§11 G1, contract2/3/3.1/5/6, grammar1.1/5/6/7.1, ontology3/5,
  static obligation map, MachineToken/statement annotations and public input/reference interfaces.
  Protocol: eight kinds/six bindable kinds, nine origins/eight reserved names/four context classes;
  explicit enum metadata only, no values, state, numeric availability or context resolution.
  Reuse five symbolic FormulaToleranceName variants; preserve MachineToken's closed keywords.
  Public exhaustive populations/token lookup and exact conversions must agree bidirectionally
  with independently authored normative rows, including absent-context kind/origin information.
  Negative lookup preserves exact spelling: no trim/case repair or excluded-form reservations.
  Independent public Rust contracts and loaded-book population controls; actual compiled mapping/
  population/role faults must fail assertions with exact source restoration. Focused syntax/name,
  strict make check, WASM, reference/language/publication and staged doctrine checks before commit.
  Own D129 stale public review-owner prose repair and completed-task/oldest ledger/lesson retention
  within existing parts/ceilings. No sourced declaration, namespace, type checker or runtime claim.
  Acceptance: complete closed metadata contract, honest public status/examples and falsified controls.
  Verification:5 public contracts/10 actual compiled body reds/exact restoration; strict native596/
  49groups, WASM3, reference/language16/publication9 terminal0. Commit: `STITCHCAD-G1-0083`.

Retention: complete predecessor payload preserved by .5b.2b.

## Completed declaration protocol — preserved from db19b90

- ID: `G1-SLICE.5b.2b`
  Status: `done`
  Goal: immutable named declarations carrying existing canonical source identities/borrowed records,
  binding kinds and prior-operation PointRef/EdgeRef; preserve single authored-value ownership.
  Acceptance: no sc-core to sc-measure cycle or value/state fetch; authored input/recipe/reserved
  sources distinguished and impossible source-kind combinations refused by types or typed errors.
  Finalize public construction/ordinal/identity protocol after .2a, before code.
  Work unit: `STITCHCAD-G1-0084`; clean predecessor cfd0748, handoff0/no jobs/user edits.
  Pre-code protocol: same contract2/3/3.1/4.1/5.2/grammar1/7.1; complete normalized statement/
  recipe, MachineToken, LengthDeclaration and stable PointRef/EdgeRef interfaces read.
  Input origins closed to measurement/ease/parameter/profile/material. Carry input metadata and
  canonical declaration identities; generic input kind is a caller-authored metadata claim, not
  record/state/value proof. Canonical length adapter borrows the actual immutable record and forces
  length without reading state or authored_value. Geometry refs force point/edge and retain creator/tag.
  Recipe source comes only from an actual normalized recipe ordinal's let; absent/zero/assertion
  positions return None, no guessed ordinal, declaration or annotation. Preserve original name/
  whole-statement/name spans and authored annotation without inference; reserved binding still
  awaits namespace checks. Reserved constructors derive their fixed names/kinds/origins/contexts.
  Names borrow validated MachineToken or existing normalized source; private immutable construction,
  Copy/Clone and opaque Debug retain lifetimes/privacy, no copied canonical value/state.
  D130 draft adapter overgeneralization repaired before commit: generic kinds use only three
  scalar domains; measurement/Ease require canonical length borrowing, enforced by distinct types.
  Independently authored five length origins/three scalar domains/six kinds, every LengthState, every
  reserved name, exact refs, actual recipe positions and borrow/private compile-fail controls.
  Actual compiled source/kind/origin/name/ordinal/span/privacy faults must fail public body assertions
  and restore exact bytes. Strict native/WASM and focused reference/book checks; per-leaf docs/commit.
  Own exact completed-record/oldest ledger/lesson retention under unchanged parts/ceilings.
  Verification:7 public contracts/19 compiled body reds/one widened API-negative contract red,
  exact restore; strict native608/50groups and focused book/reference pass0. Commit: `STITCHCAD-G1-0084`.

## Completed declaration receipts — preserved from db19b90

## Sourced declaration receipts — .5b.2b,2026-10-03 (UTC)

- Closed named immutable source views: input metadata and canonical declaration IDs kept distinct;
  existing length records borrowed by pointer; geometry creator/tag preserved, no resolution.
  Recipe annotations/name/whole-name spans/ordinal come from actual normalized let positions only.
  No state/value query in kind/origin inspection; generic metadata does not certify target records.
- Seven public contracts and five actual negative construction/private/lifetime doctests verify
  five canonical length domains, three general scalar domains/six kinds, all five LengthStates,
  point/edge refs/eight reserved names, zero/absent/assertion recipe positions and boundary4096.
-19 actual compiled source/kind/origin/identity/context/privacy body assertion reds plus one
  actual widened library API that compiles forbidden measurement input and fails its negative
  construction contract; producer restored exactly, script rc=0. Normal strict608/50groups pass0.
- Owned draft faults: assert(false) placeholders refused by strict lint, replaced by actual variant
  assertions; custom messages then hid standard assertion signatures. Classifier correctly refused
  those reds; default body messages restored, classifier unchanged. D130 five-domain generic input
  was overbroad; distinct scalar type prevents measurement/Ease non-length metadata before commit.
  Borrow-discard fault retargeted to actual cloned-record cache after the new type boundary made
  its old replacement ill-typed; pointer identity catches the compiled clone, not compiler failure.
- Reference structural/language16/publication9 terminal0:55 chapters/33 APIs/1137 source links/
  1765 rendered links, warning-free. Reference evidence remains separate from product acceptance.
- Exact cfd0748 task26lines/2390B/29b1b16a receipts,21lines/1781B/288041bd protocol and20lines/
  1601B/04dd4b10 checklist retained in constructions. Oldest ledger11lines/974B/aad9b836 and
  lesson11lines/932B/2d6e66b8 match HEAD; D130 eight-line/772B report5f6b8b11 retained in part53.
- Archive221 logical/35 resident records and fresh reconstruction10open/119sealed/disjoint pass0;
  tree10lanes/13trees/10siblings/zero gaps. Ledger/WASM/staged-gate final receipts follow.
- WASM all three libraries compile, rc=0; native608/50groups includes all five construction/
  privacy/lifetime negative contracts. Source/state/availability methods were not used for kind.
  Final tree/archive/defect predecessor checks pass0; ledger9/13 pointer controls terminal0.
  Staged doctrine gate follows; all current verification jobs observed terminal.
- Staged make gate:13 checks/all green, rc=0; final diff check0. Metadata/source predicates and
  closed API type boundary remain restored; pre-commit hook repeats the staged gate.

## Completed declaration checklist — preserved from db19b90

### G1-SLICE.5b.2b — immutable sourced declarations

- [x] **REPRODUCE / ISSUE** — initial seven public draft contracts accept generic non-length
  measurement/Ease metadata (D130), contradicting canonical length-target contracts; rc=0.
- [x] **ROOT CAUSE (WHY + WHERE)** — generic origin/kind matrix overgeneralizes reference metadata
  into canonical adapters. Current formula_declaration_contract →7 tests/0 failed, rc=0; five
  negative construction/lifetime/private doctests pass in make check. Widening the actual scalar
  API makes the forbidden measurement call compile and its negative contract fail, rc=101.
- [x] **FIX** — distinct five length/three general scalar source domains, borrowed canonical
  length records and typed refs; recipe metadata comes only from actual normalized let positions.
- [x] **ADDRESSED (verified)** — declaration_mutations.py →19 actual compiled body assertion
  reds plus one actual widened negative construction contract red; exact source restored, rc=0.
  Seven normal public contracts preserve all identities/origins/kinds/refs/ordinals/spans/privacy.
- [x] **NO REGRESSION** — strict make check →608 passes/50 groups, rc=0; full reference/structural,
  language16/publication9 pass, rc=0. WASM and final recording-gate receipts follow in recipes.
- [x] **LOCKSTEP** — declaration annex/source limits/examples/33 API rows, README and live/task/
  resume records align;55 chapters/1137 source/1765 rendered links verified, rc=0. D130 fixed
  before commit; exact prior task/ledger/lesson/report payloads retained, no ceilings raised.
  promotion: declined (canonical single-source ownership, typed metadata and original-context rules).

## Completed reserved diagnostic proposal protocol — preserved from 862c0d9

- ID: `G1-SLICE.5b.2c.1a`
  Status: `done`
  Goal: reproduce the reserved-name diagnostic mismatch before choosing product error arguments.
  Work unit: `STITCHCAD-G1-0085`; clean predecessor db19b90, no pending jobs or user edits.
  Protocol: compare contract3.1/5.2, static annex and actual reference namespace/static_statement
  entry points. Record real name/origin/message/arguments; no invented statement index. Propose a
  source-aware formula_rebinding schema, retain the closed tokens and current grammar. Obtain the
  director's decision before changing canonical semantics or implementing the blocked namespace.
  Own focused reproducibility tool/negative controls, book/live/task alignment and exact completed
  task/oldest ledger/lesson retention within existing ceilings. Product Rust remains unchanged.
  Acceptance: durable reproduction, explicit proposed required arguments and the precise decision
  blocker; complete documentation/reproduction leaf committed before handing off the ruling.
  Verification:121 actual cases/three compiled body assertion reds; source unchanged; focused
  structural/language/publication/ledger/archive checks pass0. Commit: `STITCHCAD-G1-0085`.

## Completed reserved diagnostic proposal receipts — preserved from 862c0d9

## Reserved diagnostic proposal receipts — .5b.2c.1a,2026-10-03 (UTC)

- reserved_diagnostic_review.py --mutations →120 reserved refusals/one ordinary rebinding/three
  actual compiled token/fabricated-index assertion reds, rc=0. Prefix source unchanged; no product
  diagnostic or repaired-argument acceptance. Canonical row/actual empty dictionaries reproduce D131.
- Full structural runner including new watched producer passes0; language16/publication9/ledger9
  and13 actual pointer controls pass0. Warning-free book55 chapters/33 APIs/1138 source/1767
  rendered links. git diff HEAD -- crates Cargo.toml Cargo.lock →empty, rc=0; no Rust change.
- Exact db19b90 declaration protocol29lines/2647B/7ea9f42a, receipts31lines/2836B/c1fc8b5c,
  checklist19lines/1700B/91e75ce2 retained in constructions; original routes remain live.
  Oldest ledger12lines/1035B/451e0450 and lesson13lines/1139B/c79e3803 sealed byte-exact to HEAD.
- Archive verify-retention/materialize →223 logical/37 resident records, rc=0; fresh canonical
  defect census11open/119unique sealed/no overlap, rc=0. Tree10lanes/13trees/10siblings/zero gaps.
  No existing sealed payload, registry cap, token, grammar or product implementation changed.
- D131 owned at .2c.1b for director's concrete schema ruling/repair, then .2c.2 namespace code.
  Proposal/reproduction .1a complete; neither unresolved defect nor product namespace marked done.
  Final staged doctrine receipt follows; all focused jobs observed terminal0.
- Staged make gate →13 checks/all green, rc=0. Final publication after heading placement still
  55/33/1138/1767 and9 pass/0 fail, rc=0; diff check0. Pre-commit repeats the staged doctrine gate.

## Completed reserved diagnostic proposal checklist — preserved from 862c0d9

### G1-SLICE.5b.2c.1a — reserved diagnostic conflict reproduction/proposal

- [x] **REPRODUCE / ISSUE** — reserved_diagnostic_review.py --mutations →120 reserved refusals/
  one ordinary rebinding/three actual assertion reds, rc=0. Canonical row requires two indices;
  actual eps_num initial/let refusals expose formula_rebinding and empty arguments.
- [x] **ROOT CAUSE (WHY + WHERE)** — contract3.1 reserved prohibition and static reference guards
  use rebinding, but contract5.2 raised-when/required-arguments row only describes repeated lets.
  Actual reproduction120/one and three compiled token/fabricated-index body reds, rc=0. Reserved
  sources have no prior statement; initial declarations have no attempt statement to index.
- [x] **ADDRESSED (verified)** — reproduction/proposal leaf only: actual121 cases/three assertion
  reds and unchanged producer bytes, rc=0. D131 remains open, owned and scheduled at .1b after
  director ruling; no schema change, invented context, namespace implementation or defect closure.
- [x] **NO REGRESSION** — full structural runner including121/three new controls, language16,
  publication9 and ledger9/13 pointer controls pass, rc=0. Book55 chapters/33 APIs/1138 source/
  1767 rendered links; archive223 logical/37 resident records and Rust diff unchanged, rc=0.
  Final staged doctrine receipt follows in recipes before commit.
- [x] **LOCKSTEP** — book marks argument-proof boundary and pending proposal; task/ADR/live/resume
  route .1b next. Prior task/ledger/lesson payloads retained exactly, no ceiling raised.
  promotion: declined (pending director decision; existing truthful-context doctrine).

## Completed static signature checklist — preserved from 862c0d9

### `G1-SLICE.5b.1a` — complete finite static signature evidence

- [x] **REPRODUCE / ISSUE** — direct actual reference baseline →min/max(length) refuse
  formula_dimension; within(..., size_index/size_count/is_base_size) accepts boolean, rc=0.
  New static_signature_contract.py first run →actual min(v_length) body assertion red, rc=1.
- [x] **ROOT CAUSE (WHY + WHERE)** — infer_call admits all reserved names while _matches skips
  tolerance kind; variadic _matches imposes max(2, listed arity). Actual prefix/call-path review
  and direct baseline rc=0 pinpoint those guards against grammar5/6 and ADR five-class rule.
  static_signature_contract.py --mutations →12 actual predicate faults fail body assertions, rc=0.
- [x] **FIX** — within guard admits exactly five tolerance names; min/max use the documented
  one-kind minimum. Independent hardcoded dimensional tables compare actual loaded populations
  in both directions; signature matrix traps environmental reads and execution callbacks.
- [x] **ADDRESSED (verified)** — static_signature_contract.py --mutations →4032 actual
  parse/infer cases,22 closed names,12 compiled actual predicate/body assertion reds, rc=0.
  Each faulty guard fails the assertion; producer on disk unchanged. D112/D113 close; no new
  grammar/identity/storage or product type/evaluation implementation claim.
- [x] **NO REGRESSION** — run_formula_structure_probes.sh →all existing structural/input/
  canonical/numeric/replay controls plus4032/12 green; language probes →16 pass/0 fail, rc=0.
  Publication →53 chapters/25 APIs/1102 source/1705 rendered links,9 pass/0 fail; bash -n
  on both shell producers →rc=0. Focused cargo test -p sc-core expression/canonical/recipe
  contracts →34 pass/3 groups; staged make gate →all doctrines green, rc=0.
- [x] **LOCKSTEP** — expert annex/grammar/type contract/index/status, complete .1a–.1c ownership,
  defect closure/history/live/resume agree; Rust sources/tests unchanged. G1 stays5/18;
  canonical defect marker census →10 open/102 unique sealed/0 intersection, rc=0. First scratch
  census incorrectly counted task headings; canonical markers corrected the query and draft
  defect markers before final sealing. Original report bodies/oldest ledger payloads retained.
  promotion: declined (already canonical independent-evidence doctrine; no new durable policy).

Final staged gate:all doctrines green, rc=0. Ledger:nine arms/thirteen pointer controls pass, rc=0.
Archive CLI:198 controls/189 full logical reads, rc=0; retention189 logical records/64 working
Markdown/9092 decoded lines/700909 decodedB/357231 residentB, rc=0. Prior main milestone checklist
1474B and recipe completed prefix47500B retained exactly; oldest live ledger payloads exact HEAD.
The first gate selected the primary tree's historical checklist lacking a recognized result shape;
added current owning .1a evidence before it, leaving historical bytes unchanged. Matrix guard review
replaced a kind-only dict with Mapping so get(value) cannot silently default; actual twelfth fault
proves the read trap. Draft count replacement was corrected for D112/D113 and1102 link identities
before final checks. No checker, cap, grammar or product Rust changed; no needed job remains live.
