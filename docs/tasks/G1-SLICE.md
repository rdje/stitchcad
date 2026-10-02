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
  Goal: borrowed ASCII lexical stream for the normative machine-form formula syntax; explicit
  lexical kind, original text and byte span, with typed first-error refusal and fused termination.
  Pre-code protocol: sc_core::recipe owns the syntax front-end. Immutable FormulaLexeme and
  FormulaSourceSpan retain exact source text/positions; FormulaLexer scans borrowed input without
  cloning names/numbers or interpreting values. Classify three keywords using one shared private
  name-module classifier; preserve all existing MachineToken public constructor/error behavior.
  ASCII preflight reports the first full offending Unicode scalar before producing tokens. General
  ASCII whitespace is ignored between tokens but span gaps remain; the later parser enforces the
  literal's exact single-space unit separator. Identifier spelling uses the same allocation-free
  lower-snake predicate as MachineToken. Number tokens retain digits/optional nonempty fraction,
  with no numeric conversion/rounding/implicit unit. Punctuation/operators include longest paired
  comparisons; malformed identifiers, decimal fraction, lone ! and unsupported characters refuse.
  After first error/end, iterator remains ended. Debug of lexer reveals scope/position, not source.
  Lexical success certifies only tokens: adjacent atoms, comments assembled from slash operators,
  unsupported calls/exponents/units, type/name errors and recipe bounds retain parser/checker owners.
  Own native/WASM tests, borrowing/privacy docs, every current worked example's machine-token scan,
  actual guard mutations, book/public-status map/index, canonical ADR clause-link fix D73, bounded
  live/evidence seals and commit. No evaluation, canonical formula identity or app/MCP claimed.
  Verification: 13 lexical contracts/two privacy-lifetime docs; nine actual assertion reds/exact restore;
  454 strict native tests, WASM/book; publication nine, formula 15 and ledger nine probes green.
  Commit: `STITCHCAD-G1-0037` (this recording commit).

- ID: `G1-SLICE.5a.2`
  Status: `pending`
  Goal: expression syntax/precedence/conditional trees with node/depth bounds and precise parse
  refusals; finalize the full pre-code protocol before implementation, preserve static/runtime split.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5a.3`
  Status: `pending`
  Goal: exact literal normalization and canonical S-expressions, ordered binding/assert recipes and
  statement limits. Finalize numeric/identity protocol from normative contract before code.
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
| next | `G1-SLICE.5a.2` | `pending` | Expression-tree protocol/precedence/bounds; D70 remains pending |

## Routing Evidence — D72, milestone fixture calibration

Full `make probes` reaches history_archive_probes.py's resident-overflow arm and fails with
`decoded history aggregate bound exceeded`, not its expected resident error. Inventory at 285e238
is 486538 decoded bytes; 22 × 160000 makes 4006538, above the unchanged 4000000 limit. The reader's
ordering/cap is correct. This reproduces outside garment logic in the archive test fixture. It is
kept in .4d.1 as a small prerequisite of this publication milestone, not routed to an unowned report;
production archive retention and SPINE.19's other obligations are unchanged.

## Routing Evidence — D67, lesson decision freshness

`git diff b3b9e3a^ b3b9e3a -- docs/decisions` adds no answers line; the staged SPINE file at that
revision carries a routine cleanup decline (line 620), and the commit log shows promotion green.
`check_lesson_promotion.sh` lines 93–101 scan whole staged task files, so the unrelated historical
lesson is the cause. This is outside measurement logic and reproduces in SPINE's cleanup evidence;
SPINE.22 owns the project-slot adapter. Current metadata/token questions are explicitly added now.

## Decisions

- `2026-10-01`: D35/D57 settled and implemented by `.3c.2b`: physical copies have stable ids
  (director ruling), spans address those copies, and same-copy seams are permitted only for disjoint
  current positive-length interiors. Explicit correspondence/ease/stops remain separate from journal
  traversal and physical reflection. `decision_sewing-spans-address-copies-and-permit-disjoint-self-seams.md`
  records the contract before code; G2/G3/G4 owns geometry/realized ease/profile values.

- `2026-10-01`: reviewing the reference skirt before spans reproduces D56: its separate cut-once
  L/R members cannot be represented by `.3c.1`'s total-quantity pair mode. `.3c.1a` corrects that
  omission before proceeding. D57's physical-copy addressing is a genuinely unspecified graph
  contract, asked of the director; independent piece/mark work can proceed while it is pending.

- `2026-10-01`: `.3c.2` has two children: full-range identity resolution (`.3c.2a`, fixing D55),
  then the sewing graph and same-piece seam contract (`.3c.2b`, fixing D35). The range-fold decision
  precedes implementation and preserves the distinction between coverage, endpoint resolution and
  geometric validity. A complete interval cannot be certified by sampling, even at both endpoints.

- `2026-10-01`: `.3c` is decomposed into four child leaves before implementation: pieces, sewing
  graph, marks/allowances, garment constructions. The parent preserves its full ontology §4 scope;
  each child is committed and verified independently. Directed loops encode cyclic ordering, not a
  claim about endpoint closure. Printed cut quantity, pair and fold fields derive from the piece's
  cut plan so the label cannot contradict it. Parameter states remain G4's contract; later children
  carry parameter identities until then rather than manufacture defaults.

- `2026-10-01`: `.3b`'s design boundary is recorded BEFORE the code, per the `.3` decomposition's discipline:
  `decision_reference-resolution-journal-fold.md` — a stored reference is never rewritten, resolution is a
  pure fold of an append-only edit journal, repair state is derived not stored, the offset contract consumes
  declared intervals until G2 geometry supplies real ones, and undo (`.6`) becomes journal algebra. Recorded
  so `.3c`/`.6`/`.7` inherit it rather than re-litigate.
- `2026-09-29`: leaves are numbered in dependency order (units → ontology → measure → recipe →
  bus → store → CSP → API/MCP → CLI → profiles → spikes), because every later leaf consumes the
  earlier ones and a frontier that jumps is a frontier that stalls.
- `2026-09-29`: the dev shell (`.14`) is a deliverable of G1, not G2, so the G2 viewer leaf is
  not blocked on the hardest integration in the repository (ADR-0002's stated reason).
- `2026-09-30`: property tests are dependency-free and hand-rolled with a recorded seed, not
  `proptest`/`quickcheck` — recorded in
  `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` so later crates do not
  re-litigate it. This resolves the Open Question; the choice was made by `G0-CONTRACT.18` when
  `sc-units`' suite landed, because that crate must stay dependency-free for `wasm-viewer`.
- `2026-09-30`: `.3` (the ontology) is **three slices, not one** — `.3a` the identity types, `.3b` the
  persistent-identity contract, `.3c` the geometry-bearing object types — because each is a
  signoff-quality unit and they are strictly ordered (the contract consumes the types; the objects
  consume both). Its three design boundaries are recorded **before any code**, so each implementation
  slice builds against a fixed design: `decision_entity-identity-ulid-injected-generator.md`,
  `decision_edge-parameter-bounded-exact-rational.md`,
  `decision_ontology-invariants-structural-g1-geometric-g2.md`.

## Open Questions

- Which spike runs first, browser (`.12`) or canvas (`.13`)? They share fixtures; decided at gate
  entry, and `.12` is the gate's named exit clause so it wins a tie.

## Blockers

- None intrinsic. Entry depends on `G0-CONTRACT` and `SPINE.8`.

## Acceptance Checklist

Future ontology implementation updates run `run_glossary_census.sh` as a focused check,
including chapter-local API declarations (D59), in addition to feature coverage and book rendering.

Completed identity and earlier object checklists are preserved in
[`G1-SLICE-evidence.md`](G1-SLICE-evidence.md). Current/recent slice evidence stays below.

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and
mechanically required to be fresh in that commit by leaf `SPINE.8`. A tree file carries no
unticked placeholder boxes: the spine's acceptance gate judges the FIRST matching box in the
file, so a placeholder both shadows real evidence and falsely rejects honest work (defect D15,
measured by the `SPINE.7` probe).





### `G1-SLICE.5a.1` — borrowed lexical syntax with explicit parser boundaries

- [x] **REPRODUCE / ISSUE** — complete formula contract/grammar/examples and ADR read before
  lexical protocol/subleaves. No recipe module existed at predecessor 9ef9602. D73 ADR names absent
  examples/exclusions clauses. First gap contract finds D74: 12 pass/1 fail, rc=101, byte 11 refused.
- [x] **ROOT CAUSE (WHY + WHERE)** — `cargo test -p sc-core --test formula_lex_contract` →
  UnsupportedCharacter at bytes 11..12, 12 passed/1 failed, rc=101; lexical whitespace helper omits
  vertical tab. `rg -n '§10|§11' docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md`
  found two obsolete clause pointers, rc=0; main headings are §1–§9 and actual exclusions §6.
  Missing lexical front-end is a planned feature; it must borrow source and carry no evaluation authority.
- [x] **FIX** — immutable token/kind/span and typed source-private first-error/fused scanner; shared
  spelling/three-keyword classifiers retain MachineToken behavior. Full ASCII preflight precedes tokens;
  number spelling and whitespace gaps retain later parser authority. Added vertical tab, corrected ADR
  clause links, contract/privacy tests, isolated actual source mutations and indexed syntax annex.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test formula_lex_contract` → 13 passed,
  0 failed, rc=0; all 17 bindings/four assertions/13 refusal forms consumed with explicit lexical scope.
  `bash docs/tasks/artifacts/formula_lex/run_formula_lex_mutations.sh` → nine actual assertion reds,
  rc=101 each, runner rc=0; both sources restored byte-identically. Two privacy/lifetime doctests pass.
  D73 targets exist and D74 gap regression remains green; exact logged descriptions seal to part15.
- [x] **NO REGRESSION** — `CARGO_HOME="$PWD/target/cargo-home" TMPDIR="$PWD/target/scratch" make check`
  → strict lint and 454 tests including docs green, rc=0; `make wasm` → three-library cross-build, rc=0.
  Formula-language probes 15 pass/0 fail, ledger nine pass/0 fail, publication nine pass/0 fail, rc=0.
  Feature/tree/glossary/uncertainty/fixture censuses green; source/rendered publication links verified.
  Staged `make gate` → all doctrines green, rc=0. Focused checks are appropriate for this syntax
  subleaf; full milestone belongs .5a.4. No remote-CI claim.
- [x] **LOCKSTEP** — roadmap language unchanged; .5a.2/.3/.4 own parser/canonical/bounds completion.
  Learning/availability/module/package/README/status map/index and detailed API annex state actual
  lexical scope. Forty-eight chapters/15 API rows, 991 source/1506 rendered links checked. Exact
  predecessor publication contract/checklist and oldest ledger payloads preserved; no cap raised.
  D70 required axes ruling stays pending. G1 remains 5/18 top-level leaves with .4/.5 structurally partial.

## Verification Log

| `2026-10-02` | `.5a.1` | lexical contracts/docs; actual mutation/restoration; strict native/WASM; book/publication/language/censuses/ledger/archive/staged gate | 13 contracts/two docs; nine assertion reds; 454 tests; focused checks green, rc=0 |

| `2026-10-02` | `.4d.1` | book/source/render/status/index; nine publication probes; archive/control; full milestone/censuses/ledger/staged gate | 47 chapters/14 APIs, 983/1489 links, 28 archive arms, 24 suites green; D71/D72 fixed |

| `2026-10-02` | `.4c.3c` | MTM contracts/mutations; strict Rust/WASM/book; current-reference censuses/ledger/staged gate | 15 contracts/two docs, twelve reds, 439 tests; focused checks green, rc=0 |

| `2026-10-02` | `.4c.3b` | collection contracts/mutations; strict Rust/WASM/book; glossary/uncertainty/tree/feature/ledger/gate | 18 contracts/privacy, fourteen reds, 422 tests; all checks green, rc=0 |

| `2026-10-02` | `.4c.3a` | chart contracts/mutations; Rust/WASM/book; glossary/uncertainty/tree/ledger/staged gate | 16 contracts/privacy; eight assertion reds; strict 403 tests, all checks green, rc=0 |

| `2026-10-02` | `.4c.1` | membership contracts/guard mutations, strict Rust/WASM/book, glossary/uncertainty/tree/ledger, staged gate | 12 contracts/three docs, seven real reds, 386 tests; 10 open/59 sealed; D70 ruling pending |

| `2026-10-02` | `.4b.3` | ontology/API/test field review, current measure tests, full probes/book, ledger/staged gate | 65 current tests/docs, 23 suites green; unchanged b4e0bc7 product code; .4b structural closure |

| `2026-10-02` | `.4b.2` | Rust/WASM/book, set guards, glossary/tree/ledger/staged gate, metadata/defect census | 371 tests, 14 contracts/privacy, ten real reds; 9 open/59 sealed, rc=0 |

| `2026-10-02` | `.4b.1` | strict Rust/WASM/book; seven guard mutations; restored Ease/table tests; glossary/tree/ledger/staged gate | 356 tests, 13 Ease contracts/privacy, 7 real reds; all focused checks green, rc=0 |

[Completed verification through Hem](G1-SLICE-evidence.md#historical-verification-log) is preserved
unchanged in the evidence sibling; fresh current-slice checks remain here.

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-10-02` | `.4a.3` | table contracts; eight guard mutations; check/wasm/book/censuses; milestone probes/staged gate | 16 contracts + privacy; 342 strict tests; 23 full suites; table field review; staged gate below |
| `2026-10-01` | `.4a.2c` | pushed bf29b03; Actions runs/jobs/steps; metadata review; book/censuses/ledger/gate | both CI jobs/all steps success; scoped metadata parent closed; local green, `rc=0` |
| `2026-10-01` | `.4a.2b` | metadata contracts; check/wasm/book; full probes/ledger/gate | `16 passed`; six real mutations red; `325` tests; green, `rc=0`; CI .2c |
| `2026-10-01` | `.4a.2a` | token contracts; check/wasm/book; censuses/ledger; staged gate | `6 passed`; four real mutations red; `305` tests; green, `rc=0` |
| `2026-10-01` | `.4a.1` | value contracts; check/wasm/book; focused censuses/ledger; staged gate | `8 passed`; four red mutations; `298` tests; green, `rc=0` |
| `2026-10-01` | `.3c.4c.1a` | notion contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `10 passed`; four mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4c.1b` | closure contracts; Count domain; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `10 passed`; four mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4c.2` | closure contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `15 passed`; two source mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4d.1` | Pocket; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `11 passed`; three mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4d.2` | full check/wasm/book/probes; censuses/ledger; staged gate | `287` tests; `149` object contracts; `22 suite(s)` green; D62/D63 fixed, `rc=0` |

## Commit Log

[Completed task journal through garment chart collections](G1-SLICE-measurements.md#completed-g1-task-journal--preserved-from-e299771)
retains exact e299771 bytes; earlier journals remain linked there. Current entries follow below.

## Changelog


- `2026-10-02`: .4c.3c implements canonical custom-member MTM/body/Ease inputs. The test fixture's
  terminal compression error correctly has no further source; the test now checks the actual two
  scoped wrappers. Strict lint refused explicit fixture panic; typed matches assertion fixes it without
  suppression. All 439 strict tests pass; twelve source guard/fallback mutations fail assertions and
  restore source exactly. Main chapter teaches the waist example; reference/verification is in an annex.
  Director requires incremental book/glossary/index/annexes and continuing roadmap/code/book lockstep;
  .4d.1 owns publication review next and D71 stale landing status. D70 axes ruling remains pending.
- promotion: declined (routine MTM current-reference implementation; director's publication requirement
  remains durably task-owned for .4d.1's adoption, not a new MTM semantic decision).

| `.4c.3c` | `STITCHCAD-G1-0035 (leaf G1-SLICE.4c.3c): made-to-measure charts retain canonical body and Ease inputs` | 15 contracts/two docs, twelve reds, 439 tests/WASM/book; publication review next |

- `2026-10-02`: .4d.1 applies the director's incremental book policy, glossary/index and expert
  annexes. D71 status drift is fixed. D72 resident fixture is isolated from growing archive history
  and gains a positive control; full 24-suite milestone now passes. Roadmap amendment stays inside
  unchanged 951/53153 debt baseline by preserving its old D32 explanation exactly in that decision.
  Existing chapter URLs and clause anchors remain; public/source links and 14 structural API records
  are verified, not treated as complete semantic or production certification. Local file screenshot
  access is blocked by browser policy; HTML link/table evidence is inspected without a bypass.
- promotion: promoted by `decision_book-progression.md` (fresh answers/director requirement).

| `.4d.1` | `STITCHCAD-G1-0036 (leaf G1-SLICE.4d.1): the book teaches progressively with indexed expert annexes` | 47 chapters/14 APIs; nine publication probes; 28 archive arms; full 24 suites; D71/D72 fixed |

- Staged acceptance initially refused the ROOT CAUSE bullet's missing command-output signature;
  the actual first milestone invocation/refusal and rc=2 are now cited inside that bullet. The
  mechanical gate remains unchanged; final staged verification is rerun before committing.

- `2026-10-02`: .5a.1 introduces borrowed machine tokens/spans, preserving namespace and parser
  authority boundaries; D73 clause links/D74 whitespace corrected. Nine actual guard reds and exact
  restoration; strict 454 native tests/WASM/book, focused publication/language/censuses/ledger green.
  .5a.2 expression trees is the next product slice; D70 axes remains unanswered.

| `.5a.1` | `STITCHCAD-G1-0037 (leaf G1-SLICE.5a.1): formula lexing preserves borrowed machine source and precise spans` | 13 contracts/two docs; nine actual reds; strict 454 tests/WASM; publication/language/ledger green |
Lesson promotion declined for .5a.1: routine lexical mechanics; language/canonical authority stays ADR-0003.
