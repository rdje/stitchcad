# G0-CONTRACT: product & semantic contract (roadmap gate G0)

## Metadata

- Tree ID: `G0-CONTRACT`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §11 gate **G0 — Product & semantic contract** (sources: §2, §3,
  §4.2, §5, §8, §9, §12)
- Created: `2026-09-29`
- Owner: repo-local workflow

## Goal

Close gate G0: the semantic contract of StitchCAD exists as **normative, reviewed
specification** — ontology, units and tolerances, the supported envelope, both instantiation
paths, size-set ownership, the release/approval contract, the four ADRs, the identified
measurement standards, and the reference-skirt fixture specified with real numbers — plus a
drafted governance model.

G0 is a **specification** gate with one narrow code exception the roadmap itself demands: §4.3
(`CI grows with stages (G0: fmt/clippy/unit+property/WASM smoketest)`) and §7.3 (`WASM CI at G0 =
smoketest that sc-core + sc-units compile to wasm32-unknown-unknown`) require the two smallest
crates to exist and build for the browser target. Leaf `.18` owns exactly that and nothing more.
Every other deliverable here is an mdBook specification chapter (`docs/book/src/spec/`) or a
layer-C decision record (`docs/decisions/`), because a contract that is not written and reviewed is
not a contract.

## Non-Goals

- Implementing the product: `.18` creates two skeleton crates and the G0 CI workflow, and every
  other crate, solver, exporter and viewer is G1+ work.
- Re-opening decisions locked by roadmap §15 — this tree *records* them as ADRs and
  specifies their consequences.
- Procuring evaluation seats or naming people: this tree drafts the governance model and
  records which roles need a named human, and flags the naming to the director
  (`G0-CONTRACT.14`).
- Claiming conformance to any external standard. Where a standard's text has not been read
  in this repository, the claim is recorded as **unverified** with an owner, per the
  claim-verification policy.

## Acceptance Criteria (gate G0 exit, clause by clause)

| Roadmap G0 exit clause | Leaf | Deliverable |
| --- | --- | --- |
| glossary of construction terms | `G0-CONTRACT.1` | `docs/book/src/spec/glossary.md` |
| units & tolerance policy | `G0-CONTRACT.2` | `docs/book/src/spec/units-and-tolerances.md` + numerical-contract decision record |
| garment ontology v1 (§3.1) specified | `G0-CONTRACT.3` | `docs/book/src/spec/ontology.md` |
| supported/rejected/deferred feature matrix | `G0-CONTRACT.4` | `docs/book/src/spec/feature-matrix.md` |
| dual instantiation paths specified | `G0-CONTRACT.5` | `docs/book/src/spec/instantiation-paths.md` |
| size-set ownership decided | `G0-CONTRACT.6` | decision record + `docs/book/src/spec/size-sets.md` |
| measurement/POM standards identified | `G0-CONTRACT.7` | `docs/book/src/spec/standards.md` |
| ADR-0001 license × solver recorded | `G0-CONTRACT.8` | `docs/decisions/` record |
| ADR-0003 drafting paradigm + formula language v1 | `G0-CONTRACT.9` | decision record + `docs/book/src/spec/formula-language.md` |
| ADR-0004 dialects recorded | `G0-CONTRACT.10` | decision record + `docs/book/src/spec/interchange-dialects.md` |
| UI stack / canvas hosting (ADR-0002, settled by the G1 spike) | `G0-CONTRACT.11` | decision record with the spike protocol |
| approval states & release contract (§9) specified | `G0-CONTRACT.12` | `docs/book/src/spec/release-contract.md` |
| reference skirt fully specified with numbers | `G0-CONTRACT.13` | `docs/book/src/spec/reference-skirt.md` |
| governance model drafted; procurement owner named | `G0-CONTRACT.14` | `docs/book/src/governance.md` + decision record (director input required) |
| evaluation-seat procurement started | `G0-CONTRACT.14` | same leaf — role + fallback documented, name supplied by the director |
| one message system chosen (§7.6) + externalization architecture | `G0-CONTRACT.16` | decision record + `docs/book/src/spec/i18n-architecture.md` |
| undo/redo semantics defined (§4.4) + the command-layer contract | `G0-CONTRACT.17` | `docs/book/src/spec/command-layer.md` |
| G0 CI: fmt/clippy/unit+property/WASM smoketest (§4.3, §7.3) | `G0-CONTRACT.18` | `sc-units` + `sc-core` skeletons, `.github/workflows/`, capability note |
| gate closure: every clause above evidenced | `G0-CONTRACT.15` | exit review recorded in this tree, `LIVE_STATUS.md`, roadmap status line |

## Task Tree

- ID: `G0-CONTRACT`
  Status: `active`
  Goal: gate G0 closed — the semantic contract written, reviewed and traceable.
  Children: `.1` … `.18`

- ID: `G0-CONTRACT.1`
  Status: `pending`
  Goal: the glossary of construction terms — one entry per term used anywhere in the spec
  set, each with a plain-language definition, the canonical object it names (ontology id),
  synonyms used by factories/other CADs, and the machine token that must never be rendered raw.
  Acceptance: every domain noun in roadmap §3.1 has an entry; every entry names its ontology
  object; the book chapter builds; terms used later in the spec set are all defined here.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.2`
  Status: `done`
  Goal: units & tolerance policy — internal fixed-point micrometres (i64), the five tolerance
  classes (numerical, geometric approximation, format quantization, importer comparison,
  physical acceptance) with their derivation rule, curve representation set (line/arc/cubic
  Bézier), robust-predicate requirement, and the offset error-budget contract.
  Acceptance: each tolerance class states what it bounds, who sets its value, and how a
  violation is reported; the unit-conversion and rounding rules are stated exactly enough to
  be implemented without a further decision; no single global epsilon anywhere.
  Verification: recorded below — chapter built into the book, arithmetic re-derived, gates green.
  Commit: `STITCHCAD-G0-0002`

- ID: `G0-CONTRACT.3`
  Status: `pending`
  Goal: garment ontology v1 — normative field-level specification of MeasurementTable, Ease,
  Design (construction recipe), Piece, SeamSpan/SewingGraph, Dart/Tuck/Pleat/Gather,
  SeamAllowance, Notch, Grainline, Hem/Facing/Lining/Interfacing, Closure, Pocket,
  Corner/Truing/Walking operations, plus the persistent-identity contract (§4.1).
  Acceptance: every object has identity rules, required/optional fields, invariants, and its
  uncertainty states; references are to stable topological entities, never array indices;
  each object cites the roadmap clause it comes from.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.4`
  Status: `pending`
  Goal: the supported / rejected / deferred feature matrix bounding the v1 release claim
  (woven family: A-line skirt with waist dart + CB zipper, darted bodice, set-in sleeve,
  classic collar, trousers), with the diagnostic each rejected construction must produce.
  Acceptance: every row is one of supported/rejected/deferred with a reason and a gate;
  nothing in the ontology is silently unlisted; non-goals (§1.3) appear as rejected rows.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.5`
  Status: `pending`
  Goal: both instantiation paths specified — measurement-driven regeneration and grade-rule
  instantiation (`.rul`: incremental vs cumulative, stack-point / fixed-perimeter /
  smoothing attributes) — including where they diverge and the declared equivalence
  tolerances.
  Acceptance: the known information loss between the paths is stated, not hidden; extreme-size
  checking after target-system reconstruction is specified; each path names its inputs,
  outputs and oracle.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.6`
  Status: `pending`
  Goal: decide and record SizeSet ownership (default per §3.4: referenced by Design,
  overridable per Factory Profile with a recorded transformation).
  Acceptance: a decision record with context/decision/consequences; the spec chapter states
  the object's fields, label-vs-order semantics, base size, multi-dimensional charts and the
  EN 13402 / ASTM D5585 mappings it must express.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.7`
  Status: `pending`
  Goal: identify the measurement/POM standards the model draws on (ISO 8559, ASTM D5219,
  EN 13402, ASTM D5585) — what each is used for, what is adopted, what is deliberately not,
  and the verification status of every claim about a standard's content.
  Acceptance: each standard has a named role and a claim-verification status
  (read-in-repo / cited-from-roadmap / unverified-with-owner); no clause number or table is
  asserted without a source.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.8`
  Status: `pending`
  Goal: record ADR-0001 (license × solver) as one coupled decision, per the roadmap default
  (permissive core; `slvs`/GPLv3 rejected; `sc-sketch` uses a custom or least-squares kernel).
  Acceptance: decision record states the options, the coupling, the BSL-1.1 distinction, the
  consequence for contributors and the re-open condition (a strong case from the G1 spike).
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.9`
  Status: `pending`
  Goal: record ADR-0003 (construction recipe primary) **and specify the formula language v1**:
  operators, units inside expressions, conditionals (multi-size branching), name binding
  (measurements, prior points/lengths/angles, profile parameters), evaluation order, error and
  dimension rules.
  Acceptance: the language is implementable from the chapter alone — grammar, type/unit rules,
  determinism requirement, worked examples over the reference skirt; the record names what is
  deliberately excluded (NURBS-class expressions, implicit solving).
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.10`
  Status: `pending`
  Goal: record ADR-0004 (interchange dialects) and specify the StitchCAD interchange profiles:
  AAMA named layers × ASTM numbered layers, cut-as-1 × sew-as-1, R12 × R13, BLOCK-per-piece,
  SST/PST, grading modes, tessellation policy — with ASTM D6673-10's withdrawal recorded.
  Acceptance: the layer table is reproduced with both naming modes; each mode is a named,
  separately validated export target; no claim of a universal package; every external claim
  carries a verification status.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.11`
  Status: `pending`
  Goal: record ADR-0002 (UI stack + canvas hosting) as a decision **structure**: chrome choice,
  the three canvas topologies, the egui/iced dev-shell ruling, the TypeScript domain-logic ban,
  and the exact G1 spike protocol whose evidence settles canvas hosting.
  Acceptance: the spike's measurements, pass/fail criteria and decision rule are written now,
  so the G1 outcome cannot be argued after the fact.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.12`
  Status: `pending`
  Goal: specify approval states and the release contract (§9): manifest contents, approval
  binding to package identity, stale-ification, package-completeness checking, the graduated
  acceptance states, and the human-only approval rule for agents.
  Acceptance: every manifest field is named with its source; the graduated states are ordered
  with the evidence each requires; the scoped-approval and scope-narrowing rules are stated.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.13`
  Status: `pending`
  Goal: specify the **reference skirt** with numbers — A-line, one waist dart per side, CB
  zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem, single notches at side seams, cut-on-fold or
  paired front — plus the measurement table, formulas, piece list, seam/sewing graph, notch and
  grainline placements that make it the corpus fixture for G2.
  Acceptance: a reader can draft the garment from the chapter without asking a question; every
  number has a source (measurement, formula or declared constant); it exercises dart, grain,
  variable SA, notch, fold and the included/excluded allowance policy as the roadmap requires.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.14`
  Status: `blocked` (director input: named humans)
  Goal: draft the governance model (§12) — sewist-vs-programmer review paths, domain review of
  profile changes that alter exported bytes, golden-file approval ownership, funding/procurement
  owners — and record the roles that need a named person (evaluation seats, physical plotter,
  project owner), with the partner-run manual test as documented fallback.
  Acceptance: the model is written and reviewable; each role states what authority it needs;
  the named-person gaps are flagged to the director in one place, not discovered later.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.15`
  Status: `pending`
  Goal: G0 exit review — walk the gate's exit clause list, cite the deliverable and its
  re-derivable check for each, update `LIVE_STATUS.md` and the roadmap status line, and hand
  the frontier to `G1-SLICE`.
  Acceptance: every clause is `met` with a cited artifact or `not met` with a named blocker;
  no clause is marked met on prose alone.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.16`
  Status: `pending`
  Goal: choose the ONE message system (§7.6: Fluent **or** ICU — not "Fluent or ICU"; if both ends
  are needed, a designed bridge) and specify the externalization architecture: the CI lint that
  fails on an inline user-facing string, the glossary/termbase per language (safety-relevant terms
  first: notch types, sew/cut line aliases factories use in rejection emails), pseudolocalization,
  and locale-independent canonical files (decimal-comma input ≠ stored meaning).
  Acceptance: a decision record names the chosen system, the rejected one and the reason; the
  architecture chapter states the lint rule, the termbase format and the RTL rule (mirrored layout,
  never mirrored geometry); stable diagnostic codes + typed arguments + units are the API contract,
  with localized prose as a presentation field only.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.17`
  Status: `pending`
  Goal: the command-layer contract (§4.4) — the typed command set, atomic groups, preview/commit,
  revision preconditions, idempotency, structured errors, progress for long operations, and the
  **undo/redo semantics that §4.4 requires to be defined at G0** (granularity per command group),
  plus the UI↔API↔MCP workflow-parity invariant and the shape of the coverage table that proves it.
  Acceptance: each command class states its granularity, reversibility and precondition; the parity
  table's columns and its generation rule are specified so G1 can populate it mechanically;
  agent authority levels (§7.8: inspect / propose / commit / generate / approve) are defined here as
  command-layer concepts, not as tool descriptions.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.18`
  Status: `done`
  Goal: the G0 CI shape (§4.3, §7.3) — minimal `sc-units` and `sc-core` crate skeletons in the
  workspace, and a workflow running fmt / clippy / unit+property tests / the WASM smoketest that
  proves those two crates compile to `wasm32-unknown-unknown`.
  Acceptance: CI green on all four steps; the WASM step is a real `cargo build --target
  wasm32-unknown-unknown`, not a `cargo check` on the host; the skeletons carry no domain logic
  beyond what the `.2`/`.3` specs already fix (types and invariants may land, behaviour may not);
  the starter crate question is answered — retired here or explicitly handed to `G1-SLICE.1`.
  Verification: recorded below — 30 tests green, WASM cross-build green, clippy at deny-warnings.
  Commit: `STITCHCAD-G0-0018`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `G0-CONTRACT.2` | `done` | taken first: every other chapter and every crate quotes a number, so the numerical contract had to exist before them |
| 2 | `G0-CONTRACT.18` | `done` | `sc-units` implements `.2`; the G0 CI workflow builds both crates for WASM |
| 3 | `G0-CONTRACT.3` | `pending` | **next** — the ontology is the largest chapter and the dependency of `.4`, `.5`, `.13` |
| 4 | `G0-CONTRACT.13` | `pending` | the reference skirt gives every later chapter a concrete garment to be checked against |
| 5 | `G0-CONTRACT.1`, `.4`–`.12`, `.14`–`.17` | `pending` | remaining G0 chapters, in the order the frontier table below records |

## Decisions

- `2026-09-29`: normative G0 specifications live in the mdBook under `docs/book/src/spec/`
  (the director reviews the book); ADR-style *decisions* live in `docs/decisions/` (memory
  layer C) and are summarised — never duplicated — in the book. Realises roadmap §4.3's
  `docs/adr/` as `docs/decisions/` deliberately.
- `2026-09-29`, **superseding the same day's earlier "no product code in G0" reading**: G0 owns
  exactly two skeleton crates and the G0 CI workflow (`.18`), because §4.3 and §7.3 state the G0
  CI shape and the G0 WASM smoketest in terms of `sc-core` + `sc-units` compiling. The earlier
  reading took "crates appear when their stage starts" as an absolute and would have left two
  roadmap clauses unowned. Consequence: `.18` is a CODE leaf and carries the full acceptance
  checklist with tool output; every other G0 leaf remains documentation-only.

## Open Questions

- Cut-on-fold vs paired front for the reference skirt: roadmap §11 G0 allows either. Decided
  in `G0-CONTRACT.13` with the reason recorded there (it changes the piece list and the
  notch/fold semantics the fixture must exercise).
- Which named drafting system ships as the v1 reference block set (roadmap ADR-0003: "one
  documented system, decided at G0") — decided in `G0-CONTRACT.9`.

## Blockers

- `G0-CONTRACT.14` needs named humans (project owner, procurement owner) from the director.
  It does not block `.1`–`.13` or `.15`; `.15` records it as `not met — named owner pending`
  if still unresolved.

## Acceptance Checklist

### `G0-CONTRACT.2` — the numerical contract is written down

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit list requires a units & tolerance policy, and before
  this leaf it existed only as roadmap prose (§4.2: "Single internal unit: fixed-point micrometers
  (i64) recommended" — note *recommended*, i.e. still a choice): `git ls-tree --name-only HEAD
  docs/book/src/spec/` → `index.md` alone, `rc=0`, and `git ls-files 'crates/*'` → the bedrock starter
  crate, `rc=0`. Two implementers reading §4.2 would have chosen different angle units, different
  rounding and different epsilons, and the divergence would have surfaced as golden-file diffs at G2.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/units-and-tolerances.md` is now the normative
  chapter (`wc -lc` → `291` lines / `17180` bytes, widest line `114` bytes), linked from `SUMMARY.md`,
  and the book builds:
  `make book` → `INFO HTML book written to …`, `exit=0`, producing `docs/book/book/spec/units-and-tolerances.html`.
  It fixes the open choices: i64 micrometres for length, i64 microdegrees for angle, a declared domain
  tighter than the type, half-away-from-zero rounding, single-step conversions, five tolerance classes
  each with its setter and derivation, exact integer topology predicates, and the offset error budget.
  Every arithmetic claim in it was re-derived rather than recalled: `1016 × 25 = 25400` (so an HPGL
  plotter unit is exactly 25 µm), `25400 ÷ 72 = 352.78` µm per PDF point (not an integer, hence the
  format-quantization class), `10¹⁸` needs `60` bits and `2 × 10¹⁸` needs `61` (so i128 products of
  domain-bounded coordinates are exact), `10⁹ µm = 1 km`, `10¹⁸ µm² = 1 km²`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 47 files
  measured`, `exit=0`, with the new chapter claimed by `book_collection`; `make check` →
  `test result: ok. 1 passed; 0 failed`; `make book` → `exit=0`.
  The containment ceiling set two slices earlier earned its keep on this chapter: the first draft
  carried five tolerance-class table rows of `311`–`381` bytes against a `book_collection` maxline
  ceiling of `320`, so the widest was over the limit before the chapter ever shipped. The table became
  five bounded subsections (widest line now `114` bytes) — which is also the more readable shape in a
  book, and the reason maximum-content-line is a separate axis from lines and bytes.
- [x] **FIX** — wrote the chapter; recorded the coupled choices as
  `docs/decisions/decision_numerical-contract-fixed-point.md` (indexed, with an `answers:` line) so the
  *why* survives separately from the *what*; labelled every external claim in §8 of the chapter with
  its verification status (exact arithmetic / cited-from-roadmap / to be confirmed at the gate that
  needs it) instead of asserting format details nobody has read here.
- [x] **LOCKSTEP** — `SUMMARY.md` grows the chapter; `LIVE_STATUS.md` moves G0 to In Progress;
  `MEMORY.md` points at the next leaf; `CHANGELOG.md` records the slice; the derived Knowledge Map
  picks up the new decision record.

### `G0-CONTRACT.18` — the first product code: `sc-units` implements the numerical contract

- [x] **ROOT CAUSE (WHY + WHERE)** — the roadmap's G0 CI clause (§4.3 "CI grows with stages (G0:
  fmt/clippy/unit+property/WASM smoketest)", §7.3 "WASM CI at G0 = smoketest that `sc-core` +
  `sc-units` compile to `wasm32-unknown-unknown`") had nothing to build:
  `git ls-tree -r --name-only HEAD | grep -c 'crates/sc-'` → `0`, `rc=1`, and the only crate in the
  workspace was the bedrock starter, whose `main` printed `bedrock: replace this crate with your
  project` (defect D10). A WASM smoketest over zero crates is a green light on nothing.
- [x] **ADDRESSED (verified)** — `crates/sc-units` (1 097 lines of library, 564 lines of tests,
  **zero dependencies** so it serves the `wasm-viewer` profile) now implements the `.2` chapter:
  `Length` (i64 µm), `Angle` (i64 µ°, normalized), `Area`, `Ratio` (ppm), `Count`, `Unit` with exact
  integer ratios, the five `ToleranceClass`es with a **mandatory derivation** field, `UnitError`
  diagnostics for domain/overflow/division-by-zero/non-finite, and `#![forbid(unsafe_code)]`.
  `crates/sc-core` exists as a documented skeleton naming which leaf owns each future module.
  `cargo test --all` → **30 tests, 0 failed** (21 conformance properties + 5 rounding unit tests +
  3 skeleton tests + 1 doc-test); `make wasm` →
  `wasm-viewer smoketest: sc-units + sc-core build for wasm32-unknown-unknown`; the CI workflow gained
  the `wasm32-unknown-unknown` target and a smoketest step that lists the produced `.rlib` files.
- [x] **NO REGRESSION** — `cargo fmt --all -- --check` → clean, `exit=0`;
  `cargo clippy --all-targets --all-features -- -D warnings` → `Finished`, `exit=0`;
  `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` → `7 suite(s) green`. The
  starter crate was retired with `git rm crates/app` (its only content was a template message and a
  `2 + 2` test), so D10 closes here rather than at `G1-SLICE.1`.
- [x] **FIX** — added `crates/sc-units` (7 modules) and `crates/sc-core`; workspace lints tightened
  (`unsafe_code = "forbid"`, `missing_docs`, and `unwrap_used`/`expect_used`/`panic` denied so a
  geometry kernel cannot abort a session on a degenerate input — with `.clippy.toml` re-allowing them
  in tests, where a test that cannot fail loudly protects nothing); `rust-toolchain.toml` pins the
  wasm32 target; `Makefile` gained `make wasm`; the `clippy::all` group needed `priority = -1` to
  coexist with individual lint levels.
- [x] **The tests caught a real API defect before it shipped:** `Ratio` conflated *a percentage* with
  *a multiplier*. `from_percent_rational(2, 100)` returned 200 ppm — self-consistent, and a 100× error
  for anyone reading the name as "2 percent". The constructors are now `from_percent(2, 1)` (a
  percentage value → multiplier 0.02) and `from_rational(102, 100)` (the 1.02 a 2 % shrinkage is
  applied as), each documented with the confusion it prevents. Two further failures were wrong
  expectations on my side, corrected against the spec: `as_rational_in` returns the *reduced* exact
  ratio (1 µm = 9/3175 pt, not 72/25400), and a whole-point round trip is bounded by one internal
  quantum expressed in points (0.00283 pt), not by 1e-6.
- [x] **LOCKSTEP** — `knowledge-map/subsystems.md` now names both crates and the spec directory (the
  map was still reporting "no subsystems documented"); `TOOLBOX.md` gained the `make wasm` row;
  D10 closed in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and the derived Knowledge
  Map updated in this commit.

Gate-level closure is recorded by `G0-CONTRACT.15`; each leaf carries its own evidence in the
Verification Log, and `.18` (the code leaf) additionally fills a `### G0-CONTRACT.18` checklist
subsection with real tool output in the same commit as the change. This tree file carries no
unticked placeholder boxes: the spine's acceptance gate judges the FIRST matching box in a file, so
a placeholder shadows real evidence and falsely rejects honest work (defect D15, measured by the
`SPINE.7` probe; local mitigation `SPINE.8`).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-29` | `G0-CONTRACT.2` | `wc -lc` and per-line max on the chapter; `make book`; arithmetic re-derivation; `check_live_doc_size.sh`; `make gate`; `make check` | `291` lines / `17180` bytes / maxline `114`; `exit=0`, chapter rendered; 5 arithmetic claims confirmed; `OK — 17 surfaces, 15 routes, 47 files`; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `G0-CONTRACT.18` | `cargo test --all`; `make wasm`; `cargo fmt --check`; `cargo clippy -D warnings`; `make gate`; `make probes` | 30 tests / 0 failed; wasm cross-build green; fmt clean; clippy clean; `=== all doctrines green ===`; `7 suite(s) green` |
| `2026-09-29` | coverage gaps closed | roadmap clause census (§4.3, §4.4, §7.3, §7.6) | 3 clauses were unowned → `.16`, `.17`, `.18` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0001 (leaf PLANNING.1)` | created by the seeding leaf |
| `.16`–`.18` added | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | i18n choice, command-layer contract, G0 CI + skeletons |
| `G0-CONTRACT.2` | `STITCHCAD-G0-0002 (leaf G0-CONTRACT.2): the numerical contract` | first product specification chapter; decision record added |
| `G0-CONTRACT.18` | `STITCHCAD-G0-0018 (leaf G0-CONTRACT.18): the first product code` | `sc-units` implements `.2`; `sc-core` skeleton; G0 CI + WASM smoketest; D10 closed |
| `G0-CONTRACT.1`, `.3`–`.17` | `pending` | — |

## Changelog

- `2026-09-29`: `.18` landed — the first product code. `sc-units` implements the numerical contract
  (30 tests, dependency-free, builds for `wasm32-unknown-unknown`), `sc-core` is a documented
  skeleton, the G0 CI shape exists, and the bedrock starter crate is retired (D10).
- `2026-09-29`: `.2` landed — the units & tolerance chapter is normative, the numerical contract is a
  layer-C decision, and the frontier moves to `.18` (the first product code: `sc-units` implements this
  chapter). The tree's execution order is now recorded in its frontier table.
- `2026-09-29`: Tree created by `PLANNING.1` with 15 leaves mapped clause-by-clause to the
  roadmap's G0 exit criteria.
- `2026-09-29`: `PLANNING.2` added `.16` (message system + i18n architecture, §7.6), `.17`
  (command-layer contract incl. undo/redo granularity, §4.4) and `.18` (G0 CI + `sc-units`/
  `sc-core` skeletons, §4.3/§7.3), and corrected the tree's "no code in G0" reading.
