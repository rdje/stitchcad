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
