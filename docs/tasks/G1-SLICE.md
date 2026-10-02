# G1-SLICE: executable architecture slice (roadmap gate G1)

## Metadata

- Tree ID: `G1-SLICE`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §11 gate **G1 — Executable architecture slice** (sources: §4
  architecture, §5 ADR-0001/0002, §6 constraint machinery, §7.3 runtime profiles, §7.8 API+MCP,
  §10 security)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

The architecture exists as **running code on all three runtime profiles**: the ontology and the
construction recipe evaluate, one CSP constraint resolves, a project persists and reopens, the
command bus mediates every mutation with revision checks and undo, a real browser executes the
slice end to end, and the canvas-hosting question is settled by measured evidence rather than
argument.

G1 is the first lane that lands product code, so it owns the workspace shape: the bedrock
starter crate is retired and the roadmap §4.3 crate layout appears (defect D10).

## Entry criteria

- `G0-CONTRACT` closed (or each leaf below cites the G0 spec chapter it implements; a leaf may
  not start before its spec exists — code without a contract is a guess with a build step).
- `SPINE.8` landed: the acceptance-evidence gate is sound for multi-leaf tree files, because
  every code commit from here on is judged by it (defect D15).

## Non-Goals

- The 2D correctness proof (G2), grading (G3), Factory Profiles (G4), shipped UX (G5).
- The production Profile Editor, exporters beyond the one needed to prove the browser spike's
  `export` step, or any 3D/mesh work (V1).
- Claiming WASM parity: `wasm-viewer` is a documented capability subset (§7.3), and the spike's
  output is a capability matrix with tested versions and limitations, not a parity claim.

## Acceptance Criteria (gate G1 exit, clause by clause)

| Roadmap G1 exit clause | Leaf |
| --- | --- |
| three runtime profiles compile | `.11` |
| intended browser target runs a real (not compile-only) spike: load → evaluate recipe → one CSP constraint → persist/reopen → render → export | `.12` |
| canvas-hosting spike evidence for ADR-0002 | `.13` |
| command bus live with revision checks + undo | `.6` |
| crash-consistent save/recovery demonstrated | `.7` |
| license-compatible solver builds on all targets | `.15` |
| WASM evidence: capability matrix with tested versions + limitations | `.12` |

## Task Tree

- ID: `G1-SLICE`
  Status: `active`
  Goal: the architecture slice runs on native and in a browser, with the command bus and
  persistence as the only mutation path.
  Children: `.1`, `.2`, `.3a`/`.3b`/`.3c`, `.4` … `.16` (18 leaves; `.3` was decomposed `2026-09-30`)

- ID: `G1-SLICE.1`
  Status: `done`
  Goal: workspace shape per §4.3 — retire the bedrock starter crate (`crates/app`, defect D10),
  create `sc-units` and `sc-core` with the workspace lints inherited, and extend CI to the G0
  workflow shape (fmt / clippy / unit+property / WASM smoketest).
  Acceptance: `cargo metadata` lists the roadmap crates that exist so far; no crate prints the
  template message; CI green on the new layout; the Knowledge Map names the subsystems.
  Verification: recorded below — delivered by `G0-CONTRACT.18` (commit `eb83f01`) ahead of this
  leaf; every acceptance criterion re-derived by command in the `### G1-SLICE.1` checklist.
  Commit: `STITCHCAD-G1-0001`

- ID: `G1-SLICE.2`
  Status: `done`
  Goal: `sc-units` — fixed-point micrometre quantities (i64), the five tolerance classes as
  distinct types, unit conversions with explicit rounding, rejection of non-finite and
  dimensionally invalid expressions (implements the `G0-CONTRACT.2` spec).
  Acceptance: property tests for conversion round-trips and tolerance-class separation; a
  dimension error is a typed error, never a silent coercion; compiles for `wasm32-unknown-unknown`.
  Verification: recorded below — `sc-units` was delivered in full by `G0-CONTRACT.18` (commit
  `eb83f01`); every acceptance criterion re-derived by command in the `### G1-SLICE.2` checklist,
  and the property-test-framework choice recorded as a layer-C decision.
  Commit: `STITCHCAD-G1-0002`

- ID: `G1-SLICE.3a`
  Status: `done`
  Goal: the identity layer in `sc-core` (ontology §1) — `EntityId` (a dependency-free ULID), the
  injected `IdGenerator`, `EdgeRef`/`PointRef` (the entity id of the creating operation plus a
  persistent local tag), and the bounded exact rational parameter `t` in `[0, 1]`.
  Acceptance: an `EntityId` round-trips its 26-character Crockford form and orders lexicographically
  by creation; a deterministic `IdGenerator` reproduces ids byte-for-byte (the replay property);
  the parameter is exact under `+ − × ÷` with reduction, carries a total order and an
  `in_unit_interval` predicate, and reports overflow / division-by-zero as typed diagnostics; the
  crate still compiles for `wasm32-unknown-unknown`.
  Verification: recorded below — 40 tests green (31 unit, 9 property), `make wasm` cross-builds
  `sc-core`, every criterion re-derived in the `### G1-SLICE.3a` checklist.
  Commit: `STITCHCAD-G1-0004`
  Design: `decision_entity-identity-ulid-injected-generator.md`,
  `decision_edge-parameter-bounded-exact-rational.md`.

- ID: `G1-SLICE.3b`
  Status: `done`
  Goal: the persistent-identity contract (ontology §1.1) — reference resolution under split, merge,
  reverse, delete and offset-fragmentation, and the `RepairTask` an orphaned reference becomes. No
  silent reassignment.
  Acceptance: reference stability is a tested property under split/merge/reverse — a reference at the
  split point resolves to both fragments and the consumer states which it wants, merge recomputes the
  parameter by arc length, reverse maps `t` to `1 − t`; a deleted edge's references become visible
  `RepairTask`s naming the reference, the orphaning edit and the candidate resolutions; a design with
  unresolved references is savable and inspectable but cannot be released.
  Verification: recorded below — 62 unit + 8 contract-property + 9 identity-property tests green,
  `make wasm` cross-builds `sc-core`, every criterion re-derived in the `### G1-SLICE.3b` checklist.
  Commit: `STITCHCAD-G1-0005`
  Design: `decision_reference-resolution-journal-fold.md` (recorded before the code, per the `.3`
  decomposition's discipline), inheriting `decision_entity-identity-ulid-injected-generator.md` and
  `decision_edge-parameter-bounded-exact-rational.md`.

- ID: `G1-SLICE.3c`
  Status: `done`
  Goal: the geometry-bearing object types (ontology §4) — `Piece`, `SeamSpan`/`SewingGraph`, `Notch`,
  `Grainline`, `SeamAllowance`, `Dart`/`Tuck`/`Pleat`/`Gather`, `Closure`, `Pocket` — with their
  structural invariants enforced at construction.
  Acceptance: a structurally invalid object cannot be built and the diagnostic names the invariant
  (an empty or self-repeating boundary loop, a reference to a non-existent edge, multiplicity 0, a
  cut-on-fold piece without exactly one fold edge, incomplete label data); the GEOMETRIC invariants
  (CCW winding, simplicity, holes strictly inside, closure, dart-intake conservation) are carried as a
  visible `DeferredToG2` state discharged by `G2-2D.1`, never claimed here.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`
  Design: `decision_ontology-invariants-structural-g1-geometric-g2.md`.
  Children: `.3c.1` (pieces), `.3c.2` (sewing graph), `.3c.3` (marks and allowances),
  `.3c.4` (garment constructions). The parent closes only after all four children.

- ID: `G1-SLICE.3c.1`
  Status: `done`
  Goal: immutable `Piece` content, directed cyclic edge loops, complete printed labels, explicit
  material assignment or unresolved state, and visible `GeometricValidation::DeferredToG2`.
  Signoff finding (owned here): `make book` warns that release §6's literal `<receiver>` and
  `<version>` scope placeholders are interpreted as unclosed HTML tags. Their rendered content is
  hidden instead of showing the intended scope example. Reproduce in `target/piece-book.log`;
  fix by code-formatting the literal placeholders and verify the rendered HTML escapes them.
  Acceptance: empty/repeated loops, missing edges, zero multiplicity, invalid fold declarations and
  incomplete labels are typed refusals; no public mutation bypasses validation; identity-ledger edits
  do not rewrite the piece's references and endpoint resolution exposes orphaned references; geometric
  winding, simplicity, containment and endpoint closure remain explicitly deferred. Rust checks, the
  wasm cross-build and mdBook build pass.
  Verification: 12 contract tests, privacy compile-fail test, `make check`, `make wasm`,
  warning-free `make book`, feature/release censuses and staged doctrine gate green.
  Commit: `STITCHCAD-G1-0006`

- ID: `G1-SLICE.3c.1a`
  Status: `done`
  Goal: fix D56 — the shipped piece cut plan cannot represent the reference skirt's separate
  cut-once L/R back members. Add explicit handedness and companion-piece identity while retaining
  the existing total-quantity mirrored-pair mode. Printed labels derive the member's L/R information.
  Acceptance: cut-once left/right pieces can carry reciprocal companion identities; missing hand or
  companion is unrepresentable, self-companion is refused, existing even-quantity mirrored pairs
  remain valid and odd totals in that mode remain refused. No geometric mirror claim is invented;
  companion existence/reciprocity is a design-collection obligation at `.6`.
  Verification: 14 piece-contract tests, strict `make check`, wasm, book, fixture/feature censuses,
  ledger probes and staged doctrine gate green. D56 closed in defects-part3.
  Commit: `STITCHCAD-G1-0008`
  Design: `decision_piece-pair-members-have-explicit-handedness.md`.

- ID: `G1-SLICE.3c.2`
  Status: `done`
  Goal: first discharge D55 with full-range resolution/repair (endpoints cannot certify an interior);
  then `SeamSpan` and immutable `SewingGraph`, partial and one-to-many edge ranges, declared ease
  distribution, direction and stop landmarks; resolve D35's same-piece seam rule against the supported
  dart and trouser constructions before code.
  Acceptance: a deleted interior fragment blocks full-range integrity even when endpoints resolve;
  absent pieces/edges, empty or reversed parameter ranges, duplicate span identities and
  missing stop references are typed refusals; same-piece seams have an explicit tested contract;
  edits expose repairs without rewriting authored ranges. Geometric differential checks remain G2.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Children: `.3c.2a` (full-range resolution), `.3c.2b` (sewing graph and D35).

- ID: `G1-SLICE.3c.2a`
  Status: `done`
  Goal: discharge D55 by folding the entire positive-length interval through the identity journal;
  expose every surviving fragment, deleted/trimmed interval and arithmetic refusal, preserve traversal
  order and never rewrite the authored range. Piece full-edge queries consume this contract.
  Acceptance: D55's middle deletion produces a visible range repair despite resolved endpoints;
  partial/full ranges survive split/merge/reverse exactly, offset gaps stay visible, every diagnostic
  names the held range and orphaning operation, and undeclared edges are refused. Endpoint point
  ambiguity stays governed by `.3b`; full interval coverage is not a geometric or release certification.
  Verification: 13 range-contract tests + existing suites green; strict clippy, wasm, book,
  feature census, ledger probes and staged doctrine gate green. D55 closed in defects-part2.
  Commit: `STITCHCAD-G1-0007`
  Design: `decision_range-resolution-preserves-entire-interval.md`, recorded before code.

- ID: `G1-SLICE.3c.2b`
  Status: `done`
  Goal: implement the sewing graph content and D35 after `.3c.2a` supplies its range contract.
  Director decision (D57), `2026-10-01`: every physical cut copy has a stable identity, so its
  seams can differ. Implement this explicit addressing contract before sewing spans; do not expand
  an underspecified pattern-only graph by renderer convention. D57 closes after verified code.
  Acceptance: all `.3c.2` sewing-graph criteria met; range and endpoint ambiguity visible; same-piece
  seams explicitly specified and tested. Then close the `.3c.2` parent.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Children: `.3c.2b.1` (explicit cut-copy plan), `.3c.2b.2` (sewing spans and D35).

- ID: `G1-SLICE.3c.2b.1`
  Status: `done`
  Goal: immutable `CutPlan` and `CutCopy` identities supplied explicitly by the caller, each naming
  a pattern Piece and authored/reflected orientation. Validate exact per-piece quantity, identity
  uniqueness and mirroring distribution; never derive a persistent copy identity from array order.
  Acceptance: copies of one Piece have distinct identities; reordering retains those identities;
  unknown/duplicate pieces or copies, missing/excess copies, and incompatible orientations are typed
  refusals. MirroredPairs has equal authored/reflected counts; Single and separate PairMember copies
  retain authored orientation. Copy identities do not promise geometry or manufacture assembly state.
  Signoff alignment: D59 reproduces 15 undeclared API tokens in committed ontology docs at
  `6abfac3` (`GLOSSARY_ROOT` census of a `git archive` snapshot, `rc=1`); current additions also
  need chapter-local vocabulary declarations. Fix here before commit and run the glossary census.
  Signoff alignment: D60 is the staged gate refusing the moved historical doc-only `.1` ROOT
  CAUSE box with no tool-output signature; `.2` has the same issue. Supplement both with re-derived
  commit evidence in their own bullets; do not waive the gate or shadow them with a newer box.
  Verification: nine cut-contract tests + privacy doctest, strict check, wasm, book,
  glossary/feature/tree censuses, ledger probes and staged doctrine gate green; D59/D60 closed.
  Commit: `STITCHCAD-G1-0010`
  Design: `decision_physical-cut-copies-have-stable-identities.md` (director ruling, before code).

- ID: `G1-SLICE.3c.2b.2`
  Status: `done`
  Goal: immutable sewing graph whose sides name validated CutCopy identities and partial edge ranges;
  explicit correspondence direction, ease declaration and stop anchors. Settle D35 before code.
  Acceptance: partial and one-to-many spans distinguish physical copies of one Piece; graph references
  resolve through the cut plan and owned topology; missing/duplicate references refused. Same-copy
  seams have an explicit tested contract, positive-range interiors and endpoint choices stay visible.
  D57 closes after sewing integration; D35 closes after its self-seam regression and documentation.
  Verification: 18 sewing contracts + existing range/copy/notch suites, strict check, wasm,
  book, fixture/feature/glossary censuses, ledger probes and staged doctrines green. D35/D57 closed.
  Commit: `STITCHCAD-G1-0011`

  Design: `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`, before code.
  Same-copy contract (D35): legal for ranges whose current positive-length interiors are disjoint;
  shared endpoints are legal, an overlapping/identical material interval sewn to itself is refused.
  Different copies of the same Piece may correspond over the same source interval. Same-copy
  disjointness is checked in resolved topology, not only by authored EdgeRef inequality.
  Stop landmarks: semantic Notch or born-valid edge-anchored TurnPoint, named by stable id and
  attached to an explicit span side/copy. Stops must belong to that side's current interval.
  Ease: signed A-minus-B declaration (explicit length or symbolic parameter); uniform, weighted
  parameter regions or between explicit notch landmarks. G2/G3 owns geometric differential checks.

- ID: `G1-SLICE.3c.3`
  Status: `done`
  Goal: `Notch`, directed and dual-reference `Grainline`, per-edge derived `SeamAllowance`; profile
  parameters are identity references rather than invented defaults until G4 supplies parameter states.
  Acceptance: references exist at construction; notch export geometry and encoding remain profile-owned;
  unknown profile parameters are never replaced by a value; directed references survive reversal or
  show repairs, and allowance inclusion is resolved per profile rather than as a global switch.
  Verification: notch, grain and allowance contracts + privacy; strict Rust/WASM, synchronized
  book and focused censuses/gates pass in child commits.
  Commit: child slices `STITCHCAD-G1-0009`, `STITCHCAD-G1-0012`, `STITCHCAD-G1-0013`
  Children: `.3c.3a` (notches), `.3c.3b` (grainlines), `.3c.3c` (allowances).

- ID: `G1-SLICE.3c.3a`
  Status: `done`
  Goal: semantic `Notch` with stable piece/edge/parameter anchoring and symbolic Factory Profile
  bindings for style, sample/production depth/width and encoding. Bindings carry no scalar values;
  their validation is visibly `DeferredToG4`, rather than treating an unread profile as resolved.
  Acceptance: absent edges and anchors outside the named piece are typed refusals; a split/merge/
  reverse/delete exposes the existing point-resolution contract without changing stored references;
  style vocabulary matches §4.5, sample/production bindings remain distinct, and no unresolved binding
  can supply default geometry or an encoding. The object is immutable and wasm-safe.
  Signoff alignment: fix D58's obsolete `G4-PROFILES.7` acceptance, which contradicts its own
  no-default Goal and the normative release §8 matrix (draft notch unknown → sidecar, never default).
  Verification: seven notch-contract tests + privacy doctest; strict check, wasm, book,
  feature/release censuses, ledger probes and doctrine gate green. D58 closed in defects-part4.
  Commit: `STITCHCAD-G1-0009`
  Design: `decision_profile-bindings-stay-symbolic-at-g1.md`.

- ID: `G1-SLICE.3c.3b`
  Status: `done`
  Goal: directed `Grainline`, explicit parallel/off-grain angle binding and independent stripe/plaid
  references, with born-valid anchors and visible resolution/repair after topology edits.
  Acceptance: every geometric reference exists and belongs to its declared scope; opposite directions
  are distinct; reversal reports its accumulated direction; angle bindings do not invent values;
  dual print references are first-class and no geometric direction equality is asserted until G2.
  Verification: nine grain contracts + privacy doctest; strict Rust, wasm, book, feature/glossary
  censuses, ten glossary probes, ledger and doctrine gates green; reverse-order mutation red.
  Commit: `STITCHCAD-G1-0012`

  Design: directed positive EdgeRanges in the source Piece's frame; semantic arrow plus explicit
  alignment reference, independent optional stripe and plaid references. Parallel means codirected;
  AtAngle carries an explicit Angle or formula/profile declaration, including 45° bias or 180°.
  Whole-range ownership/coverage and endpoint uniqueness checked now, straightness/actual angular
  relationship deferred to G2. Query traversal composes authored and journal direction, reverses
  fragment order for opposite traversal, and preserves raw endpoint choices/interval repair evidence.
  Implementation shares the already-tested sewing range-ownership fold instead of duplicating it.
  Book containment: split §10's executable examples into `ontology-implementation.md` before this
  append approaches the ontology chapter's 40960-byte ceiling. Keep §10 as a bounded implementation
  index, preserve all existing examples/API vocabulary, and register the chapter in SUMMARY.
  D61 signoff alignment: the glossary census's C1 diagnostic says no table "in that chapter"
  declares a token, while its actual declared-token set aggregates all spec tables. Correct the
  wording to name the actual scope; preserve the global one-token vocabulary contract and probes.

- ID: `G1-SLICE.3c.3c`
  Status: `done`
  Goal: per-edge derived allowance descriptors, authored width-parameter origin, explicit corner choice
  and symbolic per-profile inclusion policy; then close the marks/allowances parent.
  Acceptance: live owned edges required; width may come from formula/profile/explicit parameter
  entities without duplicating their uncertainty state; corner vocabulary matches §4.4; inclusion is
  resolved per target profile; offset geometry/error-budget checks remain visibly G2 obligations.
  Verification: eight allowance contracts + privacy; ownership mutation red; strict Rust, wasm,
  book, feature/glossary censuses, ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0013`

  Design before code: immutable per-edge descriptor; explicit width carries its authored parameter
  id plus nonnegative Length, formula/profile widths retain declaration ids without cached states.
  Zero is legal only when explicitly authored, never inferred from an unread binding. Corner is an
  explicit five-way enum; inclusion is a mandatory logical profile parameter, with vocabulary only
  (included in contour / generated downstream). No resolved policy or offset geometry returned at G1.
  Whole held edge resolves with complete Piece-owned interval coverage and unique endpoints. Queries
  preserve split/merge/reversal/repair evidence without reassigning the descriptor to a replacement.
  Extend the existing profile-binding decision before implementation. `.6` owns collection identity
  and parameter registry validation; G2 owns generated offsets/error budgets; G4 resolves inclusion.
  Containment ownership: move older completed object checklists to the existing evidence sibling
  before this slice's checklist crosses 1000 lines; preserve evidence and revalidate staged gates.

- ID: `G1-SLICE.3c.4`
  Status: `done`
  Goal: semantic `Dart`/`Tuck`/`Pleat`/`Gather`, `Hem`, `Facing`/`Lining`/`Interfacing`, `Closure` and
  `Pocket` with required content and validated structural references; close the object-type parent.
  Acceptance: required anchors, operation identities, composition and parameter references are carried;
  buttonhole size derives from its button rather than a second input; unsupported constructions are
  refused explicitly; intake conservation stays visibly deferred to G2. Every ontology §4 object has
  implementation evidence, synchronized book content and wasm-safe tests.
  Verification: object/support suites 149 contracts; sc-core 260 tests incl. 18 docs; full strict
  Rust/WASM/book, 22 probe suites, focused censuses/ledger and staged doctrine gates green.
  Commit: `STITCHCAD-G1-0023`

  Children: `.3c.4a` (intake constructions), `.3c.4b` (hem/layers), `.3c.4c` (closures),
  `.3c.4d` (pockets and family signoff). Safe child slices preserve the entire §4.3/§4.7 scope.

[Completed construction child contracts and acceptance evidence](G1-SLICE-constructions.md)
are preserved unchanged in a bounded sibling; .3c.4 remains closed by STITCHCAD-G1-0023.

- ID: `G1-SLICE.4`
  Status: `active`
  Goal: `sc-measure` — MeasurementTable (body vs garment POM, landmarks, source, procedure),
  Ease as a first-class body→garment mapping with fit intent, and SizeSet (implements
  `G0-CONTRACT.4`/`.6`).
  Acceptance: a POM without a landmark or procedure is rejected; ease is queryable per POM;
  size labels vs order vs base size are distinct fields with tested semantics.
  Children: .4a (measurement declarations/tables), .4b (per-POM ease), .4c (SizeSet), .4d (signoff).
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4a`
  Status: `done`
  Goal: canonical value/state/provenance plus MeasurementTable body/POM metadata and reference registries.
  Children: .4a.1 (shared length declarations), .4a.2 (measurement metadata/context), .4a.3 (table/signoff).
  Acceptance: all ontology .2.1 fields present; unknowns supply no numeric value; stable landmark/
  procedure references, kind separation and current context validation are explicit and tested.
  Verification: .4a.1/.2/.3 supply canonical length state/source, metadata/targets and named unique
  current table bindings; all ontology .2.1 fields covered structurally. Native/WASM checked,
  metadata runner proof captured; source/physical/formula/profile/release proof remains deferred.
  Commit: children `STITCHCAD-G1-0024` … `STITCHCAD-G1-0028`

[Completed .4a.1 contract and evidence](G1-SLICE-measurements.md) retain the shared length input
boundary unchanged; current metadata children remain below.

- ID: `G1-SLICE.4a.2`
  Status: `done`
  Goal: immutable body/garment measurement metadata with stable lower-snake token, entered unit,
  canonical declaration link, two landmark references, procedure and source; typed current registries.
  Children: .4a.2a (shared machine tokens), .4a.2b (metadata/records + runtime integration),
  .4a.2c (observed CI and metadata signoff).
  Acceptance: missing/foreign/ambiguous landmark/procedure metadata refuses; body/POM never interchanged;
  declaration borrowing preserves source/state without a duplicate value cache. No standards data invented.
  Verification: shared tokens and all metadata/reference fields covered by child contracts;
  observed bf29b03 Rust/doctrine jobs and all steps completed success. Physical/source proof deferred.
  Commit: children `STITCHCAD-G1-0025` … `STITCHCAD-G1-0027`

[Completed .4a.2a contract/evidence](G1-SLICE-measurements.md) preserves the shared token grammar
unchanged; the metadata/runtime child below remains the current unit.

[Completed .4a.2b contract/evidence](G1-SLICE-measurements.md) retains metadata/runtime
implementation scope unchanged; the observed-CI signoff below completes the parent.

[Completed .4a.2c observed-CI contract/evidence](G1-SLICE-measurements.md) preserves its exact
metadata signoff unchanged; the named table slice follows.

- ID: `G1-SLICE.4a.3`
  Status: `done`
  Goal: named tables/current bindings and .4a structural signoff.
  Verification: [complete preserved contract and checklist](G1-SLICE-measurements.md#named-table-contract-and-evidence--preserved-from-f19982d).
  Commit: `STITCHCAD-G1-0028`

- ID: `G1-SLICE.4b`
  Status: `done`
  Children: .4b.1 (individual intent), .4b.2 (set/membership), .4b.3 (structural review).
  Goal: first-class body-to-garment per-POM Ease, signed Length declaration and ordered close/semi/loose
  fit intent; explicit negative-ease authorization, canonical source/state/provenance and current links.
  Acceptance: lookup per POM, kind correctness, declared compression and no default unknown; only typed
  intent here, actual garment fit/physical construction remains G2/G3. Split safely before code if needed.
  Verification: individual .4b.1 and set .4b.2 contracts/mutations plus .4b.3 structural field review
  and full 23-suite milestone green; evaluation/physical/source/release proofs explicitly deferred.
  Commit: `STITCHCAD-G1-0031`

- ID: `G1-SLICE.4b.1`
  Status: `done`
  Goal: individual canonical Ease intent and explicit signed permission.
  Verification: [complete preserved contract/checklist](G1-SLICE-measurements.md#individual-ease-contract-and-evidence--preserved-from-0088969).
  Commit: `STITCHCAD-G1-0029`

- ID: `G1-SLICE.4b.2`
  Status: `done`
  Goal: unique current per-POM sets with named table membership.
  Verification: [complete preserved contract/checklist](G1-SLICE-measurements.md#per-pom-set-contract-and-evidence--preserved-from-b4e0bc7).
  Commit: `STITCHCAD-G1-0030`

- ID: `G1-SLICE.4b.3`
  Status: `done`
  Goal: structural Ease field/currentness review and milestone.
  Verification: [preserved review contract](G1-SLICE-measurements.md#structural-ease-review-contract--preserved-from-71aaf19)
  and [preserved evidence](G1-SLICE-measurements.md#structural-ease-review--preserved-from-b6bd985).
  Commit: `STITCHCAD-G1-0031`

- ID: `G1-SLICE.4c`
  Status: `in_progress`
  Children: .4c.1 (membership), .4c.2 (axes), .4c.3 (chart/breaks), .4c.4 (resolution), .4c.5 (review).
  Goal: SizeSet identity/revision/system, authored ordered labels/base, chart/state/provenance, adjacent
  breaks and axes; immutable profile resolution/transformation intent per canonical SizeSet decision.
  Acceptance: label/order/base distinct; path-2 missing-break refusal, MTM of one, no quantities/defaults;
  profile transformations never mutate Design references; execution/equivalence/approval later owned.
  Split into safe slices before implementation; preserve every size-sets chapter requirement.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.1`
  Status: `done`
  Goal: stable authored size membership and checked revision successors.
  Verification: [preserved full contract/checklist](G1-SLICE-measurements.md#size-membership-contract-and-evidence--preserved-from-0b77235).
  Commit: `STITCHCAD-G1-0032`

- ID: `G1-SLICE.4c.2`
  Status: `pending`
  Goal: axes with ordered values and member coordinate references; incomplete grids remain explicit.
  Resolve D70's optional-versus-universal-axis specification conflict before choosing its representation.
  Acceptance: current axis/value identities and complete member points validated, no grid interpolation;
  no label-derived body measurements or silent axis defaults. Preserve source/state for numeric axes.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.3`
  Status: `in_progress`
  Children: .4c.3a (garment chart observations), .3b (chart collection), .3c (MTM body inputs),
  .3d (breaks/composite SizeSet), .3e (review). .3a can proceed independently of D70 axes cardinality.
  Goal: complete SizeSet chart/break object over membership/axes, canonical per-member POM scalars,
  landmarks/procedures/state/provenance and adjacent signed breaks; current references and path readiness.
  Acceptance: chart and generated POMs distinct; missing breaks name dimension/pair and refuse path 2;
  zero/uneven breaks legal; MTM of one uses body provenance and only path 1; no quantity/default values.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.3a`
  Status: `done`
  Goal: canonical current garment chart observations with stable member/POM references.
  Verification: [preserved full contract/checklist](G1-SLICE-measurements.md#garment-chart-observation-contract-and-evidence--preserved-from-71aaf19).
  Commit: `STITCHCAD-G1-0033`

- ID: `G1-SLICE.4c.3b`
  Status: `done`
  Goal: immutable current Design/member/POM garment-chart coverage; full contract/evidence preserved
  in [measurement evidence](G1-SLICE-measurements.md#garment-chart-collection-evidence--preserved-from-e299771).
  Verification: 18 contracts/privacy, fourteen real reds; 422 strict tests/WASM/book; staged doctrines.
  Commit: `STITCHCAD-G1-0034` (`e299771`).

- ID: `G1-SLICE.4c.3c`
  Status: `done`
  Goal: custom-member body/Ease inputs; full contract/evidence retained in
  [measurement evidence](G1-SLICE-measurements.md#mtm-contract-and-evidence--preserved-from-285e238).
  Verification: 15 contracts/two privacy-role docs, twelve real reds; 439 strict tests/WASM/book.
  Commit: `STITCHCAD-G1-0035` (`285e238`).

- ID: `G1-SLICE.4c.3d`
  Status: `pending`
  Goal: signed canonical adjacent breaks and composite SizeSet over membership/axes/chart with source/
  state/provenance; per-axis/dimension coverage and typed path readiness, after D70 axes ruling.
  Acceptance: authored adjacency, zero/uneven breaks, current references, missing dimension/pair and
  MTM path-2 refusal; no rounded values, generated equivalence or profile execution claimed.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.3e`
  Status: `pending`
  Goal: chart/body/break/currentness requirement review and milestone checks before closing .3;
  remaining profile transformation .4 and full SizeSet .5/.4d stay owned.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.4`
  Status: `pending`
  Goal: immutable resolved SizeSet/profile transformation intent, pinned Design reference/profile revision,
  typed label mapping/break scaling/base substitution/chart replacement with state/evidence provenance.
  Acceptance: new resolved object retains original reference and records transformation; no in-place
  mutation. Execution, precedence/hard restrictions, equivalence and release approval retain later owners.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4c.5`
  Status: `pending`
  Goal: review every size-sets/ownership requirement and deferred execution/proof boundary; milestone
  checks and book sync before closing .4c structurally, then combined measurement/Ease/SizeSet .4d review.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4d`
  Status: `pending`
  Children: `.4d.1` book publication/alignment, `.4d.2` combined structural exit review.
  Goal: re-derive measurement/ease/size structural requirements, current reference contracts and book
  evidence; full milestone verification, close .4 only after every child acceptance is verified.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4d.1`
  Status: `done`
  Contract/checklist: [preserved publication evidence](G1-SLICE-measurements.md#publication-contract-and-evidence--preserved-from-9ef9602).
  Verification: 47 chapters/14 APIs; publication nine, archive 28, full 24 suites green.
  Commit: `STITCHCAD-G1-0036`.

- ID: `G1-SLICE.4d.2`
  Status: `pending`
  Goal: perform .4d's combined measurement/Ease/SizeSet structural exit review only after all .4
  implementation children pass; publication improvements do not close this separate requirement.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5`
  Status: `in_progress`
  Syntax frontier: `.5a` may proceed while D70 blocks complete SizeSet. Membership/table/Ease
  foundations are implemented; expression syntax consumes no axis or full SizeSet representation.
  This dependency exception leaves .4's remaining children owned and unclosed.
  Goal: formula graph + construction-recipe evaluation — acyclic dependency graph, single
  deterministic pass, name binding (measurements, prior points/lengths/angles, profile
  parameters), conditionals, units inside expressions (implements the `G0-CONTRACT.9` language).
  Acceptance: evaluation is byte-reproducible across runs and platforms; a cycle or unbound name
  is a structured diagnostic naming the formula; every spec example in the formula chapter is a
  test.
  Pocket openings: validate logical declaration kind/domain/state and component recipe dependencies;
  G3 executes physical construction and determines supported opening scope without approximation.
  Buttonhole derivation: validate typed operation/dependency on the canonical button-size declaration;
  G3 executes it and validates positive physical length without a separately authored hole length.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a`
  Status: `in_progress`
  Children: `.5a.1` lexical stream, `.5a.2` expression trees, `.5a.3` literal/canonical recipes,
  `.5a.4` syntax review. Finalize each protocol after reading its complete affected contracts.
  Goal: expression-language syntax from the full formula contract/grammar/examples. Read all three
  before finalizing a pre-code protocol and safe parser subleaves; introduce immutable syntax and
  precise source diagnostics without claiming evaluation, generated geometry or resolved SizeSet.
  Acceptance: syntax, keywords, literals/units, precedence/conditionals, structural bounds and
  rejected forms conform to the normative language; canonical/display forms and source spans are
  explicit. Type/name binding, exact evaluation/DAG and operation recipes remain .5's later children.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.1`
  Status: `done`
  Contract/checklist: [preserved lexical evidence](G1-SLICE-formulas.md#lexical-contract-and-evidence--preserved-from-60c7305).
  Verification: 13 contracts/two privacy-lifetime docs; nine actual reds; strict 454 tests/WASM/book green.
  Commit: `STITCHCAD-G1-0037`.

- ID: `G1-SLICE.5a.2`
  Status: `done`
  Goal: expression syntax/precedence/conditional trees with node/depth bounds and precise parse
  refusals; finalize the full pre-code protocol before implementation, preserve static/runtime split.
  Children: `.5a.2a` reference structural-bound control; `.5a.2b` bounded expression trees.
  Prerequisite D75: live census passes abs-wrapped expressions with 258 nodes/17 conditional levels,
  reporting only seven/zero because its tuple-only walker skips call argument lists. Correct oracle
  traversal/early depth refusal before using it as independent parser evidence; language caps stay fixed.
  Verification: product 15 contracts/three privacy-lifetime docs; eleven actual assertion reds;
  reference twelve shape/count/depth fixtures; restored strict 472 tests/WASM green.
  Commit: `STITCHCAD-G1-0040` (this recording commit).

Completed reference structural contract .5a.2a is preserved in
[G1-SLICE-formulas](G1-SLICE-formulas.md#structural-reference-contract-and-evidence--preserved-from-9d26ddc).

- ID: `G1-SLICE.5a.2b`
  Status: `done`
  Children: `.5a.2b.1` reference machine-input parity; `.5a.2b.2` product expression trees.
  Goal: finalize expression-tree protocol from complete grammar and corrected reference bounds, then
  implement immutable precedence/conditional syntax with precise refusals; no numeric evaluation claim.
  Verification: product 15 contracts/three privacy-lifetime docs; eleven actual assertion reds;
  reference twelve shape/count/depth fixtures; restored strict 472 tests/WASM green.
  Commit: `STITCHCAD-G1-0040` (this recording commit).

Completed reference input contract .5a.2b.1 is preserved in
[G1-SLICE-formulas](G1-SLICE-formulas.md#reference-input-contract-and-evidence--preserved-from-e797874).

- ID: `G1-SLICE.5a.2b.2`
  Status: `done`
  Contract/checklist: [preserved production syntax](G1-SLICE-formulas.md#production-expression-contract-and-evidence--preserved-from-b681a49).
  Verification: 15 contracts/three docs, eleven actual reds, twelve reference fixtures; strict 472/WASM.
  Commit: `STITCHCAD-G1-0040`.

- ID: `G1-SLICE.5a.3`
  Status: `in_progress`
  Goal: exact literal normalization and canonical S-expressions, ordered binding/assert recipes and
  statement limits. Finalize each numeric/identity protocol from full normative contracts before code.
  Children: .3a rounding primitive guard, .3b reference literal identity, .3c exact literals,
  .3d canonical expressions, .3e ordered recipe syntax, .3f coupled contract review.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.3a`
  Status: `done`
  Contract/checklist: [preserved rounding evidence](G1-SLICE-formulas.md#rounding-contract-and-evidence--preserved-from-543dfa6).
  Verification: four public/36 Fraction/five reds; strict 476/release four/WASM green.
  Commit: `STITCHCAD-G1-0041`.

- ID: `G1-SLICE.5a.3b`
  Status: `done`
  Goal: repair D79/D83 reference numeric identity/domains and resolve D84/D95 before product proof.
  Children/contracts: [complete preserved reference subtree](G1-SLICE-journal.md#completed-reference-subtree--preserved-during-g1-0058).
  Verification: independent rational/scalar/binding/canonical/angular/replay controls and actual
  compiled assertion reds; [scoped review](G1-SLICE-journal.md#scoped-reference-review-evidence--preserved-from-ea3f64f).
  Commit: `STITCHCAD-G1-0057`; production conversion/evaluation remains separately owned.

- ID: `G1-SLICE.5a.3c`
  Status: `done`
  Goal: exact bounded literal normalization with typed kind/integer/source and shared rounding;
  D95 literal magnitude/reduced rational width128 is distinct from bound signed64. Preserve kind
  and unary syntax, input quantum/scalar boundaries; normalized Angle cannot hold raw sweeps.
  Children: .1 shared unsigned128 magnitude rounding; .2 exact typed literal normalization;
  .3 immutable normalized expression arena; .4 coupled normalization review. No evaluation.
  Literal conversion must reduce exact converted rational before its128-bit width check; scalar
  length bound follows input rounding. Arbitrarily large raw decimals must not narrow before valid
  cancellation, and pathological input must not require unbounded workspace. The completed .2 protocol uses bounded exact decimal reduction with no added dependency; whole
  arena normalization and its coupled review are complete in .3/.4.
  Verification: exact source identities and full obligation map; independent frontier176/four actual
  compiled reds/current native514/release14/WASM3. No binding/evaluation/serializer/entity claim.
  Commit: `STITCHCAD-G1-0058` … `STITCHCAD-G1-0061`.

- ID: `G1-SLICE.5a.3c.1`
  Status: `done`
  Goal: shared unsigned128 half-away magnitude rounding for wide positive canonical literal nodes.
  Verification: five public contracts/138 independent Decimal rows, nine compiled debug reds/one
  release red; original signed four contracts/36 Fraction rows/five reds retained. Strict native494,
  sc-units46, release public contracts and three WASM crates pass. Source restoration exact.
  [Complete pre-code protocol](G1-SLICE-journal.md#unsigned-rounding-pre-code-protocol--preserved-during-g1-0058).
  Commit: `STITCHCAD-G1-0058`; no literal conversion/arena/evaluation claim.

- ID: `G1-SLICE.5a.3c.2`
  Status: `done`
  Goal: exact typed canonical literal inputs from validated borrowed syntax; retain kind/span/source,
  full128 magnitude and unary separation. Bounded decimal reduction precedes128-bit rational check,
  then shared input rounding and scalar length check. Raw angles retain turns; no binding/evaluation.
  Protocol: [complete pre-code contract](G1-SLICE-journal.md#literal-conversion-pre-code-protocol--completed-in-g1-0059).
  Verification: five public contracts/100 independent Fraction rows/two privacy-lifetime docs;
  thirteen compiled actual assertion reds/exact restoration; strict native501, release five/WASM3.
  Commit: `STITCHCAD-G1-0059`.

- ID: `G1-SLICE.5a.3c.3`
  Status: `done`
  Goal: immutable normalized arena retains every node/edge/name/operator/span and syntax bound;
  convert all literals with .2, preserve unary separation/raw turns and source borrowing. No execution.
  Protocol: [complete pre-code plan](G1-SLICE-journal.md#normalized-arena-pre-code-protocol--completed-in-g1-0060).
  Verification: eight public contracts/24 independent shape rows/nested100 literal rows/25 book
  expressions/three privacy-lifetime docs plus runnable doc; seventeen compiled actual assertion reds/
  exact restoration; strict native513, release eight/WASM3. Flat conversion/clone/drop on64KiB stack.
  Commit: `STITCHCAD-G1-0060`.

- ID: `G1-SLICE.5a.3c.4`
  Status: `done`
  Goal: coupled scoped production normalization review against the numerical input contract/D95/D84.
  Protocol: [complete pre-code review](G1-SLICE-formulas.md#coupled-normalization-pre-code-review--completed-in-g1-0061).
  Verification: all obligations mapped to public controls/source identities; independent176 Fraction
  frontier rows/103 accepted/73 width refusals through individual and nested whole APIs; four actual
  compiled assertion reds/exact restoration. Strict native514, release14, WASM3, book checks pass.
  D101 index/D102 record placement repaired and independently checked; .5a.3c closes for inputs only.
  Commit: `STITCHCAD-G1-0061`.

- ID: `G1-SLICE.5a.3d`
  Status: `done`
  Goal: iterative canonical S-expression identity over privately normalized expression nodes;
  preserve operators/names/child order, source locations distinct from canonical bytes, explicit scope.
  Children: .1 byte contract/D103; .2 owned serializer; .3 coupled identity review, all done.
  Verification: [complete expression proof map](../book/src/annexes/formula-literals.md#coupled-canonical-identity-review);
  nine public contracts/25 worked expressions/max255 arguments/three new compiled assertion reds,
  native525/release9/WASM3 pass. No statement/binding/evaluation or project-storage claim.
  Commit: `STITCHCAD-G1-0062` … `STITCHCAD-G1-0064`.

Completed identity children are preserved in
[G1-SLICE-canonical](G1-SLICE-canonical.md#completed-expression-identity-subtree--preserved-from-5d8ab5b).

- ID: `G1-SLICE.5a.3e`
  Status: `done`
  Children: .1 single immutable statements, .2 ordered recipe/4096/context, .3 coupled syntax review.
  Goal: ordered immutable let/assert syntax, six bindable kinds/five tolerances, statement boundaries,
  4096 limit and diagnostic statement/expression context where available; no binding/evaluation claim.
  Verification: [coupled syntax proof map](../book/src/annexes/formula-statements.md#coupled-syntax-and-diagnostic-review);
  statement9/recipe11, native553/release20/WASM3, five new actual assertion reds/source exact.
  D110 fixed; D109 exact recipe byte contract explicitly next .3f.1; no numerical acceptance.
  Commit: `STITCHCAD-G1-0065` … `STITCHCAD-G1-0067`.

- ID: `G1-SLICE.5a.3e.1`
  Status: `done`
  Goal: immutable borrowed single-statement syntax, closed annotation roles, exact full-source spans
  and expression-part refusals; reuse bounded flat parser without conversion/name/type/evaluation.
  Protocol: [complete statement plan](G1-SLICE-canonical.md#single-statement-pre-code-protocol).
  Verification: nine public contracts/15 independent rows/six reference refusals/all21 worked
  statements/25 expression bytes/15 actual compiled assertion reds/exact restoration; native538/
  release9/WASM3. Three privacy/lifetime docs and runnable API; D106/D107 fixed.
  Commit: `STITCHCAD-G1-0065`.

- ID: `G1-SLICE.5a.3e.2`
  Status: `done`
  Goal: ordered complete recipe syntax, empty/multiline lists, fixed4096 statement bound and measured
  first-excess refusal, statement indices/global spans, no partial accepted result or evaluation.
  Protocol: [ordered recipe plan](G1-SLICE-canonical.md#ordered-recipe-pre-code-protocol).
  Verification: eight public contracts/nine authored whole sources/14 statements/18 identities,
  all21 worked statements/25 bytes;15 new actual assertion reds/prior15 statement faults/exact restore;
  native550/release17/WASM3, three private/lifetime docs/runnable API; D108 archive copy repaired.
  Commit: `STITCHCAD-G1-0066`

- ID: `G1-SLICE.5a.3e.3`
  Status: `done`
  Goal: coupled recipe/statement syntax and diagnostic review with all worked statements; keep
  normalization/statement canonical bytes/static binding/evaluation owners explicit before .3f.
  Protocol: [coupled syntax review plan](G1-SLICE-canonical.md#coupled-statement-recipe-review-protocol).
  Verification: three coupled controls/four authored sources/eight chunks/ten identities; twelve
  header/four prefix reference refusals; simultaneous4096×2×256/16 on64KiB; five new compiled
  assertion reds/exact restore and all product sources unchanged415d577. Native553/release20/WASM3;
  structural/reference, language16/publication9 and book proof map pass. D110 wording fixed.
  Commit: `STITCHCAD-G1-0067`.

- ID: `G1-SLICE.5a.3f`
  Status: `pending`
  Children: .1 complete recipe literal normalization/canonical bytes, .2 coupled contract review.
  Goal: complete canonical recipes and coupled contract review before .5a.4 milestone;
  syntax-only success never grants static name/type/evaluation or geometry acceptance.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.3f.1`
  Status: `pending`
  Goal: finalize exact statement/ordered-recipe byte contract and scoped complete-recipe input
  normalization; preserve names/annotations/order/spans and owned identity without binding/evaluation.
  D109 P2 prerequisite: exact assertion/empty/ordered-recipe bytes and spacing/envelope/terminal-newline
  contract must be settled with concrete examples before any serializer or persistent identity.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.3f.2`
  Status: `pending`
  Goal: coupled statement/recipe normalization/identity review and all worked examples, prerequisite
  map and full syntax-scope closure before .5a.4; no numerical or geometry execution claim.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.4`
  Status: `pending`
  Goal: review all syntax/canonical-form requirements and deferred name/type/evaluation owners,
  full milestone gate before closing .5a; exact evaluator/DAG and operations remain .5 children.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.6`
  Status: `pending`
  Goal: command bus (§4.4) — typed validated commands, atomic groups, preview/commit, revision
  preconditions, idempotency keys, structured errors, progress for long operations, and the
  undo/redo granularity decided at G0.
  Acceptance: no mutation path bypasses the bus (enforced by module privacy and a test that the
  domain types expose no public mutators); a stale revision is rejected; undo/redo restores
  semantics, not just geometry. Design-level reference validation consumes BOTH point registrations
  and whole-range repair evidence (`.3c.2a`); a lost interior blocks readiness even with live endpoints.
  Pair-member metadata (`.3c.1a`) is checked against the design collection: companions exist, name
  each other, have opposite handedness and equal quantities. Geometric mirroring stays G2's check.
  Sewing amount references (`.3c.2b.2`) also resolve to existing length-valued formula/profile
  declarations before design acceptance; symbolic identities are not valid parameter values.
  Verification: `pending`
  Commit: `pending`

  Pockets: validate current copy/source bindings, position/orientation and opening declarations;
  inspect every component Piece contour repair before execution/release. A borrowed target or
  successful Pocket placement check does not certify component geometry or opening scope.

  Closures: validate every current placement/instance, declaration and notion count before execution;
  unsupported fly requests retain named env_fly with requested Closure/trousers gap/G7.

  Notion placements: validate current plan, original source Piece and all held anchor/direction
  references with validate_current(); raw evidence alone is not a registry/release certificate.

  Hem composition: validate current Facing identities/served owners/sources and depth/fold declarations
  before execution; independent edge/target queries alone do not grant release readiness.

  Layer execution scope: `.3c.4b.1` models lining but its `require_in_scope()` returns env_lining.
  Apply envelope refusals before requested construction execution; inspection preserves modelled content.

- ID: `G1-SLICE.7`
  Status: `pending`
  Goal: `sc-store` (§4.5) — canonical-text project directory (stable key order, declared float
  formatting, ULID ids, no wall-clock), schema version + migrations, recoverable-unknown-extension
  preservation, crash-consistent atomic multi-file commits, autosave/recovery, and the repository
  trait with the named WASM (files + IndexedDB) backend.
  Acceptance: forced-interruption and failed-migration recovery are tested, not asserted; a
  saved project reopens to a semantically equal instance; two saves of the same state are
  byte-identical.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.8`
  Status: `pending`
  Goal: `sc-constraints` kernel (§6.1/§6.2) — typed constraint AST, finite-domain propagation +
  search with declared variable/value ordering, the four distinguished outcomes (satisfied /
  proven-unsatisfiable / unknown / numerically-failed), and the non-default `csp-z3` differential
  oracle feature wired but not shipped.
  Acceptance: determinism proven by repeated runs over the same input (fixed search order, no
  randomness, no wall-clock); `cargo build --features csp-z3` is a CI-only path and the default
  release artifact contains no Z3 dependency (verified by a dependency census).
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.9`
  Status: `pending`
  Goal: `sc-api` + `sc-mcp` (§7.8, §10) — versioned experimental command API over jsonrpsee
  (HTTP/WebSocket native, stdio for MCP), in-process Rust API distinct from the wire format,
  curated MCP tool set (workflows, not one tool per getter), scoped agent authority
  (inspect / propose / commit / generate / approve) enforced in core logic, actor trace on every
  mutating command, imported files treated as data.
  Acceptance: authority is enforced by tests that a scoped token cannot exceed (an `approve`
  attempt from a `propose` token fails); stdio-only transport; the parity table's API column is
  populated from the same command list the bus exposes.
  Director 2026-10-01 reaffirmed comprehensive external-agent control through MCP/API. Curated
  workflows must expose every supported user operation with typed discovery, units, stable identities,
  current-state/provenance and actionable diagnostics. Evaluate discovery, invalid-input recovery,
  interruption/resume and parity using independent artifact checks (§7.8); API/MCP control is not a
  measured stitching-expertise claim. Existing human-only release approval remains §10's contract.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.10`
  Status: `pending`
  Goal: `sc-cli` — headless command replay that is deterministic: the same command script produces
  the same project state and the same serialized bytes (§4.4; prerequisite for the G2 exit).
  Acceptance: a recorded session replays byte-identically on a second run; failures exit nonzero
  with structured diagnostics; no wall-clock or locale leaks into output.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.11`
  Status: `pending`
  Goal: the three runtime profiles build (§7.3): `native-full`, `native-headless`, `wasm-viewer`,
  each an explicit cargo feature/target combination with a published capability matrix, plus the
  CI WASM smoketest.
  Acceptance: one command per profile in `Makefile`, all green in CI; the capability matrix lists
  what each profile excludes and why; browser execution is tested for real, not `cargo check`.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.12`
  Status: `pending`
  Goal: the real browser spike (G1 exit) — load project → evaluate recipe → resolve one CSP
  constraint → persist/reopen → render → export, in the intended browser target, with the WASM
  capability matrix (tested versions + limitations) as its output.
  Acceptance: a recorded run (commands + observed output) demonstrates all six steps in a
  browser; limitations are written down (startup size, COOP/COEP, WebGPU baseline) and routed to
  G5 where they belong.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.13`
  Status: `pending`
  Goal: the canvas-hosting spike that settles ADR-0002 against the protocol written at
  `G0-CONTRACT.11` — zoom/pan, snapping, picking, annotations on a realistic pattern set, native
  AND browser, across the three topologies.
  Acceptance: the measurements the protocol named are recorded in
  `docs/tasks/artifacts/canvas_spike/results.tsv` — one row per applicable (topology, profile) pair, every
  gated metric filled, with the reference hardware, the OS versions and the corpus script's identity in the
  file's comment block, because a verdict is scoped to them; the corpus is the declared one (16 pieces,
  400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000 fidelity round trips) or the
  deviation is recorded with its reason; the decision is whatever
  `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` prints, rule trace included, and a human
  overruling it does so by a recorded decision naming the rows it overrules (R6); ADR-0002's canvas half
  moves from `proposed` to `active` citing that output; and where R5 escalates, the fallback named in
  `spike_rule.tsv` is what ships until a re-spike.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.14`
  Status: `pending`
  Goal: the egui/iced dev shell for Stages 1–4 internal tools (ADR-0002's dev-shell ruling) —
  the harness the G2 viewer/editor and the conformance lab are built in, explicitly not the
  shipped product UX.
  Acceptance: the shell hosts a canvas and the command bus on both native and (where possible)
  browser; it is labeled a dev shell in the book so nobody mistakes it for the product.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.15`
  Status: `pending`
  Goal: the license census (G1 exit: "license-compatible solver builds on all targets") — every
  dependency's license recorded, copyleft excluded from the shipped tree per ADR-0001, solver
  builds on all three profiles.
  Acceptance: a re-runnable census command (dependency list + license fields) whose output is
  recorded in the leaf; any GPL/LGPL dependency is either absent or quarantined behind a
  non-default, non-shipped feature with the reason recorded.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.16`
  Status: `pending`
  Goal: G1 exit review — every exit clause cited against its artifact, the capability matrix
  published, the frontier handed to `G2-2D`.
  Acceptance: each clause is `met` with a re-runnable check or `not met` with a named blocker.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| done | `G1-SLICE.4a.3` | `done` | Named table/unique current bindings complete .4a structurally |
| done | `G1-SLICE.4c.1` | `done` | Size membership identity/order/base and revision foundation |
| done | `G1-SLICE.4c.3a` | `done` | Current authored garment chart observations |
| done | `G1-SLICE.4c.3b` | `done` | Explicit current Design/member/POM coverage |
| done | `G1-SLICE.4c.3c` | `done` | Custom member-of-one body/Ease inputs |
| done | `G1-SLICE.4d.1` | `done` | Incremental book, glossary/index/annexes; D71/D72 fixed |
| done | `G1-SLICE.5a.1` | `done` | Borrowed source tokens/spans; no expression-validation/execution claim |
| done | `G1-SLICE.5a.2a` | `done` | Complete reference node/depth traversal and fragment ownership, D75/D77 fixed |
| done | `G1-SLICE.5a.2b.1` | `done` | D76 input parity fixed; 130+3 controls/nine actual reds |
| done | `G1-SLICE.5a.2b.2` | `done` | Immutable iterative expression syntax and structural bounds |
| done | `G1-SLICE.5a.3a` | `done` | D78 extreme public rounding fixed with actual boundary proof |
| done | `G1-SLICE.5a.3b.1` | `done` | D79 literal identity, D80/D81 publication drift fixed |
| done | `G1-SLICE.5a.3b.2` | `done` | D82 exact operators/selector model preserve sub-quantum results |
| done | `G1-SLICE.5a.3b.3a.1` | `done` | D85/D86/D87 angular guards and scoped reference proof |
| done | `G1-SLICE.5a.3c.1` | `done` | Shared unsigned128 rounding prerequisite |
| done | `G1-SLICE.5a.3c.2` | `done` | Exact typed individual literal inputs |
| done | `G1-SLICE.5a.3c.3` | `done` | Immutable normalized syntax arena/all literal inputs |
| done | `G1-SLICE.5a.3c.4` | `done` | Complete coupled normalization input review |
| done | `G1-SLICE.5a.3d.1` | `done` | Exact byte contract settled by D103 director ruling |
| done | `G1-SLICE.5a.3d.2` | `done` | Owned iterative canonical expression serializer |
| done | `G1-SLICE.5a.3d.3` | `done` | Complete expression identity proof map |
| done | `G1-SLICE.5a.3e.1` | `done` | Single immutable let/assert syntax |
| done | `G1-SLICE.5a.3e.2` | `done` | Ordered recipe/4096/context |
| done | `G1-SLICE.5a.3e.3` | `done` | Coupled syntax/diagnostic review |
| next | `G1-SLICE.5a.3f.1` | `pending` | D109 exact recipe bytes, then normalization/identity |

[Completed milestone routing evidence](G1-SLICE-journal.md#milestone-routing-evidence--preserved-from-9b3b9b3)
retains exact D72/D67 diagnostics and ownership.

## Decisions

[Historical design boundaries](G1-SLICE-journal.md#ontology-design-decisions--preserved-from-f70edf7)
retain exact committed notes and their canonical decision links. D84 director ruling preserves signed/
multi-turn formula values and normalizes entity directions; [decision](../decisions/decision_angles.md).
.5a.3b.3c verification is complete. D70 SizeSet axes remains pending under .4c.2.

## Open Questions

- Which spike runs first, browser (`.12`) or canvas (`.13`)? They share fixtures; decided at gate
  entry, and `.12` is the gate's named exit clause so it wins a tie.

## Blockers

- D70 awaits director decision for complete SizeSet under .4c.2. D83/D84 scoped reference review
  is complete; production literal/binding/evaluation proof remains separate.

## Acceptance Checklist

Current code evidence is fresh, ticked and tool-backed in its owning leaf. Future ontology changes
also run glossary/API, feature and publication checks. Prior checklists and authoring rules remain in
[retained navigation](G1-SLICE-journal.md#acceptance-navigation--retained-during-g1-0052),
[formula evidence](G1-SLICE-formulas.md) and [object evidence](G1-SLICE-evidence.md).

Completed lexical/expression/numeric/identity protocols, checklists and commit journals remain in
[formula evidence](G1-SLICE-formulas.md), [numeric journal](G1-SLICE-journal.md) and
[identity/statement evidence](G1-SLICE-canonical.md#prior-resume-routes--preserved-during-g1-0065).


### `G1-SLICE.5a.3e.3` — coupled statement/recipe syntax review

- [x] **REPRODUCE / ISSUE** — complete grammar/contract/API review identifies D109 missing future
  recipe byte envelope and D110 impossible unconditional diagnostic context; tracked before edits.
- [x] **ROOT CAUSE (WHY + WHERE)** — grammar4 lacks assertion/envelope exact bytes; contract5.2
  assumes accepted canonical/index context before malformed syntax can supply it. Syntax API is sound.
  `git diff -- docs/book/src/spec/formula-language.md` shows old unconditional versus available
  context wording, rc=0.
- [x] **FIX** — qualify book/ADR context availability; map every syntax obligation to public proof,
  add coupled token/diagnostic/maximum controls and reference fixtures; schedule D109 next .3f.1.
- [x] **ADDRESSED (verified)** — five actual compiled boundary/index/rule/EOF/limit faults fail
  assertions only in new controls; failed-body classifier and exact restoration observed, rc=0.
- [x] **NO REGRESSION** — strict native553/45groups, release20/WASM3, syntax reference/language16/
  publication9 pass; all product Rust sources byte-identical415d577. Maxima checked on64KiB stack.
  `make check` and release `cargo test` observed terminal rc=0;553/20 passing tests.
- [x] **LOCKSTEP** — book proof map/index/grammar/contract/existing ADR and live/current records
  align; original closure/oldest ledger payloads preserved exactly. D110 closed, D109 owned next.

## Verification Log

Pre-change review protocol preceded test/instrument/doc edits. Prior ordered-recipe closure remains
exact in [semantic sibling](G1-SLICE-canonical.md#ordered-recipe-closure--preserved-during-g1-0067).
Three new public controls bring recipe suite11, statement suite9. Actual coupled fault runner5
compiled assertion reds/exact restoration; strict native553/45groups and release20/WASM3 observed0.
Reference legacy9/14/18 and coupled4/8/10 authored sources/statements/operand identities; maxshape
256nodes/16if;12 header/four prefix refusals;20 unique actual fault anchors/noise controls pass0.
All product Rust sources compare byte-identically415d577, rc=0. Simultaneous4096 assertions×two256
nodes/16if on64KiB is syntax shape proof, not numerical execution or unbounded-memory certification.
Language16/publication9,51 chapters/21 APIs/1058 source/1636 rendered links pass0. D110 context
fixed/sealed with original report; D109 exact assert/recipe byte contract stays next .3f.1.
Ledger9 arms/13 pointer controls, archive28 arms/181 CLI controls/172 logical reads and retention
172 records/47 working Markdown/8639 decoded lines/668602 decoded bytes/324924 resident bytes0.
Fresh reconstructed defects11open/98unique sealed/zero overlap; tree10 lanes/13trees/nine siblings/
zero gaps, glossary310/nine/158, feature105/29, uncertainty133/16, fixture20/four/five all pass0.
Predecessor415d577 closure and oldest ledger/lesson payloads compare exact0; final whitespace check0.
Staged `make gate` → === all doctrines green ===, rc=0; all scoped jobs observed terminal.

## Commit Log

| Leaf | Commit subject | Verification |
| --- | --- | --- |
| `.5a.3e.3` | `STITCHCAD-G1-0067 (leaf G1-SLICE.5a.3e.3): complete coupled statement and recipe syntax review` | recipe11/five new reds/native553 |

## Changelog

- `2026-10-02`: coupled recipe syntax review closes .3e; D109 exact recipe bytes next .3f.1.
- promotion: declined (routine coupled syntax review and existing ADR clarification, no new policy).
