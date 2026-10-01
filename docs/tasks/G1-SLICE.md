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
  Status: `active`
  Goal: the geometry-bearing object types (ontology §4) — `Piece`, `SeamSpan`/`SewingGraph`, `Notch`,
  `Grainline`, `SeamAllowance`, `Dart`/`Tuck`/`Pleat`/`Gather`, `Closure`, `Pocket` — with their
  structural invariants enforced at construction.
  Acceptance: a structurally invalid object cannot be built and the diagnostic names the invariant
  (an empty or self-repeating boundary loop, a reference to a non-existent edge, multiplicity 0, a
  cut-on-fold piece without exactly one fold edge, incomplete label data); the GEOMETRIC invariants
  (CCW winding, simplicity, holes strictly inside, closure, dart-intake conservation) are carried as a
  visible `DeferredToG2` state discharged by `G2-2D.1`, never claimed here.
  Verification: `pending`
  Commit: `pending`
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
  Status: `active`
  Goal: semantic `Dart`/`Tuck`/`Pleat`/`Gather`, `Hem`, `Facing`/`Lining`/`Interfacing`, `Closure` and
  `Pocket` with required content and validated structural references; close the object-type parent.
  Acceptance: required anchors, operation identities, composition and parameter references are carried;
  buttonhole size derives from its button rather than a second input; unsupported constructions are
  refused explicitly; intake conservation stays visibly deferred to G2. Every ontology §4 object has
  implementation evidence, synchronized book content and wasm-safe tests.
  Verification: `pending`
  Commit: `pending`

  Children: `.3c.4a` (intake constructions), `.3c.4b` (hem/layers), `.3c.4c` (closures),
  `.3c.4d` (pockets and family signoff). Safe child slices preserve the entire §4.3/§4.7 scope.

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
  Status: `active`
  Goal: Pocket position, orientation, opening and component Piece references; complete construction
  family signoff and close `.3c` after all child types have structural evidence and book coverage.
  Acceptance: every component exists, composition required, owned placement/direction references,
  symbolic opening resolution and execution scope remain G3 obligations; named envelope diagnostics
  must be consumed before execution, not replaced by approximation. All §4 families accounted for.
  Verification: `pending`
  Commit: `pending`

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
  Status: `pending`
  Goal: re-derive all geometry-bearing object families against roadmap/ontology contracts, tests,
  immutable interfaces, current repair behavior, deferred obligations and mdBook implementation index.
  Acceptance: every child structurally complete with owned later obligations; full milestone checks/
  probes green; no geometric or unsupported-envelope support claim inferred from modelled metadata.
  Close `.4d`, `.3c.4`, `.3c` and top-level `.3` only after all structural criteria are verified.
  Director reaffirmed SOTA, signoff and production-grade as the bar on 2026-10-01. Structural
  completion is not production certification: geometry/interoperability/reliability and independent
  review gates retain their owned proofs; measured evidence, not API presence, earns signoff.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.4`
  Status: `pending`
  Goal: `sc-measure` — MeasurementTable (body vs garment POM, landmarks, source, procedure),
  Ease as a first-class body→garment mapping with fit intent, and SizeSet (implements
  `G0-CONTRACT.4`/`.6`).
  Acceptance: a POM without a landmark or procedure is rejected; ease is queryable per POM;
  size labels vs order vs base size are distinct fields with tested semantics.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.5`
  Status: `pending`
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
| — | `G1-SLICE.3c.4d.2` | `pending` | Re-derive construction/object-family structural signoff |

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

### `G1-SLICE.13` (acceptance rewritten by `G0-CONTRACT.11`) — a consumer leaf names its instrument, not a protocol in prose

This tree had no completed leaf, so it carried no acceptance boxes — and a staged `docs/tasks/*.md` file
with no ticked box is refused by `scripts/check_task_acceptance.sh` whenever the same commit stages code.
These boxes are the evidence for the change `G0-CONTRACT.11` made to this tree, added in the commit that
made it, which is the remedy `G0-CONTRACT.4c` used for `G3-GRADING.md`.

- [x] **REPRODUCE / ISSUE** — `.13`'s acceptance pointed at a protocol that did not exist anywhere in the
  repository: `git show HEAD:docs/tasks/G1-SLICE.md | grep -c 'the measurements the protocol named are
  recorded'` → `1`, `rc=0`, while `git ls-tree HEAD docs/decisions/ | grep -c adr-0002` → `0`, `rc=1`. A
  leaf whose acceptance names an absent document cannot be verified, and the spike it owns is the one the
  roadmap warns must not be argued after the fact.
- [x] **ROOT CAUSE (WHY + WHERE)** — the protocol is `G0-CONTRACT.11`'s deliverable
  (`grep -n 'ADR-0002 — UI stack' ROADMAP.md` → `328`, `rc=0`), and that leaf had not been taken, so the
  consumer was written against an intention. The fix is not a better sentence in this tree: it is the
  protocol existing as an instrument whose output this leaf's acceptance can cite.
- [x] **ADDRESSED (verified)** — `.13`'s acceptance now names the instrument by path, the corpus by its
  declared numbers (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000 fidelity
  round trips), and the evidence obligations a verdict is scoped to (reference hardware, OS versions, corpus
  script identity), and requires a recorded decision naming the rows for any human overruling the printed
  verdict. The instrument exists and reports the honest state:
  `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` → `spike verdict: 0 rows / 0 profiles /
  PENDING — ADR-0002's canvas half stays `proposed` until G1-SLICE.13 records measurements / 0 refusal(s)`,
  `exit=0`; `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh`
  → `probes: 13 pass / 0 fail`.
- [x] **NO REGRESSION** — `scripts/check_doctrines.sh` → `=== all doctrines green ===` once this section
  existed (before it, the same command refused this file by name: `TASK-ACCEPTANCE: docs/tasks/G1-SLICE.md
  has no 'ROOT CAUSE' box in its acceptance checklist`, `exit=1`, which is the gate working rather than a
  defect in it); `make probes` → `18 suite(s) green`; `bash
  docs/tasks/artifacts/planning/run_tree_coverage_census.sh` → `census: 10 lanes / 13 trees / 3 sibling(s)
  / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`. No Rust file changed in this slice, so `make
  check` is not its gate; the two new instruments are bash and both were run.
- [x] **FIX** — rewrote `.13`'s acceptance to consume the protocol as an artifact, and added this
  subsection so the tree file carries fresh evidence for the change it stages.
- [x] **LOCKSTEP** — `G0-CONTRACT.md`'s leaf `.11`, its frontier, decisions, three logs and checklist;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md`,
  `docs/decisions/INDEX.md` and the regenerated Knowledge Map, all in this commit.

## Verification Log

[Completed verification through Hem](G1-SLICE-evidence.md#historical-verification-log) is preserved
unchanged in the evidence sibling; fresh current-slice checks remain here.

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-10-01` | `.3c.4c.1a` | notion contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `10 passed`; four mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4c.1b` | closure contracts; Count domain; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `10 passed`; four mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4c.2` | closure contracts; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `15 passed`; two source mutations red; restored checks/gates green, `rc=0` |

| `2026-10-01` | `.3c.4d.1` | Pocket; check; wasm; book; fixture/feature/glossary/tree; ledger; gate | `11 passed`; three mutations red; restored checks/gates green, `rc=0` |

## Commit Log

[Completed commits through physical placements](G1-SLICE-evidence.md#historical-commit-log) are
preserved unchanged in the evidence sibling; fresh current-slice entries remain here.

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.3c.4c.1b` | `STITCHCAD-G1-0020 (leaf G1-SLICE.3c.4c.1b)` | zipper/hook-bar instances, derived Count, current targets, env_fly |

| `.3c.4c.2` | `STITCHCAD-G1-0021 (leaf G1-SLICE.3c.4c.2)` | canonical buttonhole source; Closure parent closed |

| `.3c.4d.1` | `STITCHCAD-G1-0022 (leaf G1-SLICE.3c.4d.1)` | physical Pocket composition, placement, explicit opening deferral |

## Changelog

[Completed task changelog through zipper/hook-bar](G1-SLICE-evidence.md#historical-task-changelog)
is preserved unchanged in the evidence sibling; new changes are recorded here.

- `2026-10-01`: `.3c.4c.2` preserves the completed closure checklist and task changelog unchanged;
  independent committed-payload comparison passes. Active contracts retain their evidence pointers.

- `2026-10-01`: `.3c.4c.2` lands button/hole pairs with one canonical size/operation source; Closure
  parent closes structurally. Physical hole derivation remains G3; next `.3c.4d` pockets/signoff.

- `2026-10-01`: `.3c.4d.1` implements Pocket physical composition/placement intent and visible opening
  deferral; next `.3c.4d.2` re-derives structural family signoff against the director's quality bar.
