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
  Status: `done`
  Goal: the glossary of construction terms — one entry per term used anywhere in the spec
  set, each with a plain-language definition, the canonical object it names (ontology id),
  synonyms used by factories/other CADs, and the machine token that must never be rendered raw.
  Acceptance: every domain noun in roadmap §3.1 has an entry; every entry names its ontology
  object; the book chapter builds; terms used later in the spec set are all defined here.
  Verification: recorded below and in the acceptance checklist — 239 terms across 8 parts, the census at
  `0 failure(s)`, its 10 probe arms green, the book built, every gate green.
  Commit: `STITCHCAD-G0-0001`

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
  Status: `done`
  Goal: garment ontology v1 — normative field-level specification of MeasurementTable, Ease,
  Design (construction recipe), Piece, SeamSpan/SewingGraph, Dart/Tuck/Pleat/Gather,
  SeamAllowance, Notch, Grainline, Hem/Facing/Lining/Interfacing, Closure, Pocket,
  Corner/Truing/Walking operations, plus the persistent-identity contract (§4.1).
  Acceptance: every object has identity rules, required/optional fields, invariants, and its
  uncertainty states; references are to stable topological entities, never array indices;
  each object cites the roadmap clause it comes from.
  Verification: recorded below — chapter built into the book, inside its per-part bounds, gates green.
  Commit: `STITCHCAD-G0-0003`

- ID: `G0-CONTRACT.4`
  Status: `done`
  Goal: the supported / rejected / deferred feature matrix bounding the v1 release claim
  (woven family: A-line skirt with waist dart + CB zipper, darted bodice, set-in sleeve,
  classic collar, trousers), with the diagnostic each rejected construction must produce.
  Acceptance: every row is one of supported/rejected/deferred with a reason and a gate;
  nothing in the ontology is silently unlisted; non-goals (§1.3) appear as rejected rows.
  Verification: recorded below and in the acceptance checklist — 105 rows, 29 declared diagnostics, the
  coverage census at `0 failure(s)`, its 10 probe arms green, and the ontology/non-goal/envelope
  coverage all derived rather than asserted.
  Commit: `STITCHCAD-G0-0004`

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
  Status: `done`
  Goal: specify the **reference skirt** with numbers — A-line, one waist dart per side, CB
  zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem, single notches at side seams, cut-on-fold or
  paired front — plus the measurement table, formulas, piece list, seam/sewing graph, notch and
  grainline placements that make it the corpus fixture for G2.
  Acceptance: a reader can draft the garment from the chapter without asking a question; every
  number has a source (measurement, formula or declared constant); it exercises dart, grain,
  variable SA, notch, fold and the included/excluded allowance policy as the roadmap requires.
  Verification: recorded below — every derived value re-derived, the allocation balance closes,
  book builds, gates green.
  Commit: `STITCHCAD-G0-0013`

- ID: `G0-CONTRACT.13b`
  Status: `done`
  Goal: declare every machine token the reference fixture uses (defect **D26**), and record the
  waistband contradiction the token census found in `.13` (defect **D27**), so the chapter a G2 golden
  will be frozen over carries no undeclared name and no silently disputed number.
  Acceptance: every snake_case token in the chapter is declared by a table whose first cell names it;
  no derived value changes; the contradiction is stated in the chapter with both readings, their
  arithmetic, and the leaf that owns choosing between them; `G0-CONTRACT.1`'s census reports the
  chapter clean.
  Verification: recorded below — tokens used-and-undeclared went `19` → `6`, and the six left are
  glossary vocabulary, not fixture names; every §4 result is unchanged; the book builds.
  Commit: `STITCHCAD-G0-0013b`

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
| 3 | `G0-CONTRACT.3` | `done` | the ontology now exists, so `.4`, `.5` and `.13` have objects to refer to |
| 4 | `G0-CONTRACT.13` | `done` | the fixture now gives every later chapter a concrete garment to be checked against |
| 5 | `G0-CONTRACT.13b` | `done` | the fixture's undeclared tokens (D26) and its disputed waistband (D27), found while building `.1`'s census; the glossary's coverage rule cannot pass over a chapter that uses names it never declares |
| 6 | `G0-CONTRACT.1` | `done` | the glossary: 239 terms in 8 domain parts, one owner per machine token, ⚠ on the safety-relevant ones, and a census that derives its coverage |
| 7 | `G0-CONTRACT.4` | `done` | the supported / rejected / deferred matrix: 105 rows, 29 diagnostics, and a census that derives its coverage of the ontology, the non-goals and the envelope |
| 8 | `G0-CONTRACT.5` | `pending` | **next** — both instantiation paths: regeneration and `.rul` grade rules, where they diverge, and the declared equivalence tolerance. It also owns the three `.rul` attributes (`stack point`, `fixed perimeter`, `smoothing`) the glossary deliberately leaves unspecified |
| 9 | `G0-CONTRACT.6`–`.12`, `.14`–`.17` | `pending` | the remaining G0 chapters, in the order the acceptance table records |

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

- `2026-09-30`, leaf `.1`: the glossary is **partitioned into eight domain parts behind one index
  chapter**, not one file. Reason, measured: a single file would have been ~70 KB of five-column rows
  against a `book_collection` per-part health of 24 576 bytes and a ceiling of 40 960 — a termbase is not
  the shape of a prose chapter, and one file would have breached its ceiling within two more chapters.
  Consequences: the A–Z index is **derived** from the parts and compared against them by the census in
  both directions; a new term goes in the part whose domain it belongs to and in no other; every part
  carries the same five columns, so the termbase extraction `G0-CONTRACT.16` owes has one shape to read.
- `2026-09-30`, leaf `.1`: a **machine token has exactly one owner**, and an entry that shares another's
  token writes `→ token`. Recorded as `docs/decisions/decision_machine-tokens-declared-where-used.md`
  with the measurement behind it, because the rule binds every later chapter and every crate.

- `2026-09-30`, leaf `.4`: a feature matrix row whose proof no gate has accepted says **`unnamed (D32)`**
  rather than borrowing a gate. Assigning a gate would invent a commitment on that gate's behalf, and a
  silently borrowed gate is how an envelope claim becomes untestable. The census prints those rows on
  every run (advisory `A1`), so the gap is closed by a decision at `.15`, not by being forgotten.
- `2026-09-30`, leaf `.4`: a citation in the matrix's reason column repeats its source per clause
  (`ontology §1, ontology §1.1`, never `ontology §1 and §1.1`), because the census reads citations
  mechanically and a bare `§1.1` after a comma is ambiguous between two documents with overlapping clause
  numbers.

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

### `G0-CONTRACT.3` — the garment ontology is written down

- [x] **ROOT CAUSE (WHY + WHERE)** — the ontology existed only as roadmap prose: §3.1 lists the objects
  in bullets with no fields, no invariants and no identity rules, so `sc-core`'s skeleton could name the
  module it owes (`grep -c 'G0-CONTRACT.3' crates/sc-core/src/lib.rs` → `1`, `rc=0`) but had nothing to
  implement against. `git ls-tree --name-only HEAD docs/book/src/spec/` → `index.md`,
  `units-and-tolerances.md`, `rc=0`: no ontology chapter, while `.4`, `.5`, `.13`, `G1-SLICE.3` and
  `G1-SLICE.4` all depend on it.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/ontology.md` now specifies all of it:
  `wc -lc` → `295` lines / `16187` bytes, widest line `198` bytes (inside the `book_collection`
  per-part health of 400 lines / 24 576 bytes / 200 B). It covers ULID entity identity and the stable
  topological reference contract with a per-edit preservation table (split, merge, reverse, delete,
  offset-fragmentation → preserved or a visible repair task, never silent reassignment);
  `MeasurementTable` with landmark and procedure mandatory and body-vs-POM kinds distinct; `Ease` as a
  first-class body→garment mapping with fit intent; `Design` as recipe (formula graph + ordered
  operations + revision) with `walk`/`true` as first-class operations; `Piece`, `SeamSpan`/`SewingGraph`,
  `Dart`/`Tuck`/`Pleat`/`Gather`, `SeamAllowance` as a per-edge derived object with profile-resolved
  inclusion, `Notch`, `Grainline`, hem/facing/lining/interfacing/closure/pocket, materials, the five
  uncertainty states, canonical serialization, and seven named test obligations. Wired into
  `SUMMARY.md` and the spec index; `make book` → `INFO HTML book written to …`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 50 files
  measured`, `exit=0`; `make check` → `test result: ok. 3 passed; 0 failed` (30 across the workspace);
  `make probes` → `7 suite(s) green`. No product code changed, so no acceptance evidence was owed
  beyond this record.
- [x] **FIX** — wrote the chapter; converted its verification-status table to bounded prose after the
  containment checker reported a `238`-byte row against a `200`-byte health target (the chapter now sits
  at `198`); linked it from the spec index and `SUMMARY.md`.
- [x] **LOCKSTEP** — `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` updated; `knowledge-map/subsystems.md`
  already names `docs/book/src/spec/` as a subsystem, so the derived map needed no edit;
  `sc-core/src/lib.rs`'s module table already points at this leaf as the specifier.

### `G0-CONTRACT.13` — the reference skirt, specified to the millimetre

- [x] **ROOT CAUSE (WHY + WHERE)** — gate G2's exit criteria and the mutation, golden, offset and agent
  suites all need one subject garment, and the roadmap's G0 fixture clause described it in prose only:
  "A-line, one waist dart/side, CB zipper, grain ∥ CB, SA 1 cm sides / 3 cm hem, single notches at side
  seams, cut-on-fold or paired front". Two implementers would have produced two different skirts, and
  every comparison between them would have been meaningless. `git ls-tree --name-only HEAD
  docs/book/src/spec/` → `index.md`, `ontology.md`, `units-and-tolerances.md`, `rc=0`: no fixture.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/reference-skirt.md` (`wc -lc` → `245` lines /
  `14538` bytes, widest line `118`) specifies the garment completely: 4 body measurements with
  landmarks, 2 ease entries with fit intent, 9 declared drafting constants, **17 derived values each
  with its formula**, a 9-step drafting recipe, 6 pieces with multiplicity/fold/pair/material/layer,
  per-edge allowances with corner treatment and policy, notch and grainline placement, a 4-span sewing
  graph with declared zero ease, the closure, a fixture-property → ontology-clause → dependent-gate
  traceability table, and 8 test obligations. The arithmetic was re-derived rather than asserted, and
  the allocation **balance closes exactly**: `4 × (3.0 + 4.0) = 28.0` = `garment_hip − garment_waist`
  = `102.0 − 74.0`; side seams total `12.0` cm and darts `16.0` cm, summing to the same `28.0`.
- [x] **NO REGRESSION** — `make book` → `INFO HTML book written to …`, `exit=0` with the chapter
  rendered; `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 51
  files measured`, `exit=0`; `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 3 passed; 0 failed` (no product code changed).
- [x] **FIX** — wrote the chapter; **recorded an interpretation rather than silently choosing one**
  (the roadmap's "one waist dart/side" is realised as one dart per quadrant, with the arithmetic reason:
  a single dart per body side would need 8.0 cm of intake); marked the 5 drafting constants and 3
  closure constants as `assumed` pending domain review, with `G0-CONTRACT.14` named as the leaf that
  owns naming the reviewer, so a golden frozen at G2 cannot freeze an unreviewed guess as if it were a
  fact; converted the §11 verification table to bounded prose when the containment checker reported a
  `276`-byte row against the `200`-byte book health target.
- [x] **LOCKSTEP** — chapter linked from `SUMMARY.md` and the spec index; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` updated; the fixture's test obligations are the G2 leaves' acceptance inputs, so
  `G2-2D.3`/`.8`/`.11` now have numbers to assert against.

### `G0-CONTRACT.1` — the glossary, and the census that derives its claims

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit clause list requires a "glossary of construction terms"
  and none existed: `git ls-tree --name-only HEAD docs/book/src/spec/` → `index.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0`, and
  `git ls-tree -r --name-only HEAD docs/book/src/spec/glossary/ | wc -l` → `0`. Three normative chapters
  were already using the vocabulary with nothing binding a term to one meaning — `ontology.md` said
  "Terms used below are defined in the glossary" and pointed at a file that did not exist. The population
  needing definitions is measurable rather than a matter of taste: the census counts `82`
  identifier-shaped machine tokens and `183` bolded spans across the spec chapters. The blocking part was
  worse than absence: the reference fixture used `19` tokens no table declared (D26), so a glossary written
  over that chapter would have defined words the chapter's own formulas contradicted — hence `.13b` first.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/glossary.md` plus eight domain parts under
  `docs/book/src/spec/glossary/` carry **239 terms**, each with a plain-language meaning, the canonical
  object that specifies it, the synonyms factories and other CADs use, and the machine token; `150` tokens
  are owned, `19` cross-referenced with `→`, `22` entries are required to carry the safety mark and `72`
  do. Every claim the glossary makes about itself is derived, not asserted:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` →
  `glossary census: 239 terms / 8 parts / 138 tokens / 0 failure(s)`, `exit=0`, with `155` canonical-object
  references resolved and `0` dead, `82` tokens used by the spec set and `0` unaccounted, index↔parts
  drift `0`. Its discrimination is proved, not assumed:
  `bash docs/tasks/artifacts/glossary/run_glossary_probes.sh` → `probes: 10 pass / 0 fail` (two GREEN arms
  and eight RED arms that break one property each in a copy and require the census to name it). The book
  builds with all nine glossary pages: `make book` → `INFO HTML book written to …`, `exit=0`,
  `ls docs/book/book/spec/glossary/*.html | wc -l` → `8`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `9 suite(s) green` (`62` arms across nine suites, `0 fail`); `make check` → `test result: ok. 21 passed;
  0 failed` and `ok. 5 passed` and `ok. 3 passed` across the workspace, no product code touched;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 62 files measured`,
  `exit=0`. Containment is respected per part (largest glossary part `48` lines / `7 471` B against a
  health of `400` / `24 576`; the index chapter `369` lines / `21 052` B) with **one axis deliberately over
  health and inside its ceiling**: the widest table row is `268` B against a `200` B maxline health and a
  `320` B ceiling, because a five-column termbase row is not the shape the prose-derived health target was
  measured from. It is recorded here rather than paid for in vaguer definitions; if a later chapter pushes
  the collection past `320` B the answer is a recorded decision and a table-shaped health target, not
  silent trimming.
- [x] **FIX** — wrote the index chapter (the machine-token rule, the ⚠ policy, the parts table, the
  derived A–Z index, the termbase relationship to `G0-CONTRACT.16`, and the verification status of every
  synonym class) and the eight parts; partitioned rather than filed as one ~70 KB chapter, which would
  have breached the `book_collection` per-part ceiling within two more chapters; made the index derived
  (`--emit-index`) and compared in both directions so it cannot drift; built the census as the tracked
  producer and the probe suite as its ground truth; added
  `docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` because this slice's changelog append
  crossed the rollover milestone and the doctrine's step 6 requires ordering, uniqueness and retrieval
  checks that nothing executed before.
- [x] **The probes found three defects in the census before it found one in the glossary** — which is the
  argument for RED arms over reading your own code. (1) `R1` read the *meaning* column instead of the
  object column, so it examined no cell at all and printed `dead references: 0` — a green verdict on a
  rule that never ran; only `DEAD-CLAUSE` failing to fire revealed it. (2) The clause slice was one byte
  off: `§` is two bytes and `LC_ALL=C` makes awk count bytes, so every extracted clause carried a stray
  `\xa7` and matched nothing. (3) `resolve()` collapsed `a/b/../c.md` by deleting `/../`, leaving
  `a/b/c.md`, so all `155` references reported dead until the pattern also removed the parent segment.
  All three were caught by probes; none by inspection.
- [x] **The rollover this slice triggered is performed and verified in the same commit** — the live window
  crossed its health target, so the five oldest entries are sealed into
  `docs/history/stitchcad-changelog-part2.md` (`152` lines / `12 811` B / `sha256:5783ac36d8bc7cee…`), the
  window is reordered into commit order (D29), the stale "_Inherited spine history_ divider" sentence is
  corrected (D31), and part1's wrong coverage line is corrected by superseding record rather than by
  editing an immutable segment (D30). Losslessness is proved, not claimed: all `12` pre-existing entries
  have byte-identical bodies across live+sealed, `0` missing, `0` changed, no id both live and sealed; and
  the ledger probe → `probes: 6 pass / 0 fail`, including part1's `365` lines still reproducing
  `sha256:f4aec75ac7dd1fa5…`.
- [x] **LOCKSTEP** — `SUMMARY.md` gains the glossary and its eight parts; `spec/index.md` links the
  Glossary row; `ontology.md`'s forward reference resolves; `TOOLBOX.md` gains the census and its probe
  suite; `knowledge-map/subsystems.md` names the glossary as a subsystem (the derived map picks up the new
  decision record); `docs/decisions/decision_machine-tokens-declared-where-used.md` is the layer-C record
  the `DEV_NOTES.md` lesson is promoted to; D29/D30/D31 closed in `PLANNING.md`; `LIVE_STATUS.md`,
  `MEMORY.md` and `CHANGELOG.md` updated in this commit.

### `G0-CONTRACT.4` — the release claim gets a boundary, and the boundary is derived

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit list requires a "supported/rejected/deferred feature
  matrix" and roadmap §3.2 promises "A supported/rejected/deferred feature matrix is a G0 deliverable",
  but the boundary existed only as prose in two places that do not enumerate: `git ls-tree --name-only
  HEAD docs/book/src/spec/` → `glossary/`, `glossary.md`, `index.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0` — no matrix. Without one, three populations
  stay unchecked: the ontology's object clauses (a feature nobody dispositioned is a feature a factory
  discovers), roadmap §1.3's non-goals (a refusal a partner does not know about is a refusal they test),
  and roadmap §3.2's envelope. The gap is not hypothetical — writing the rows surfaced **D32**: collar,
  trousers, buttons, pockets and the fly they imply are inside the declared envelope and outside every
  gate's exit criteria, `sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`
  → `0`.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/feature-matrix.md` (`306` lines / `28 646` bytes,
  widest line `197`) dispositions **105 rows**: `76` supported, `19` rejected, `10` deferred; `29`
  diagnostic tokens are declared in §10 with the arguments each must carry, and §1 fixes the three rules
  (no silent approximation, a supported row names its proof, modelled is not supported). Its acceptance
  clause — "nothing in the ontology is silently unlisted" — is derived, not asserted:
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` →
  `feature-matrix census: 105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `16` required ontology
  object clauses all cited, `8` of `8` roadmap §1.3 non-goals matched to a rejected row (the mapping is
  printed), `5` of `5` envelope garments supported and `3` of `3` named refusals rejected, and every gate
  cell resolving to a real roadmap §11 gate or to `unnamed (D32)`. The glossary absorbed the `26` terms
  the matrix introduces (`239` → `265`), re-derived:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `265 terms / 8 parts / 139 tokens /
  0 failure(s)`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `10 suite(s) green` (`72` arms, `0 fail`), including the new
  `run_feature_matrix_probes.sh` → `probes: 10 pass / 0 fail` and the glossary suite still at
  `10 pass / 0 fail`; `make check` → `5` `test result: ok` lines across the workspace, no product code
  touched; `make book` → `INFO HTML book written to …`, `exit=0` with `6` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 64 files measured`, `exit=0`.
  Containment is inside every ceiling with two axes over *health* and recorded here rather than trimmed:
  the matrix part is `28 646` B against a `24 576` B per-part health (117 %, ceiling `40 960`), and the
  glossary's widest table row is `268` B against a `200` B health (ceiling `320`) — a five-column
  reference table is not the prose shape those health targets were derived from. `subsystems_input` is at
  `3 131` B against `3 072` (102 %, ceiling `6 144`); its registry row already names `SPINE.5` as the
  owner of re-deriving that target.
- [x] **FIX** — wrote the chapter (dispositions and their three rules, the garment family, five feature
  sections, the non-goal table, the D32 gap table, the diagnostic contract, what G7's statement must
  carry, verification status, the derivation, and the test obligations); added `26` glossary entries and
  regenerated the A–Z index with `--emit-index`; built the census as the tracked producer and its probe
  suite as ground truth; logged D32 with the census as its reproduce command so the gap stays visible on
  every run. The DEV_NOTES lesson's promotion decision, on one line because the gate reads lines:
  promotion: declined (the rule binds probe authors, so it went into TOOLBOX.md's probe conventions, which CLAUDE.md step 3 sends every agent to; a layer-C record would duplicate it).
- [x] **Two probe arms were wrong before the census was right, and the census was right** — measured, not
  assumed. `UNDECLARED-DIAG` renamed `` `ngo_costing` `` everywhere, which renames the §10 declaration and
  its row together and therefore changes nothing the census can see; `UNCITED-ONTOLOGY` de-cited one of the
  three rows citing `ontology §4.4`, leaving the clause covered. Both arms passed for the wrong reason —
  exit `0` where a refusal was owed — until each mutation became the *only* copy of the property. The
  general rule, which is the same one D15's shadowing probe taught: **a RED arm must remove the property,
  not one instance of it.** A third bug was mine and not the arms': replacing a `{2,4}` interval with
  `###+` for portability silently dropped every `##`-level heading, so M3 required three clauses fewer
  and reported green. Only `REAL` plus the citation arm caught it, which is why both exist.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; the glossary grew `26` terms and
  re-derived its index; `TOOLBOX.md` gains the matrix census and its probe suite (and the changelog-ledger
  probes the previous slice added but had not listed); `knowledge-map/subsystems.md` names the matrix as a
  subsystem, so the derived map carries it; D32 logged in `PLANNING.md`; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `docs/TASK_TREE.md` updated in this commit.

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
| `2026-09-29` | `G0-CONTRACT.3` | `wc -lc` + per-line max on the chapter; `make book`; `check_live_doc_size.sh`; `make gate`; `make check`; `make probes` | `295` lines / `16187` B / maxline `198`; `exit=0` and rendered; `OK — 17 surfaces, 15 routes, 50 files`; all gates green |
| `2026-09-29` | `G0-CONTRACT.13` | arithmetic re-derivation incl. the allocation balance; `wc -lc` + per-line max; `make book`; `check_live_doc_size.sh`; `make gate` | balance `28.0 = 28.0`; `245`/`14538`/maxline `118`; `exit=0`; `OK — 17 surfaces, 15 routes, 51 files`; all green |
| `2026-09-29` | `G0-CONTRACT.18` (CI verdict, observed after the exceptional push) | `git push origin main`; GitHub Actions runs API for `head_sha=119946b` | `051a075..119946b  main -> main`, ahead `0`; **`rust` completed `success`** (first execution of the new wasm32 smoketest step) and **`doctrines` completed `success`**; `runs: 2`, both concluded |
| `2026-09-29` | coverage gaps closed | roadmap clause census (§4.3, §4.4, §7.3, §7.6) | 3 clauses were unowned → `.16`, `.17`, `.18` |
| `2026-09-29` | `G0-CONTRACT.13b` | token census over the fixture at `HEAD` vs the working tree; `python3` re-derivation of both waistband readings; `make book`; `make gate` | undeclared tokens `19` → `6`, the six being glossary vocabulary; folded band `10.0` cm vs faced `6.0` cm per piece, so §4 and §6 are different garments (D27); no §4 result changed; `exit=0` both |
| `2026-09-30` | `G0-CONTRACT.4` | feature-matrix census; both probe suites; glossary census; `make book`; `make gate`; `make probes`; containment | `105 rows / 29 diagnostics / 0 failure(s)`; `probes: 10 pass / 0 fail` twice; `265 terms / 8 parts / 0 failure(s)`; `exit=0`, 6 spec pages; `=== all doctrines green ===`; `10 suite(s) green` |
| `2026-09-30` | `G0-CONTRACT.1` | glossary census; both probe suites; `make book`; `make gate`; `make probes`; `make check`; containment | `239 terms / 8 parts / 138 tokens / 0 failure(s)`; `probes: 10 pass / 0 fail` and `6 pass / 0 fail`; `exit=0`, 9 pages rendered; `=== all doctrines green ===`; `9 suite(s) green` — detail in the checklist |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| tree seed | `STITCHCAD-PLANNING-0001 (leaf PLANNING.1)` | created by the seeding leaf |
| `.16`–`.18` added | `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)` | i18n choice, command-layer contract, G0 CI + skeletons |
| `G0-CONTRACT.2` | `STITCHCAD-G0-0002 (leaf G0-CONTRACT.2): the numerical contract` | first product specification chapter; decision record added |
| `G0-CONTRACT.18` | `STITCHCAD-G0-0018 (leaf G0-CONTRACT.18): the first product code` | `sc-units` implements `.2`; `sc-core` skeleton; G0 CI + WASM smoketest; D10 closed |
| `G0-CONTRACT.3` | `STITCHCAD-G0-0003 (leaf G0-CONTRACT.3): the garment ontology` | 12 object families, the identity contract, 7 test obligations |
| `G0-CONTRACT.13` | `STITCHCAD-G0-0013 (leaf G0-CONTRACT.13): the reference skirt, to the millimetre` | 17 derived values with formulas; 5 constants flagged `assumed` pending review |
| `G0-CONTRACT.13b` | `STITCHCAD-G0-0013b (leaf G0-CONTRACT.13b): the fixture declares its tokens` | D26 and D28 fixed; D27 logged, owned by `.14`, and recorded in the chapter |
| `G0-CONTRACT.1` | `STITCHCAD-G0-0001 (leaf G0-CONTRACT.1): the glossary` | 239 terms, 8 parts, one census and two probe suites; D29/D30/D31 fixed by the rollover |
| `G0-CONTRACT.4` | `STITCHCAD-G0-0004 (leaf G0-CONTRACT.4): the supported envelope` | 105 rows, 29 diagnostics; D32 logged and kept visible by the census |
| `G0-CONTRACT.1`, `.4`–`.12`, `.14`–`.17` | `pending` | — |

## Changelog

- `2026-09-30`: `.4` landed — the v1 release claim has a boundary: 105 rows dispositioned
  supported/rejected/deferred, 29 declared diagnostics with their required arguments, and a census that
  derives the coverage of the ontology (16 object clauses cited), of roadmap §1.3 (8 non-goals rejected)
  and of §3.2 (5 envelope garments supported, 3 named refusals rejected). The glossary absorbed the 26
  terms the matrix introduces (239 → 265). D32 records the five rows no gate has accepted.
- `2026-09-30`: `.1` landed — the glossary is normative vocabulary: **239 terms** in 8 domain parts behind
  an index chapter carrying the machine-token rule, the safety-relevant (⚠) policy and the derived A–Z
  index. Its claims are derived by `run_glossary_census.sh` (`0 failure(s)`) and its discrimination proved
  by `run_glossary_probes.sh` (`10 pass / 0 fail`). The changelog rollover this slice's append triggered is
  performed and verified in the same commit (D29, D30, D31), with `run_changelog_ledger_probes.sh` as the
  standing verifier. The frontier moves to `.4`, the feature matrix.

- `2026-09-29`: `.13b` landed — the reference fixture now declares every machine token it uses
  (D26: `19` used-and-undeclared → `6`, all of them glossary vocabulary), its formulas are written over
  tokens only, and the waistband contradiction the token census exposed is recorded in the chapter as
  D27 with `.14`'s domain review as its owner (D28, a wrong clause cross-reference, fixed on the way).
- `2026-09-29`: `.13` landed — the reference skirt is fully specified with numbers, its allocation
  balance closes exactly, and its 8 unreviewed constants are explicitly `assumed` with an owner named
  for the review.
- `2026-09-29`: `.3` landed — the garment ontology is normative: identity and the persistent-reference
  contract, measurement/ease/size-set, the design-as-recipe, every geometry-bearing object with its
  invariants, uncertainty states, serialization and the test obligations that follow.
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
