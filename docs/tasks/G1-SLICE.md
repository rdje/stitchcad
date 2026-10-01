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
  Status: `active`
  Goal: `Notch`, directed and dual-reference `Grainline`, per-edge derived `SeamAllowance`; profile
  parameters are identity references rather than invented defaults until G4 supplies parameter states.
  Acceptance: references exist at construction; notch export geometry and encoding remain profile-owned;
  unknown profile parameters are never replaced by a value; directed references survive reversal or
  show repairs, and allowance inclusion is resolved per profile rather than as a global switch.
  Verification: `pending`
  Commit: `pending`
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
  Status: `pending`
  Goal: per-edge derived allowance descriptors, authored width-parameter origin, explicit corner choice
  and symbolic per-profile inclusion policy; then close the marks/allowances parent.
  Acceptance: live owned edges required; width may come from formula/profile/explicit parameter
  entities without duplicating their uncertainty state; corner vocabulary matches §4.4; inclusion is
  resolved per target profile; offset geometry/error-budget checks remain visibly G2 obligations.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.3c.4`
  Status: `pending`
  Goal: semantic `Dart`/`Tuck`/`Pleat`/`Gather`, `Hem`, `Facing`/`Lining`/`Interfacing`, `Closure` and
  `Pocket` with required content and validated structural references; close the object-type parent.
  Acceptance: required anchors, operation identities, composition and parameter references are carried;
  buttonhole size derives from its button rather than a second input; unsupported constructions are
  refused explicitly; intake conservation stays visibly deferred to G2. Every ontology §4 object has
  implementation evidence, synchronized book content and wasm-safe tests.
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
| — | `G1-SLICE.3c.3c` | `pending` | allowance descriptors; directed grainline content landed |

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

Completed `.1`, `.2`, `.3a` and `.3b` checklists are preserved in
[`G1-SLICE-evidence.md`](G1-SLICE-evidence.md). Current/recent slice evidence stays below.

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and
mechanically required to be fresh in that commit by leaf `SPINE.8`. A tree file carries no
unticked placeholder boxes: the spine's acceptance gate judges the FIRST matching box in the
file, so a placeholder both shadows real evidence and falsely rejects honest work (defect D15,
measured by the `SPINE.7` probe).

### `G1-SLICE.3c.3b` — directed grainlines and independent print references

- [x] **REPRODUCE / ISSUE** — ontology §4.6 requires direction, explicit angular intent and dual
  print references; `ea631c6` has no grainline object. Directed arrow semantics cannot collapse to
  endpoint-only identity or an undirected axis. D61 misstates the glossary declaration scope.
- [x] **ROOT CAUSE (WHY + WHERE)** — ordered interval traversal and authored direction are distinct
  from point fate and journal direction. `cargo test -p sc-core --test grain_contract
  reversed_traversal_orders_split_fragments_and_interior_repairs_from_its_own_start` → `1 passed`,
  `rc=0`: reverse traversal visits the second split fragment first and retains a lost middle repair
  in traversal position. `sed -n '388,426p' docs/tasks/artifacts/glossary/run_glossary_census.sh`
  → a global DECLARED_TOKENS set supplies C1, locating D61's diagnostic-only discrepancy, `rc=0`.
- [x] **FIX** — immutable Grainline with directed arrow, explicit alignment/angle source and optional
  independent stripe/plaid ranges; all fields require complete owned intervals and unique endpoints.
  Directional views reverse fragment order and compose reversal without rewriting raw range evidence.
  Reuse the tested ownership fold; expose G2 geometry and G4 profile-value obligations explicitly.
  Correct D61's diagnostic scope without changing predicates or exemptions.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test grain_contract` → `9 passed`, `rc=0`;
  opposite arrows, reversed split/repair order, all field scopes, ambiguous endpoints, merged foreign
  remainder after reversal, explicit/symbolic angles, optional metadata and immutability discriminate.
  Disable reversed next_back → order regression fails, `rc=101`; restored `make check` passes.
  Glossary probes → `10 pass / 0 fail`, `rc=0`; D61 closes in defects-part8. The partition oracle
  compares the old executable body to the linked chapter → `231 lines / 18018 bytes unchanged`, `rc=0`.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust suites + privacy green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; feature/glossary censuses → `0 failure(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  Existing eighteen sewing and seven notch contracts remain green after ownership sharing.
- [x] **LOCKSTEP** — existing structural/geometric decision extended before code; module/subsystem
  status, ontology bounded §10 index + linked implementation examples/SUMMARY, feature matrix,
  G1 frontier/evidence/logs, TASK_TREE, MEMORY, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES.
  D61 seal/pointer; changelog-part17/devnotes-part12 preserve oldest live entries. Next `.3c.3c`.

### `G1-SLICE.3c.2b.2` — copy-addressed sewing spans; D35/D57 verified closed

- [x] **REPRODUCE / ISSUE** — D57 requires physical-copy addressing so cut-two copies can have
  different neighbours; D35 needs an explicit same-copy sewing rule before the type exists.
  At `9ba32e0`, copy plans exist but no graph does. The canonical folded band's short-end finish
  is the relevant same-copy case; G1 must not invent its unconstructed numeric G2 ranges.
- [x] **ROOT CAUSE (WHY + WHERE)** — Piece identity alone conflates physical material domains;
  different held edge names alone cannot certify self-seam disjointness. `cargo test -p sc-core
  --test sewing_contract self_seam_disjointness_is_checked_after_merge_instead_of_by_edge_name`
  → `1 passed`, `rc=0`: two held edges resolve to disjoint current intervals, while a new merged
  overlapping range is refused before/after reversal. The separate hidden-interior ownership
  regression expects refusal of an unowned middle third despite owned endpoints.
- [x] **FIX** — immutable SewingGraph/SeamSpan with copy ids, exact positive ranges, explicit
  correspondence, signed ease source/distribution and side-specific semantic stop ids. Validate
  the complete plan, scope identities, interval ownership/coverage, endpoint uniqueness, disjoint
  self-seams, stops and weighted/notch-anchored domains. Shared born-valid anchor validation supplies
  TurnPoint without changing the NotchError API alias. Queries preserve held content and missing ids.
  Geometry/realized ease and formula/profile value resolution remain explicit later obligations.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test sewing_contract` → `18 passed`,
  `rc=0`: copy-specific neighbours, partial/one-to-many spans, disjoint self-seams and overlap
  after merge/reversal, complete ownership, interior repairs and endpoint choices, stop/ease
  domains and missing replacement targets discriminate. Self-overlap and ownership refusals each
  disabled → their independent regression red (`rc=101`); restored source passes. D35/D57 close
  in defects-part7; graph privacy doctest passes, and all seven existing notch contracts pass.
- [x] **NO REGRESSION** — `make check` → fmt/strict clippy/all Rust suites green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; fixture → `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`; feature/glossary censuses → `0 failure(s)`; ledger probes →
  `9 pass / 0 fail`; staged `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The prior copy milestone's full `make probes` → `22 suite(s) green`, `rc=0`; no golden numbers change.
- [x] **LOCKSTEP** — pre-code sewing decision + INDEX, Rust module docs, ontology §4.2/§10 +
  local API vocabulary, feature matrix, fixture's now-settled self-seam record, task status/parents/
  evidence/frontier/logs, index, MEMORY, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES. D35/D57
  closure descriptor + live pointer; changelog-part16/devnotes-part11 seal oldest entries atomically.
  The map keeps paths/owners after shortening its glossary orientation. Next `.3c.3b` is grainlines.

### `G1-SLICE.3c.2b.1` — explicit physical-copy identities, quantities and orientations

- [x] **REPRODUCE / ISSUE** — D57's director ruling (`2026-10-01`) requires stable physical-copy
  identities so copies of a cut-two Piece can have different neighbours. At `6abfac3` only Piece
  quantity exists. The glossary census against that committed snapshot also reports `15 failure(s)`
  (`rc=1`, D59), showing existing API terms lacked vocabulary ownership before this slice.
- [x] **ROOT CAUSE (WHY + WHERE)** — pattern identity/quantity cannot select one physical copy;
  ordinal-derived ids would change under reordering. `cargo test -p sc-core --test cut_contract
  copies_retain_explicit_identity_when_reordered_and_geometry_stays_deferred` → `1 passed`, `rc=0`:
  the explicit-id oracle retains both copy definitions across reversed list order. The committed
  snapshot's `GLOSSARY_ROOT` census → `15 failure(s)`, `rc=1`, pins D59 to undeclared API tokens
  in ontology prose, not an index or glossary-term duplication.
- [x] **FIX** — immutable `CutPlan`/`CutCopy`, editable explicit ids/source Piece/orientation.
  Validate unique/disjoint identities, known Pieces, exact quantity and equal authored/reflected
  populations for MirroredPairs; authored-only modes refuse reflection. No id is minted or reassigned.
  Geometry stays DeferredToG2; companion and global design checks remain `.6`. Genuine new concepts
  enter the glossary; the chapter-local API table declares implementation names without exemptions.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test cut_contract` → `9 passed`, `rc=0`:
  identity/reorder/replay, duplicates/collisions, unknown/unmentioned Pieces, exact quantities,
  mirroring, explicit replacement and empty plan contracts pass. Disabling quantity refusal turns
  its regression red (`rc=101`), restored source passes. Glossary census → `310 terms / 9 parts /
  158 tokens / 0 failure(s)`, `rc=0`; D59 closes in defects-part5. D60 closes in defects-part6 after historical-root revalidation;
  staged `make gate` → all doctrines green, `rc=0`. D57 awaits graph integration.
- [x] **NO REGRESSION** — `make check` → strict clippy, all suites and three core privacy doctests
  green; `make wasm` → green; `make book` → warning-free; feature/glossary censuses → `0 failure(s)`;
  tree coverage → `10 lanes / 13 trees / 4 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The original moved payload was byte-identical before D60 added revalidation evidence to the
  historical doc-only `.1`/`.2` ROOT CAUSE bullets; all four checklists are validated separately.
- [x] **LOCKSTEP** — copy-identity decision + INDEX, source/module docs, ontology §4.2/§10,
  feature matrix, two glossary entries + derived A–Z index, G1 task evidence sibling/status/logs,
  task index, MEMORY, LIVE_STATUS, CHANGELOG, promoted DEV_NOTES, closed D59/D60 descriptors/live pointers;
  devnotes-part10 seals the oldest lesson. Next `.3c.2b.2` implements copy-addressed sewing spans.

### `G1-SLICE.3c.3a` — semantic notches; physical profile bindings stay unresolved

- [x] **REPRODUCE / ISSUE** — ontology §4.5 requires semantic edge/parameter anchoring with
  profile-owned style, sample/production dimensions and encoding. At `486607b` no `Notch` exists.
  D58's dependent G4 acceptance says default-plus-sidecar despite its no-default Goal and release §8.
- [x] **ROOT CAUSE (WHY + WHERE)** — G1 needs semantic identity before G4's target-profile schema
  exists; copying physical values would invent facts and duplicate parameter state. Ownership must
  compare current ranges and current positions, not only live edge ids. `cargo test -p sc-core
  --test notch_contract ownership_uses_surviving_partial_ranges_and_resolved_parameters` → `1 passed`,
  `rc=0`: the independently expected owned `[0,3/10]` portion refuses the merged foreign remainder,
  including after reversal. `run_release_contract_census.sh` → `8 matrix rows / 0 failure(s)`, `rc=0`
  identifies the canonical matrix D58's Acceptance must follow.
- [x] **FIX** — immutable `Notch`, editable `NotchDefinition`, exact `EdgeAnchor`, symbolic
  `ProfileParameterRef` and complete sample/production/style/encoding bindings. Construction requires
  a live unique owned point; edits return the original ledger's choices/repairs without mutations.
  `ProfileBindingValidation::DeferredToG4` exposes the missing physical validation. G4 acceptance
  now names badge preview / sidecar draft / block production and no defaults; G4 still owns execution.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test notch_contract` → `7 passed`, `rc=0`:
  boundary/hole/construction anchors and endpoints accepted, absent/foreign anchors refused, exact
  reverse/merge recomputation, split-side choice, deletion operation/held-reference evidence and
  preserved symbolic fields tested. Disabling ownership refusal makes the partial-merge test fail
  (`rc=101`); restored source passes. The privacy doctest also passes. D58 closes in defects-part4.
- [x] **NO REGRESSION** — `make check` → fmt, strict clippy, all existing/new suites green, `rc=0`;
  `make wasm` → green; `make book` → warning-free; feature/release censuses → `0 failure(s)`;
  ledger probes → `9 pass / 0 fail`; `make gate` → `=== all doctrines green ===`, all `rc=0`.
  The style vocabularies implement no geometry and do not widen the staged release envelope.
- [x] **LOCKSTEP** — binding decision + INDEX, Rust module docs, ontology §10 + feature matrix,
  G4 dependent acceptance, task status/frontier/evidence/logs, index, MEMORY, LIVE_STATUS,
  CHANGELOG and promoted DEV_NOTES; D58 closure + descriptor and live pointer. Oldest ledger
  entries seal in changelog-part15/devnotes-part9; map orientation is tightened and regenerated.
  D57 was answered before commit: stable identities per physical cut copy. Next `.3c.2b` implements
  that contract and sewing spans; D57 closes only after verification.

### `G1-SLICE.3c.1a` — the canonical fixture's separate cut-once L/R members (D56)

- [x] **REPRODUCE / ISSUE** — `reference-skirt.md` §6 declares separate `skirt_back_right` and
  `skirt_back_left` Piece identities, quantity one each and reciprocal pairing. At `0a64a17`,
  `piece.rs`'s mirroring enum has only `Single` and even-total `MirroredPairs`; it cannot carry a
  cut-once member's hand/companion. Applying an even-total restriction to every pair mode makes
  `the_reference_skirts_cut_once_pair_members_print_their_own_handedness` fail (`rc=101`).
- [x] **ROOT CAUSE (WHY + WHERE)** — the first implementation conflated two pairing forms: one
  definition requesting both hands, and two separately identified members. The glossary's mirrored
  pair and the fixture's explicit Piece rows are the independent authorities. `cargo test -p sc-core
  --test piece_contract the_reference_skirts_cut_once_pair_members_print_their_own_handedness` →
  `1 passed`, `rc=0`: preserving cut-one quantities requires explicit member metadata, not name parsing.
- [x] **FIX** — `Mirroring::PairMember { handedness, companion }` and `Handedness::Left/Right`;
  this mode counts only this member's copies. `PieceError::SelfCompanion` rejects a member naming
  itself. The existing even-total mode is unchanged. Complete printed labels derive both forms
  from their cut plan. `.6` owns collection-level reciprocity/opposite-hand/equal-quantity checks;
  G2 owns geometric mirroring. D57's copy-address question stays separate and unanswered.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test piece_contract` → `14 passed`,
  `rc=0`: the fixture-shaped separate cut-one L/R members retain reciprocal ids and exact label
  metadata; self-companion is the expected typed refusal, and existing even-total and odd-total
  cases still discriminate. Reinstating the overbroad even-total condition makes the new fixture
  regression red (`rc=101`); restored source passes. D56 is sealed closed in defects-part3.
- [x] **NO REGRESSION** — `make check` → fmt, clippy `-D warnings`, all existing suites and new
  pair-member tests green, `rc=0`; `make wasm` → cross-build green; `make book` → warning-free;
  fixture derivation → `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  feature census → `0 failure(s)`; ledger probes → `9 pass / 0 fail`; `make gate` →
  `=== all doctrines green ===`, all `rc=0`. No fixture cut quantity, pairing or geometry was edited.
- [x] **LOCKSTEP** — pairing decision + INDEX, ontology §10 and label examples, Rust type docs,
  task evidence/frontier, task index, resume pointer, LIVE_STATUS, CHANGELOG and promoted DEV_NOTES
  lesson; closed D56 pointer and sealed descriptor, open D57 with the exact census and pending
  director question. The generated map retains orientation paths/owners after its units entry is
  shortened. Next `.3c.3` is independent work while the sewing-copy decision is pending.

### `G1-SLICE.3c.2a` — whole-range resolution, fixing D55 before sewing spans

- [x] **REPRODUCE / ISSUE** — D55's tracked point-only counterexample at `0d7a4a5` leaves both held
  endpoints resolved after deleting the middle of three fragments. Its ledger verdict is still
  `Releasable`, correctly scoped to point registrations. `cargo test -p sc-core --test piece_contract
  endpoint_inventory_cannot_certify_the_interior_of_an_edge_range` → `1 passed`, `rc=0`: sampling
  cannot prove an interval's integrity, and the new piece assertion now demands a visible range repair.
- [x] **ROOT CAUSE (WHY + WHERE)** — the point fold answers one parameter's fate, while a sewing span
  holds an interval; the error is treating those as the same quantity. `cargo test -p sc-core --test
  range_contract arbitrarily_narrow_trimmed_gaps_cannot_escape_the_interval_fold` → `1 passed`, `rc=0`:
  every hundredths-grid point resolves while the exact uncovered interval is returned as a repair.
  The solution is interval partition through the journal, recorded before code in
  `decision_range-resolution-preserves-entire-interval.md`, not adding more sampling points.
- [x] **FIX** — `ontology::range`: `EdgeRange`, `RangeResolution`, `ResolvedRange`, ordered
  `RangePortion`s, typed `RangeRepairTask`/`RangeIssue`, and the exact whole-interval fold on the
  existing public journal. Split/offset use interval intersection (offset gaps retained); merge uses
  declared arc lengths; reversal preserves authored traversal; deletion and arithmetic refusal retain
  visible tasks. `Piece::range_resolutions` exposes the full-edge evidence. Point API unchanged.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test range_contract` → `13 passed`, `rc=0`:
  D55 returns a task naming the original held range, deleted middle fragment and delete operation,
  despite resolved endpoints; partial ranges, merge/reverse, offset order/gaps, missing sources and
  overflow are checked by exact payload. Generated scripts compare the interval fold with the
  independent existing point fold away from ambiguous boundaries. Disabling the range-delete arm
  makes `d55_deleted_interior_blocks_range_coverage_despite_two_resolved_endpoints` fail, `rc=101`;
  restored source passes. D55 closes in immutable `stitchcad-defects-part2.md`.
- [x] **NO REGRESSION** — `make check` → fmt, strict clippy and all unit/property/contract/doc suites
  green, `rc=0` (62 core unit, 8 identity-contract, 9 identity-property, 12 piece-contract and 13
  range-contract tests; sc-units unchanged). `make wasm` → wasm cross-build green; `make book` →
  warning-free build; feature census → `0 failure(s)`; ledger probes → `9 pass / 0 fail`; `make gate`
  → `=== all doctrines green ===`, all `rc=0`. The point registration verdict is not broadened:
  full coverage, endpoint ambiguity and G2 geometric validity remain separately visible.
- [x] **LOCKSTEP** — pre-code decision + INDEX, ontology §10, feature-matrix identity row, crate
  module docs, task decomposition/evidence/frontier, task index, resume pointer, LIVE_STATUS,
  CHANGELOG and DEV_NOTES; regenerated Knowledge Map. Rollover seals the oldest changelog entry
  into part14 and two dev-note lessons into part8, with exact digests watched by ledger probes.
  The closed-defect pointer names new part2; counts derive to 9 open / 45 sealed. The map's
  interchange orientation entry is shortened without losing its entry path or owner; D53 remains owned.

### `G1-SLICE.3c.1` — immutable structural pieces with visibly deferred geometry

- [x] **REPRODUCE / ISSUE** — ontology §4.1 requires piece content and invariants, but
  `git grep -n 'pub struct Piece' a6d465f -- crates` → no matches, `rc=1`. The existing identity types
  and journal do not supply a piece constructor. `make book` also exposed a formatting defect in
  release §6: literal scope placeholders were parsed as unclosed `<receiver>` / `<version>` HTML tags.
- [x] **ROOT CAUSE (WHY + WHERE)** — the missing constructor is the next ontology layer, not a
  geometry-kernel defect: `cargo test -p sc-core --test piece_contract` → `12 passed`, `rc=0`, with
  invalid-loop/cut-plan/label/live-reference cases pinpointing the constructor boundary in `piece.rs`.
  The structural/geometric decision remains authoritative. An intentional mutation replacing the
  empty-loop predicate with `false` makes `empty_boundary_and_each_empty_hole_name_the_exact_loop`
  fail at the expected empty-boundary assertion (`rc=101`); the source was restored before signoff.
- [x] **FIX** — `ontology::piece` adds immutable `Piece`, editable `PieceDefinition`, directed cyclic
  edge lists, explicit material assignment, complete label view, typed `PieceError` and the only
  geometric state `DeferredToG2`. Labels derive cut information from the plan. Public access is
  shared-only; a compile-fail doctest exercises private-content protection. Literal release scope
  placeholders are code-formatted, so they render visibly. `.3c`'s four children own the remaining scope.
- [x] **ADDRESSED (verified)** — `cargo test -p sc-core --test piece_contract` → `12 passed`, `rc=0`:
  all required content retained; invalid boundary/hole/live-edge/cut-plan/fold/material/label cases
  name their exact typed refusal; cloned-input mutation leaves the validated object unchanged;
  reverse/delete endpoint queries expose direction and repair without rewriting authored content.
  `make wasm` → cross-build green, `rc=0`. `make book` → no warnings, `rc=0`; rendered release HTML
  contains `<code>&lt;receiver&gt;</code> <code>&lt;version&gt;</code>` instead of hidden raw tags.
  D55's passing counterexample explicitly proves the endpoint inventory is not range completeness;
  its repair is scheduled immediately at `.3c.2`, before sewing spans consume the journal.
- [x] **NO REGRESSION** — `make check` → fmt and clippy `-D warnings` clean, existing 62 unit +
  8 contract-property + 9 identity-property tests, 12 piece-contract tests and the privacy doctest
  green; `sc-units` unchanged and its unit/property/doc tests green, `rc=0`. `make gate` →
  `=== all doctrines green ===`, `rc=0`; release and feature-matrix censuses → `0 failure(s)`, `rc=0`;
  `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`, `rc=0`. The new ontology §10 initially made
  feature coverage red (one uncited clause); citing it in the label row restored the census without
  weakening its coverage rule.
- [x] **LOCKSTEP** — ontology §10 and its implementation-status note, feature-matrix label row,
  release-scope rendering, crate docs/description, subsystem map input and generated Knowledge Map;
  `.3c` decomposition and evidence, defect D55 with its scheduled owner, task index, resume pointer,
  LIVE_STATUS, CHANGELOG and DEV_NOTES in this commit. Reviewed read-only source policy bodies match
  the local neutral bodies; cleanup is not due (latest run `2026-09-30` 19:00 UTC, startup
  `2026-10-01` 12:47 UTC).
  promotion: declined (the structural/geometric boundary is already a decision; D55 is a tracked
  open contract to be resolved, rather than a settled general rule).

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

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-30` | `.1` | `cargo metadata --no-deps`; `make check`; `make wasm`; `make gate`; `run_g0_exit_review.sh` | crates `sc-core, sc-units`; `21 passed` property + `1` doc-test; wasm build green; `=== all doctrines green ===`; `G0-17`/`G0-18` `MET` — every `.1` criterion re-derived, `rc=0` |
| `2026-09-30` | `.2` | `cargo test -p sc-units --test property`; `make wasm`; `make gate` | `21 passed` (round-trips, class separation, typed dimension/non-finite errors); wasm cross-build green; `=== all doctrines green ===` — every `.2` criterion re-derived, `rc=0` |
| `2026-09-30` | `.3a` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate` | `31 passed` unit + `9 passed` property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===` — every `.3a` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3b` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate`; `make probes`; `run_changelog_ledger_probes.sh` | `62 passed` unit + `8 passed` contract property + `9 passed` identity property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===`; `22 suite(s) green`; ledger probes `9 pass / 0 fail` — every `.3b` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3c.1` | `make check`; piece contract; `make wasm`; `make book`; `make gate`; release/feature censuses; ledger probes | `12 passed`; privacy doctest green; wasm green; book warning-free; all doctrines green; `0 failure(s)`; `9 pass / 0 fail`, `rc=0` |
| `2026-10-01` | `.3c.2a` | range contract; `make check`; wasm; book; gate; feature census; ledger probes | `13 passed`; all Rust suites green; wasm green; book warning-free; doctrines green; `0 failure(s)`; `9 pass / 0 fail`, `rc=0` |
| `2026-10-01` | `.3c.1a` | piece contract; `make check`; wasm; book; fixture/feature censuses; ledger probes; gate | `14 passed`; all Rust suites green; wasm/book green; `0 mismatch(es)`; `0 failure(s)`; `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.3a` | notch contract; `make check`; wasm; book; feature/release censuses; ledger probes; gate | `7 passed`; privacy check green; Rust/WASM/book green; `0 failure(s)`; `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.2b.1` | cut contract; check; wasm; book; feature/glossary/tree censuses; ledger; gate | `9 passed`; privacy green; all Rust/WASM/book green; censuses green; ledger `9 pass / 0 fail`; doctrines green, `rc=0` |

| `2026-10-01` | `.3c.2b.2` | sewing contracts; check; wasm; book; fixture/feature/glossary; ledger; gate | `18 passed`; graph privacy + notch suites green; Rust/WASM/book green; censuses/ledger/doctrines green, `rc=0`; prior copy milestone full probes `22 suite(s) green` |

| `2026-10-01` | `.3c.3b` | grain contracts; check; wasm; book; feature/glossary/probes; ledger; gate | `9 passed`; reversal mutation red; all restored checks green, `rc=0`; D61 fixed |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | created by the seeding leaf |
| `.1` | `STITCHCAD-G1-0001 (leaf G1-SLICE.1)` | reconciled: delivered by `G0-CONTRACT.18` (`eb83f01`) ahead of this leaf |
| `.2` | `STITCHCAD-G1-0002 (leaf G1-SLICE.2)` | reconciled: `sc-units` delivered by `G0-CONTRACT.18`; property-test decision recorded |
| `.3` | `STITCHCAD-G1-0003 (leaf G1-SLICE.3)` | decomposed into `.3a`/`.3b`/`.3c`; the three design boundaries recorded as layer-C decisions |
| `.3a` | `STITCHCAD-G1-0004 (leaf G1-SLICE.3a)` | the identity layer: `EntityId`/ULID + injected `IdGenerator`, `EdgeRef`/`PointRef`/`LocalTag`, the exact `Rational`/`Param`; 40 tests, wasm green |
| `.3b` | `STITCHCAD-G1-0005 (leaf G1-SLICE.3b)` | the persistent-identity contract: `IdentityLedger`'s append-only edit journal, fold resolution under split/merge/reverse/delete/offset, derived `RepairTask`s + release rule; 62 unit + 8 property tests, wasm green; recorded in `decision_reference-resolution-journal-fold.md` |
| `.3c.1` | `STITCHCAD-G1-0006 (leaf G1-SLICE.3c.1)` | immutable structural pieces with deferred geometry; D55 owned by the next child |
| `.3c.2a` | `STITCHCAD-G1-0007 (leaf G1-SLICE.3c.2a)` | exact whole-range journal fold and visible range repairs; D55 fixed |
| `.3c.1a` | `STITCHCAD-G1-0008 (leaf G1-SLICE.3c.1a)` | D56 fixed: separate cut-once L/R members, explicit companion metadata |
| `.3c.3a` | `STITCHCAD-G1-0009 (leaf G1-SLICE.3c.3a)` | semantic notches and symbolic bindings; D58 fixed |
| `.3c.2b.1` | `STITCHCAD-G1-0010 (leaf G1-SLICE.3c.2b.1)` | explicit physical-copy plan; D59/D60 fixed |
| `.3c.2b.2` | `STITCHCAD-G1-0011 (leaf G1-SLICE.3c.2b.2)` | copy-addressed sewing; D35/D57 fixed; sewing parents closed |
| `.3c.3b` | `STITCHCAD-G1-0012 (leaf G1-SLICE.3c.3b)` | directed grainlines; independent print references; D61 fixed |
| `.3c.3c` … `.16` | `pending` | marks/allowances and constructions remain in `.3c` |

## Changelog

- `2026-09-29`: Tree created by `PLANNING.2` with 16 leaves; owns defect D10 (starter crate).
- `2026-09-30`: `.1` reconciled and closed — its workspace shape (starter crate retired, `sc-units` +
  `sc-core` created, G0 CI shape) was delivered by `G0-CONTRACT.18` (`eb83f01`) "ahead of `G1-SLICE.1`";
  every acceptance criterion re-derived by command and recorded in the `### G1-SLICE.1` checklist. D10 was
  already closed by that commit; the frontier advances to `.2`.
- `2026-09-30`: `.2` reconciled and closed — `sc-units` (fixed-point µm/µ°, the five tolerance classes T1–T5,
  exact-ratio conversions, typed `UnitError` for domain/overflow/division-by-zero/non-finite) was delivered in
  full by `G0-CONTRACT.18` (`eb83f01`); its 21 property tests, typed dimension errors and wasm build were
  re-derived against every acceptance criterion. The property-test-framework Open Question is resolved into
  the Decisions section and recorded as `decision_property-tests-dependency-free-recorded-seed.md`. The
  frontier advances to `.3` (the `sc-core` ontology), G1's first new product code.
- `2026-09-30`: `.3` decomposed into `.3a` (identity types), `.3b` (the persistent-identity contract) and
  `.3c` (the geometry-bearing object types) — one leaf was three signoff-quality slices in strictly ordered
  dependency. Its three cross-cutting design boundaries were recorded before any code, so the implementation
  slices build against a fixed design: a dependency-free ULID `EntityId` with an injected generator, a
  bounded exact rational edge parameter in `sc-core` (not `sc-units`, not the formula bigint), and the
  structural-at-G1 / geometric-at-G2 invariant boundary. The tree is 18 leaves; the frontier advances to
  `.3a`.
- `2026-09-30`: `.3a` landed — G1's first new product code. `sc_core::ontology`'s identity layer:
  `EntityId` (a dependency-free hand-rolled ULID, Crockford base32, lexicographically sortable) and the
  injected `IdGenerator` with a `DeterministicIdGenerator` for replay; `EdgeRef`/`PointRef`/`LocalTag` (stable
  topological references, never indices); the bounded exact `Rational` and its `[0, 1]` `Param`. 31 unit tests
  and 9 dependency-free recorded-seed properties green, `sc-core` still cross-builds to wasm, `make gate`
  green. Built against `decision_entity-identity-ulid-injected-generator.md` and
  `decision_edge-parameter-bounded-exact-rational.md`. The frontier advances to `.3b` (the persistent-identity
  contract).
- `2026-10-01`: `.3b` landed — G1's second new product code. `sc_core::ontology::topology`: the
  `IdentityLedger` (an append-only journal of typed `TopologyEdit`s + a live-edge index + registrations) and
  the pure fold that resolves a held `(EdgeRef, Param)` through split, merge, reverse, delete and
  offset-fragmentation. A stored reference is never rewritten; repair state (`RepairTask`, `open_repairs`,
  `release_readiness`) is derived, never stored, so it cannot drift from the journal. Split offers both
  fragments at the split point for the consumer to state a `SplitSide`; merge recomputes by caller-declared
  arc length in exact `Rational`; reverse maps `t` to `1 − t` and tells directed consumers via `Direction`;
  delete and offset orphan references into visible `RepairTask`s naming the reference, the orphaning edit and
  the candidates — no silent reassignment. 62 unit + 8 recorded-seed contract properties green (merge checked
  against a cross-multiplied `i128` oracle, offset against a hundredths grid, the live set against the journal
  replayed, replay byte-identical), `sc-core` still cross-builds to wasm, `make gate` + `make probes` (22
  suites) green. Built against `decision_reference-resolution-journal-fold.md`, recorded before the code. The
  append also discharged two containment obligations it owed (D54: the CHANGELOG and DEV_NOTES rollovers the
  prior slice crossed without sealing) and surfaced D53 (the KNOWLEDGE_MAP ceiling pressure `.3a` flagged).
  The frontier advances to `.3c` (the geometry-bearing object types).

- `2026-10-01`: `.3c` decomposed into four children; `.3c.1` landed `Piece`, its immutable validated
  content and complete derived label view. Structural refusals are tested, geometry is always
  `DeferredToG2`, endpoints expose repairs without rewriting references. A tracked middle-fragment
  deletion counterexample creates D55, owned immediately by `.3c.2` before sewing spans are built.
  The book's two literal scope placeholders were repaired after the renderer diagnosed hidden HTML;
  feature coverage now cites the implemented piece contract. All focused gates pass; next `.3c.2`.

- `2026-10-01`: `.3c.2` split into range resolution (`.2a`) and sewing graph (`.2b`). `.2a` fixes
  D55: the exact interval fold retains deleted/trimmed portions and arithmetic failures as repairs,
  separately from point endpoint ambiguity and G2 geometry. Thirteen contract tests and the existing
  suites pass, with the delete arm observed red when disabled. The ledgers roll over atomically;
  D55 is sealed closed in defects-part2. Next `.3c.2b` implements sewing spans and settles D35.

- `2026-10-01`: `.3c.1a` fixes D56, found by comparing the new model with the canonical fixture:
  separate cut-once L/R members now carry explicit hand/companion metadata; self-pairing is refused.
  Existing even-total pair requests stay supported. Fourteen piece tests and all focused checks pass;
  D56 is sealed closed. D57's physical-copy addressing is asked of the director and owned by `.3c.2b`;
  the frontier takes independent marks/allowances (`.3c.3`) while that decision is pending.

- `2026-10-01`: `.3c.3` decomposed into notches, grainlines and allowances; `.3c.3a` lands
  immutable semantic Notch anchors and symbolic target-profile bindings with no physical defaults.
  Seven contract tests + privacy doctest pass; ownership mutation goes red. D58's G4 acceptance
  aligns with the normative release matrix and seals closed. Ledgers roll over atomically;
  the director answered D57 before commit: stable physical-copy identities; next `.3c.2b`.

- `2026-10-01`: `.3c.2b` splits into copy plan and spans. `.3c.2b.1` implements the director's
  stable-copy ruling with explicit identities and complete quantity/orientation validation. Nine
  contract tests pass and quantity mutation goes red. D59's committed-baseline vocabulary failures
  are repaired; glossary census is a required focused ontology check. Completed `.1`/`.2`/`.3a`/`.3b`
  evidence blocks move to a linked sibling (SHA-256
  `ab447c14f2db092114863e1ccbb4f555441dbb204abe18de80cbe19fd8698517`). Next `.3c.2b.2`.

  D60's staged relocation refusal is fixed by preserving the original `.1`/`.2` text and adding
  re-derived delivery evidence inside their ROOT CAUSE bullets; all four moved checklists pass
  the per-bullet audit and staged gate. Original payload checksum above describes the pre-supplement
  move, not the final augmented sibling.

- `2026-10-01`: `.3c.2b.2` lands immutable copy-addressed sewing intent, disjoint same-copy seams,
  semantic stops and explicit ease distributions. Eighteen tests and graph privacy pass; both
  self-overlap and interval-ownership mutations go red. D35/D57 seal verified closed; `.3c.2b`
  and `.3c.2` close. Prior copy milestone full probes: 22 suites green. Books/censuses remain aligned,
  with no invented folded-end geometry or changed arithmetic golden. Next `.3c.3b` grainlines.

- `2026-10-01`: `.3c.3b` lands directed grain and independent stripe/plaid references with
  explicit angle intent and deferred geometric/value validation. D61 diagnostic fixed; ontology §10
  executable examples move unchanged to a linked chapter. Next `.3c.3c` allowances.
