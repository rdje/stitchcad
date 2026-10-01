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

- ID: `G1-SLICE.3c.2`
  Status: `pending`
  Goal: first discharge D55 with full-range resolution/repair (endpoints cannot certify an interior);
  then `SeamSpan` and immutable `SewingGraph`, partial and one-to-many edge ranges, declared ease
  distribution, direction and stop landmarks; resolve D35's same-piece seam rule against the supported
  dart and trouser constructions before code.
  Acceptance: a deleted interior fragment blocks full-range integrity even when endpoints resolve;
  absent pieces/edges, empty or reversed parameter ranges, duplicate span identities and
  missing stop references are typed refusals; same-piece seams have an explicit tested contract;
  edits expose repairs without rewriting authored ranges. Geometric differential checks remain G2.
  Verification: `pending`
  Commit: `pending`

- ID: `G1-SLICE.3c.3`
  Status: `pending`
  Goal: `Notch`, directed and dual-reference `Grainline`, per-edge derived `SeamAllowance`; profile
  parameters are identity references rather than invented defaults until G4 supplies parameter states.
  Acceptance: references exist at construction; notch export geometry and encoding remain profile-owned;
  unknown profile parameters are never replaced by a value; directed references survive reversal or
  show repairs, and allowance inclusion is resolved per profile rather than as a global switch.
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
  semantics, not just geometry.
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
| — | `G1-SLICE.3c.2` | `pending` | full-range integrity (D55) before sewing spans; same-piece seam rule (D35), partial and one-to-many correspondences with declared ease |

## Decisions

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

Filled per leaf, in a `### <leaf-id>` subsection added by the same commit as the work, and
mechanically required to be fresh in that commit by leaf `SPINE.8`. A tree file carries no
unticked placeholder boxes: the spine's acceptance gate judges the FIRST matching box in the
file, so a placeholder both shadows real evidence and falsely rejects honest work (defect D15,
measured by the `SPINE.7` probe).

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

### `G1-SLICE.3b` — the persistent-identity contract: a reference is never rewritten, the journal folds

G1's second new product code. Ontology §1.1 requires that when an edit changes the topology a reference
points into, the reference either resolves onto the new topology or becomes a visible repair task — never a
silent reassignment. This slice implements that contract in `sc_core::ontology::topology` against the design
decision recorded before the code, dependency-free and wasm-safe.

- [x] **REPRODUCE / ISSUE** — the requirement, located and unimplemented: `grep -n 'The persistent-identity
  contract' docs/book/src/spec/ontology.md` → line 39, `rc=0` (§1.1's five-row edit table and the "no silent
  reassignment" rule), against `git show HEAD:crates/sc-core/src/ontology/mod.rs | grep -c topology` → `0`,
  `rc=1`. `.3a` landed the identity *types* (`EdgeRef`/`Param`); nothing resolved a reference through an edit
  and nothing turned an orphan into a repair task, so the contract the whole design's reference integrity
  rests on was spec-only.
- [x] **ROOT CAUSE (WHY + WHERE)** — the design is a recorded decision, not an ad-hoc choice:
  `decision_reference-resolution-journal-fold.md` (`grep -c '^answers:' …` → `1`, `rc=0`) — a stored reference
  is never rewritten; resolution is a pure fold of an append-only edit journal, so no consumer can be missed,
  and repair state is *derived* (`open_repairs`/`release_readiness`), never stored, so it cannot drift. It
  inherits `.3`'s two prior boundaries: the injected-generator ULID
  (`decision_entity-identity-ulid-injected-generator.md`) and the bounded exact rational
  (`decision_edge-parameter-bounded-exact-rational.md`), which is why split/merge recompute with zero drift.
- [x] **ADDRESSED (verified)** — every acceptance criterion is a passing test. `cargo test -p sc-core` →
  `62 passed` (unit) + `8 passed` (contract property) + `9 passed` (identity property), `rc=0`. **Split**:
  the trichotomy resolves into the containing fragment, and a reference at the split point returns a
  `SplitPoint` with both sides for the consumer to state (`SplitSide`) —
  `a_reference_at_the_split_point_resolves_to_both_and_the_consumer_states_which_it_wants`, property
  `split_resolves_by_trichotomy_and_the_split_point_offers_both_sides`. **Merge by arc length**:
  `merge_recomputes_the_parameter_by_arc_length` (1 cm + 3 cm maps `t=1/2` to `1/8` and `5/8` exactly), and
  the property checks the defining proportion cross-multiplied in `i128`, an oracle sharing no code with the
  fold. **Reverse**: `1 − t` + `Direction::Reversed`, an involution. **Delete → RepairTask**:
  `delete_orphans_a_reference_into_a_visible_repair_task` — the task names the reference, the orphaning edit
  (`Deleted { operation, edge }`) and the candidates (empty for a deletion); the property proves exactly the
  references resolving onto the victim orphan and nothing else moves. **Savable/inspectable/not-releasable**:
  `a_design_with_unresolved_references_is_inspectable_but_not_releasable` — every query still answers and
  `release_readiness()` → `Blocked { unresolved: 1 }`. `make wasm` → `sc-units + sc-core build for
  wasm32-unknown-unknown`.
- [x] **NO REGRESSION** — `make check` → fmt clean, clippy `-D warnings` clean, `cargo test --all` green
  (sc-units' 21 properties + doc-test and `.3a`'s 9 identity properties unaffected); `make gate` → `=== all
  doctrines green ===`, `rc=0`; `make probes` → `22 suite(s) green`; `make wasm` green. The change is
  additive: `sc-core` gained an `ontology::topology` module and a second property-test target, no existing
  behaviour changed, and `sc-units` and `.3a`'s `id`/`rational`/`reference` were untouched. The two
  rolling-ledger rollovers this append owed (D54) are verified by `run_changelog_ledger_probes.sh` →
  `9 pass / 0 fail`.
- [x] **FIX** — implemented `sc_core::ontology::topology`: `IdentityLedger` (an append-only journal +
  live-edge index + registrations), `TopologyEdit` (declare/split/merge/reverse/delete/offset),
  `Resolution`/`ResolvedRef`/`Direction`/`SplitSide`, `RepairTask`/`OrphaningEdit`/`OpenRepair`,
  `ReleaseReadiness`, `OffsetInterval`/`OffsetFragment`, `LedgerError`, `MAX_EDGES_PER_OPERATION`;
  re-exported from `ontology/mod.rs`; updated `lib.rs`'s status + module table; 31 inline unit tests and 8
  dependency-free recorded-seed properties in `tests/identity_contract_property.rs`. Also repaired two
  hand-kept-doc defects the sync gates cannot see (they check derivation, not source form): a run-on bullet
  in `knowledge-map/subsystems.md` (two entries shared one line) and an accidental duplicated sentence in
  `docs/TASK_TREE.md`'s execution-order prose (a D34-class drift, logged in the leaf rather than a new id
  since `PLANNING.5` owns deriving that index).
- [x] **LOCKSTEP** — `decision_reference-resolution-journal-fold.md` (new) + its INDEX row;
  `knowledge-map/subsystems.md` + regenerated `KNOWLEDGE_MAP.md` (trimmed under its 8192 ceiling, D53);
  `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action → `.3c`), `LIVE_STATUS.md` (G1 → 4 of 18,
  census → 9 open), `CHANGELOG.md` (entry + the part13 rollover), `DEV_NOTES.md` (lesson + the part7
  rollover), `PLANNING.md` (D53, D54), and this tree's frontier, three logs and changelog.
  promotion: promoted by `decision_reference-resolution-journal-fold.md` (carries `answers:`) — the durable
  design boundary is recorded there so `.3c`/`.6`/`.7` inherit it rather than re-litigate.

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

### `G1-SLICE.1` (reconciled) — the workspace shape shipped under `G0-CONTRACT.18`, so this leaf closes on re-derived evidence, not new code

`G0-CONTRACT.18` (commit `eb83f01`) landed the G0 CI shape and, in its own words, retired the bedrock
starter crate "closing defect D10 ahead of `G1-SLICE.1`". It therefore delivered every artifact this leaf
names, but the leaf was left `pending` — a tree↔code drift: the leaf's status disagreed with the shipped
workspace. This slice audits that delivery against each acceptance criterion and records the closure. No
Rust changes; the gate that judges it is the doctrine enforcer plus the re-derived commands below.

- [x] **REPRODUCE / ISSUE** — the leaf said `pending` while its deliverables were committed:
  `git show HEAD:docs/tasks/G1-SLICE.md | awk '/ID: .G1-SLICE\.1./{f=1} f&&/Status:/{print;exit}'` →
  ``  Status: `pending` ``, `rc=0`, against `git log --oneline -- crates/sc-units crates/sc-core` →
  `eb83f01 STITCHCAD-G0-0018 (leaf G0-CONTRACT.18): the first product code - sc-units`, `rc=0`. A frontier
  that points at a leaf whose work already shipped misdirects the next session into redoing it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `G0-CONTRACT.18`'s acceptance let it answer the starter-crate question
  itself ("retired here or explicitly handed to `G1-SLICE.1`"), and its commit message records "The bedrock
  starter crate is retired (git rm crates/app), closing defect D10 ahead of G1-SLICE.1". The G0 leaf closed;
  the G1 leaf it pre-empted was never reconciled. The remedy is the audit, not new code: the deliverables
  exist and are green.
- [x] **ADDRESSED (verified)** — each acceptance criterion re-derived by command. (1) `cargo metadata
  --no-deps --format-version 1` lists exactly the roadmap crates that exist so far, `sc-core, sc-units`
  (§4.3 — the rest appear when their stage starts), `rc=0`. (2) `git ls-tree HEAD crates/` shows only those
  two trees, so `crates/app` is gone, and `grep -rn 'Hello\|template\|starter' crates/` returns no match —
  no crate prints the template message. (3) The G0 CI shape is green locally and in the gate: `make check`
  → `test result: ok. 21 passed` (property) plus `1 passed` (doc-test), `make wasm` → `wasm-viewer
  smoketest: sc-units + sc-core build for wasm32-unknown-unknown`, and `run_g0_exit_review.sh` reports
  `G0-17 MET` (CI) and `G0-18 MET` (the wasm build). (4) `KNOWLEDGE_MAP.md` names both subsystems (lines 12
  and 17, each with entry point, conformance and owner).
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `rc=0`; `make check` green; `make
  wasm` green. This slice stages no Rust, so the acceptance gate (`check_task_acceptance.sh`, which fires
  only on staged code) does not govern it; these boxes are the tree's own per-leaf record.
- [x] **FIX** — marked `.1` `done`, recorded its verification and commit, advanced the frontier, and added
  this subsection so the closure carries re-derivable evidence.
- [x] **LOCKSTEP** — `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action), `LIVE_STATUS.md` (G1 row
  → In Progress), `CHANGELOG.md`, `DEV_NOTES.md` and the regenerated Knowledge Map, all in this commit.
  promotion: declined (instance of the D34 hand-kept-state class `PLANNING.5` owns; this slice fixes the instance, a new record would duplicate that ownership).

### `G1-SLICE.2` (reconciled) — `sc-units` shipped under `G0-CONTRACT.18`, and the property-test choice it made is now a recorded decision

`G0-CONTRACT.18` (commit `eb83f01`) landed `sc-units` in full — 1097 lines of library and 564 lines of
property tests — as "the first product code", well past the skeleton its own leaf scoped. That is this leaf's
deliverable, so `.2` closes on the same audit `.1` did: every acceptance criterion re-derived by command, no
new code. The one thing `eb83f01` left unrecorded was the property-test-framework choice this tree's Open
Questions deferred to `.2`; closing the leaf records it.

- [x] **REPRODUCE / ISSUE** — the leaf said `pending` while `sc-units` was committed and green:
  `git show HEAD:docs/tasks/G1-SLICE.md | awk '/ID: .G1-SLICE\.2./{f=1} f&&/Status:/{print;exit}'` →
  ``  Status: `pending` ``, `rc=0`, against `wc -l crates/sc-units/src/*.rs crates/sc-units/tests/*.rs` →
  `1723` lines delivered by `eb83f01`. The Open Question "Property-test framework choice … decided in `.2`"
  was never discharged, so a later crate would re-litigate `proptest` vs `quickcheck`.
- [x] **ROOT CAUSE (WHY + WHERE)** — the same pre-emption as `.1`: `G0-CONTRACT.18` answered the G0 CI clause
  by building `sc-units` out fully rather than as a skeleton, and closed only itself. `.2`'s work was done but
  its leaf, and the decision the Open Question routed to it, were not recorded.
- [x] **ADDRESSED (verified)** — each acceptance criterion re-derived. (1) Property tests for conversion
  round-trips and tolerance-class separation: `cargo test -p sc-units --test property` → `test result: ok. 21
  passed; 0 failed`, including `conversion_round_trips_for_integral_units` (line 62),
  `the_classes_disagree_so_they_are_load_bearing` (448) and `counts_are_their_own_dimension` (500). (2) A
  dimension error is a typed error, never a silent coercion: `UnitError` (`crates/sc-units/src/error.rs`)
  carries `DomainExceeded`/`DivisionByZero`/`NonFinite`/`Overflow`/`EmptyDerivation`, and
  `non_finite_floats_are_rejected_at_the_boundary` (291) proves the boundary rejects NaN/inf. (3) Compiles for
  `wasm32-unknown-unknown`: `make wasm` → `wasm-viewer smoketest: sc-units + sc-core build for
  wasm32-unknown-unknown`. The five tolerance classes are distinct `ToleranceClass` variants (T1–T5,
  `tolerance.rs`). The framework choice is recorded as
  `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` (with `answers:`, listed in the
  INDEX), resolving the Open Question.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `rc=0`; `make check` green; `make wasm`
  green. The new decision record is a `docs/decisions` file; the KNOWLEDGE-MAP doctrine confirms the
  regenerated map is in sync. No Rust staged, so the acceptance gate does not fire.
- [x] **FIX** — marked `.2` `done`, recorded its verification and commit, resolved the Open Question into the
  Decisions section and a layer-C record, advanced the frontier to `.3`, and added this subsection.
- [x] **LOCKSTEP** — `docs/decisions/decision_property-tests-dependency-free-recorded-seed.md` (new) and its
  INDEX row; `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action), `LIVE_STATUS.md` (G1 row → 2 of
  16), `CHANGELOG.md`, `DEV_NOTES.md` (the lesson, promoted by the new record's `answers:`) and the
  regenerated Knowledge Map, all in this commit.

### `G1-SLICE.3a` — the identity layer: a ULID with an injected generator, stable references, an exact parameter

G1's first new product code. Ontology §1 requires every entity to carry a ULID stable across saves and
exports, every geometric sub-entity to be addressed by identity rather than index, and a parameterized
position to be an exact rational in `[0, 1]`. This slice implements that identity layer in `sc-core::ontology`
against the two design decisions `.3` recorded — dependency-free, and wasm-safe.

- [x] **REPRODUCE / ISSUE** — the requirement, located: `grep -n 'a ULID, assigned once at creation'
  docs/book/src/spec/ontology.md` → line 24, `rc=0`, and the glossary's `parameterized reference` → "a
  position along an edge as a rational in [0, 1]". Before this slice `sc-core` was a skeleton
  (`git show HEAD:crates/sc-core/src/lib.rs | grep -c 'Status: skeleton'` → `1`, `rc=0`); the identity layer
  the whole ontology rests on did not exist in code.
- [x] **ROOT CAUSE (WHY + WHERE)** — the design is the two recorded decisions, not an ad-hoc choice.
  `EntityId` is a hand-rolled dependency-free ULID because `sc-core` builds for `wasm32-unknown-unknown`
  (`make wasm` → green) and a ULID crate would pull a clock and entropy into the wasm graph, so generation is
  behind an injected `IdGenerator` (`decision_entity-identity-ulid-injected-generator.md`); the parameter is a
  bounded exact `Rational` in `sc-core`, not `sc-units`' ppm `Ratio`, which would round and drift on every
  split/merge (`decision_edge-parameter-bounded-exact-rational.md`). Both records carry `answers:` and are in
  the INDEX: `grep -c 'answers:' docs/decisions/decision_entity-identity-ulid-injected-generator.md` → `1`,
  `rc=0`.
- [x] **ADDRESSED (verified)** — every acceptance criterion is a passing test. `cargo test -p sc-core` →
  `test result: ok. 31 passed` (unit) and `9 passed` (property), `rc=0`: an `EntityId` round-trips its
  26-character Crockford form and its text order equals its byte order
  (`an_entity_id_round_trips_and_sorts_lexicographically`); a deterministic generator reproduces a sequence
  byte-for-byte (`a_deterministic_generator_reproduces_its_sequence`); the parameter is exact under the four
  operators (`arithmetic_is_exact_and_never_rounds`, `subtraction_and_division_are_exact_inverses`), reduced to
  a canonical form (`reduction_is_canonical`), totally ordered (`ordering_is_exact_and_total`), bounded by
  `in_unit_interval` (`the_unit_interval_predicate_is_exact_at_its_ends`), and reports overflow and
  division-by-zero as typed diagnostics (`a_result_past_i64_is_an_overflow_not_a_wrap`,
  `division_by_zero_is_a_diagnostic`). `make wasm` → `sc-units + sc-core build for wasm32-unknown-unknown`.
- [x] **NO REGRESSION** — `make check` → fmt clean, clippy `-D warnings` clean, `cargo test --all` green
  (sc-units' 21 properties and doc-test unaffected); `make gate` → `=== all doctrines green ===`, `rc=0`. The
  change is additive: `sc-core` gained an `ontology` module and a property-test target, no existing behaviour
  changed, and `sc-units` was not touched.
- [x] **FIX** — implemented `sc_core::ontology`: `id` (`EntityId`, `IdError`, `IdGenerator`,
  `DeterministicIdGenerator`), `rational` (`Rational`), `reference` (`LocalTag`, `EdgeRef`, `PointRef`, `Param`,
  `ParamError`), re-exported from `ontology/mod.rs`; updated `lib.rs`'s status and module table; 31 unit tests
  inline and 9 dependency-free recorded-seed properties in `tests/identity_property.rs`.
- [x] **LOCKSTEP** — `knowledge-map/subsystems.md` (sc-core is no longer a skeleton) and the regenerated
  `KNOWLEDGE_MAP.md`; `docs/TASK_TREE.md` (frontier cell), `MEMORY.md` (next action → `.3b`), `LIVE_STATUS.md`
  (G1 → 3 of 18), `CHANGELOG.md`, `DEV_NOTES.md`, and this tree's frontier, logs and changelog.
  promotion: declined (the durable design choices are the three decision records `.3` landed; this slice's note is one leaf's implementation history, and the two conventions it records are already the house style `sc-units` set, not a new general rule).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-30` | `.1` | `cargo metadata --no-deps`; `make check`; `make wasm`; `make gate`; `run_g0_exit_review.sh` | crates `sc-core, sc-units`; `21 passed` property + `1` doc-test; wasm build green; `=== all doctrines green ===`; `G0-17`/`G0-18` `MET` — every `.1` criterion re-derived, `rc=0` |
| `2026-09-30` | `.2` | `cargo test -p sc-units --test property`; `make wasm`; `make gate` | `21 passed` (round-trips, class separation, typed dimension/non-finite errors); wasm cross-build green; `=== all doctrines green ===` — every `.2` criterion re-derived, `rc=0` |
| `2026-09-30` | `.3a` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate` | `31 passed` unit + `9 passed` property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===` — every `.3a` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3b` | `cargo test -p sc-core`; `make check`; `make wasm`; `make gate`; `make probes`; `run_changelog_ledger_probes.sh` | `62 passed` unit + `8 passed` contract property + `9 passed` identity property; fmt/clippy `-D warnings` clean; `sc-core` cross-builds to wasm; `=== all doctrines green ===`; `22 suite(s) green`; ledger probes `9 pass / 0 fail` — every `.3b` criterion re-derived, `rc=0` |
| `2026-10-01` | `.3c.1` | `make check`; piece contract; `make wasm`; `make book`; `make gate`; release/feature censuses; ledger probes | `12 passed`; privacy doctest green; wasm green; book warning-free; all doctrines green; `0 failure(s)`; `9 pass / 0 fail`, `rc=0` |

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
| `.3c.2` … `.16` | `pending` | `.3c` closes after all four children |

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
