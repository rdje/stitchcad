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
  Status: `done`
  Goal: both instantiation paths specified — measurement-driven regeneration and grade-rule
  instantiation (`.rul`: incremental vs cumulative, stack-point / fixed-perimeter /
  smoothing attributes) — including where they diverge and the declared equivalence
  tolerances.
  Acceptance: the known information loss between the paths is stated, not hidden; extreme-size
  checking after target-system reconstruction is specified; each path names its inputs,
  outputs and oracle.
  Verification: recorded below — the worked example re-derived (both paths agree to `0.00e+00` on this
  fixture, and the chapter says why that is the fixture's property and not a general result), the loss
  stated in three parts, eight extreme-size checks each with its class, the book built, gates green.
  Commit: `STITCHCAD-G0-0005`

- ID: `G0-CONTRACT.6`
  Status: `done`
  Goal: decide and record SizeSet ownership (default per §3.4: referenced by Design,
  overridable per Factory Profile with a recorded transformation).
  Acceptance: a decision record with context/decision/consequences; the spec chapter states
  the object's fields, label-vs-order semantics, base size, multi-dimensional charts and the
  EN 13402 / ASTM D5585 mappings it must express.
  Verification: recorded below — the record carries context, three candidate owners, the decision and its
  consequences; the chapter states all ten fields, the label/order separation, the base rules, breaks,
  axes and the five systems; both censuses green; the book builds.
  Commit: `STITCHCAD-G0-0006`

- ID: `G0-CONTRACT.7`
  Status: `done`
  Goal: identify the measurement/POM standards the model draws on (ISO 8559, ASTM D5219,
  EN 13402, ASTM D5585) — what each is used for, what is adopted, what is deliberately not,
  and the verification status of every claim about a standard's content.
  Acceptance: each standard has a named role and a claim-verification status
  (read-in-repo / cited-from-roadmap / unverified-with-owner); no clause number or table is
  asserted without a source.
  Verification: recorded below and in the acceptance checklist — 6 designations registered, all
  `cited-from-roadmap`, every deferral from three other chapters dispositioned, the census at
  `0 failure(s)` and its 6 probe arms green, and not one quoted clause anywhere in the book.
  Commit: `STITCHCAD-G0-0007`

- ID: `G0-CONTRACT.8`
  Status: `done`
  Goal: record ADR-0001 (license × solver) as one coupled decision, per the roadmap default
  (permissive core; `slvs`/GPLv3 rejected; `sc-sketch` uses a custom or least-squares kernel).
  Acceptance: decision record states the options, the coupling, the BSL-1.1 distinction, the
  consequence for contributors and the re-open condition (a strong case from the G1 spike).
  Verification: recorded below — all five acceptance elements present, indexed with an `answers:` line, the
  workspace licence field re-read from `Cargo.toml` rather than recalled, and the licence census already
  owned by `G1-SLICE.15` named as the enforcement.
  Commit: `STITCHCAD-G0-0008`

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

- ID: `G0-CONTRACT.13c`
  Status: `done`
  Goal: correct the reference fixture's waist geometry (defect **D33**) — §5 step 2 placed the waist side
  point at `quarter_waist − ss_suppress` (15.5 cm) where §3's `ss_suppress = 3.0` and §4's allocation
  balance require `quarter_hip − ss_suppress` (22.5 cm) — and give §4 the oracle that would have caught
  it, because the balance check the chapter calls its own invariant is blind to where the waist point sits.
  Acceptance: drafted literally, the fixture's finished waist equals its declared garment waist; every
  number the correction touches is re-derived and shown; the two dart-centre formulas are corrected to the
  span they meant; a waist-closure check joins §4 and §12; the correction is recorded in the chapter and
  the sealed changelog mention is superseded rather than edited; no other derived value changes.
  Verification: recorded below — all `18` derived rows re-derived from §2 and §3 with `0` mismatches, the
  finished waist `74.0` cm as declared instead of `46.0` cm, both closure checks closing.
  Commit: `STITCHCAD-G0-0013c`

- ID: `G0-CONTRACT.13d`
  Status: `done`
  Goal: resolve defect **D27** under the director's ruling of `2026-09-30`
  (`docs/decisions/decision_director-ruling-2026-09-30-four-findings.md`): choose ONE waistband
  construction for the reference fixture, make §4, §6, §8, §11 and §12 describe that one garment, and
  re-derive every number the choice touches. External sources may be read and must be cited with URL and
  date; a choice resting on general practice rather than a read source stays `assumed` with `.14`'s
  reviewer named.
  Acceptance: the finished waistband dimensions follow from §2 and §3 by stated formulas; the piece list,
  its layer indices and the sewing graph agree with the chosen construction and account for every piece
  (no piece without a span); both §4.1 closure checks still hold; `waistband_cut_width` is either corrected
  or replaced by per-piece widths; all 18+ derived rows re-derive with 0 mismatches; the glossary and
  feature-matrix censuses stay green; D27 closes with the decision recorded in the chapter.
  Verification: recorded below and in the acceptance checklist — the chapter re-derives at `20` derived
  rows / `4` closure checks / `5` pieces / `0 mismatch(es)` where the same instrument reported `7`
  mismatches over the chapter at `HEAD`; its probe suite is at `9 pass / 0 fail` and was shown sensitive
  by neutering one rule at a time; the glossary grew to `276` terms with `0 failure(s)`. Its clause "no
  piece without a span" is met as **no piece without an account**: a fused interfacing is sewn by nobody,
  so the invariant that holds is a span *or* a declared non-sewn attachment, and that is what rule `P1`
  checks — recorded in the checklist rather than quietly reinterpreted.
  Commit: `STITCHCAD-G0-0013d`

- ID: `G0-CONTRACT.4b`
  Status: `pending`
  Goal: resolve defect **D32** under the same ruling: give each of the five rows §9 lists as
  `unnamed (D32)` — classic collar, trousers, button/buttonhole, pocket, fly — a **proving gate**, and
  prepare the exact `ROADMAP.md` §11 amendment text that gate's exit criteria need, as a *proposal* in a
  decision record (the roadmap itself is the director's to amend).
  Acceptance: no row in the matrix says `unnamed (D32)`; every reassigned row names a gate whose exit
  criteria either already cover it or are covered by the proposed amendment; the proposal quotes the
  current exit text and the proposed text side by side; the census's M6 rule accepts the new cells and its
  A1 advisory reports `0` unnamed rows; D32 closes with the proposal flagged to the director rather than
  silently applied.
  Verification: `pending`
  Commit: `pending`

- ID: `G0-CONTRACT.14`
  Status: `pending` (drafting unblocked by the director's ruling of `2026-09-30`; **only the naming of
  humans remains blocked** — project owner, procurement owner, domain reviewer)
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
| 8 | `G0-CONTRACT.13c` | `done` | the fixture's waist geometry is arithmetically wrong (D33): drafted as written its finished waist is 46.0 cm, not the declared 74.0 cm, and its own oracle cannot see it. `.5` uses this fixture for its divergence example, so the numbers must be right first |
| 9 | `G0-CONTRACT.5` | `done` | both instantiation paths, the three-part information loss, the `.rul` attributes given StitchCAD semantics, the equivalence contract and the eight extreme-size checks |
| 10 | `G0-CONTRACT.6` | `done` | size-set ownership decided and recorded; the chapter states the object, the label/order separation, breaks, axes and the five designation systems |
| 11 | `G0-CONTRACT.7` | `done` | the standards registry: six designations with role, adoption, status and owner; the deferral ledger; and a census that refuses an unregistered citation anywhere in the book |
| 12 | `G0-CONTRACT.8` | `done` | ADR-0001 settled before any solver code exists, as roadmap §5 requires |
| 13 | `G0-CONTRACT.13d` | `done` | the director ruled on `2026-09-30` that the engineer decides D27: one straight band, cut once and folded at its midpoint, plus a fused interfacing piece — and the agreement is now derived by a tracked instrument rather than by reading |
| 14 | `G0-CONTRACT.4b` | `pending` | **next** — D32: give the five unnamed envelope rows a proving gate and prepare the roadmap amendment as a proposal |
| 15 | `G0-CONTRACT.14` | `pending` | the governance model drafted in full; only the naming of humans stays blocked |
| 16 | `G0-CONTRACT.9` | `pending` | ADR-0003 plus the formula language v1: grammar, units inside expressions, conditionals, name binding, evaluation order, error and dimension rules, worked over the reference skirt. It also names the drafting system that ships as the reference block set |
| 17 | `G0-CONTRACT.10`–`.12`, `.15`–`.17` | `pending` | the remaining G0 chapters, in the order the acceptance table records |

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

- `2026-09-30`, leaf `.6`: a **size label is prose, not a token**, so labels are written in quotes in the
  specification and never in a machine-token style. The glossary census found the violation (three example
  labels in backticks were reported as undeclared tokens) before it became a convention in the chapters
  that follow.
- `2026-09-30`, leaf `.4`: a feature matrix row whose proof no gate has accepted says **`unnamed (D32)`**
  rather than borrowing a gate. Assigning a gate would invent a commitment on that gate's behalf, and a
  silently borrowed gate is how an envelope claim becomes untestable. The census prints those rows on
  every run (advisory `A1`), so the gap is closed by a decision at `.15`, not by being forgotten.
- `2026-09-30`, leaf `.4`: a citation in the matrix's reason column repeats its source per clause
  (`ontology §1, ontology §1.1`, never `ontology §1 and §1.1`), because the census reads citations
  mechanically and a bare `§1.1` after a comma is ambiguous between two documents with overlapping clause
  numbers.

- `2026-09-30`, leaf `.7`: the glossary's derived A–Z index is the reason `glossary.md` grows one line per
  term (`405` lines now, against a `400`-line per-part health and a `700`-line ceiling). The remedy is
  already mechanical and is recorded here so it is not rediscovered as a surprise: when the index passes
  ~500 lines it splits by letter range into two derived halves, because `--emit-index` generates whatever
  the census compares. Owner: the next leaf that adds a batch of terms.

- `2026-09-30`, leaf `.13d`: the fixture's waistband is **one straight band, cut once and folded at its
  midpoint**, plus one interfacing piece cut at the band's finished dimensions and fused — five pieces, not
  the faced reading's six. The choice is sourced, not preferred: the drafting references prescribe the
  two-piece cut for a **contoured** band, and this fixture's band is straight at the natural waist.
  Recorded as `docs/decisions/decision_reference-fixture-waistband-straight-folded.md` with its five
  sources, their URLs and the date read, the one disagreement between them (`wb_width`), and the re-open
  condition.
- `2026-09-30`, leaf `.13d`: **every piece is accounted for — by a span or by a declared non-sewn
  attachment from a closed list**, which holds `fused` alone. "Every piece has a span" is false for a fused
  interfacing, and the false invariant is what let D27's inner band pass for nine commits; stating the
  invariant so a fused piece is *declared* rather than *missing* is what made it checkable. The list is
  closed on purpose: a sewn-in interlining is sewn, so it would need a span.
- `2026-09-30`, leaf `.13d`: an external source that is not a standard is labelled **`read-external`** with
  its URL and the date it was read, and it never upgrades a claim about a standard — that vocabulary stays
  closed in `docs/book/src/spec/standards.md` §1. A source read and *not* used is recorded too, so the next
  session does not re-read it.
- `2026-09-30`, leaf `.13d`: the fixture's arithmetic has a **tracked producer**,
  `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`, with `run_fixture_probes.sh` as its
  ground truth. Until this leaf the numbers that found D33 and D27 were `python3 -c` strings inside task
  leaves — re-runnable by nobody, the leg-3 breach this repository had already committed once as D20.

## Open Questions

- Cut-on-fold vs paired front for the reference skirt: roadmap §11 G0 allows either. Decided
  in `G0-CONTRACT.13` with the reason recorded there (it changes the piece list and the
  notch/fold semantics the fixture must exercise).
- Which named drafting system ships as the v1 reference block set (roadmap ADR-0003: "one
  documented system, decided at G0") — decided in `G0-CONTRACT.9`.
- Whether a `SeamSpan` may name the same piece on both sides (a *self-span*), which the fixture's folded
  waistband ends would need — deliberately **not** decided by `.13d`, logged as D35 and owned by
  `G1-SLICE.3`, because a type invariant settled by whichever way a fixture happens to be written is a
  contract settled by accident.

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

### `G0-CONTRACT.6` — size-set ownership decided, and the commit path stops needing a hand-carried variable

- [x] **ROOT CAUSE (WHY + WHERE)** — roadmap §3.4 leaves the owner of the size set open at G0 ("whether it
  lives in the Design, the Factory Profile, or a third Order object is decided at G0") and ontology §2.3
  forwards to a chapter that did not exist: `git ls-tree --name-only HEAD docs/book/src/spec/` →
  `feature-matrix.md`, `glossary/`, `glossary.md`, `index.md`, `instantiation-paths.md`, `ontology.md`,
  `reference-skirt.md`, `units-and-tolerances.md`, `rc=0`. Two consequences were already live: `.5` consumes
  a base size and breaks it could not name an owner for, and roadmap §7.5's rule that quantities belong to an
  Order object had nothing enforcing it, so a size set could have grown a ratio and coupled every commercial
  change to a design revision. A second, unrelated cause is fixed in the same commit: the ledger probe
  needed `LEDGER_PENDING=<id>` in the environment, so `make probes` — which `COMMIT.md` requires before a
  push — was red during every commit that appended a changelog entry (`make probes` → `Error 1`, measured
  twice this session).
- [x] **ADDRESSED (verified)** — `docs/decisions/decision_size-set-ownership.md` (context, three candidate
  owners argued, six operative rules, consequences, re-open condition, `answers:` line, indexed) and
  `docs/book/src/spec/size-sets.md` (`187` lines / `12 812` B, widest line `216`): ten fields with types and
  requiredness, the label/order separation with the sort that proves it ("XS, S, M, L, XL, 2XL" sorts 2XL
  first), the base-size rules, breaks per adjacent pair, multi-dimensional axes, the five designation
  systems stated as *what the model must express* with EN 13402 and ASTM D5585 content explicitly not
  asserted and owned by `.7`, and MTM as a set of one. Both censuses green:
  `run_glossary_census.sh` → `275 terms / 8 parts / 144 tokens / 0 failure(s)`, `exit=0` (the glossary grew
  by `5` terms and re-derived its index); `run_feature_matrix_census.sh` →
  `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`. The probe seam is now derived: with no environment
  variable set, `run_changelog_ledger_probes.sh` → `probes: 6 pass / 0 fail` and `make probes` →
  `10 suite(s) green`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `10 suite(s) green` (`73` arms, `0 fail`), including the matrix suite at `probes: 11 pass / 0 fail` after
  gaining the M7 link rule and its `DEAD-LINK` arm; `make book` → `exit=0` with `8` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 67 files measured`, `exit=0`, with
  `memory_pointer` back inside its `34`-line health target after trimming and every book part under its
  per-part ceilings; no product code changed, so `make check` is unaffected (`test result: ok` across the
  workspace on the last run that touched Rust).
- [x] **FIX** — wrote the record and the chapter; renamed two size-set fields that would have collided with
  tokens the glossary already owns (`system` → `size_system`, `base` → `base_size`) so one token keeps one
  meaning; gave `provenance` and `chart` their own glossary tokens; added the M7 link rule to the matrix
  census because this slice introduced cross-chapter links into a normative table; replaced the ledger
  probe's hand-carried variable with a derivation from the working tree (the newest live entry is excused
  only when `git diff HEAD` shows this tree adding it, so a mistyped id fails on the next run instead of
  being excused forever).
- [x] **The census caught a convention violation in the chapter before it shipped** — and the fix was the
  rule, not an exemption. Three example size labels were written in backticks, so C1 reported `S`, `M` and
  `L` as machine tokens nothing declared: `glossary census … 3 failure(s)`. A size label is prose a factory
  reads, not an identifier a program reads, so labels are now quoted and §3 of the chapter states the
  distinction normatively. That is the token decision record applying itself to a case its author had not
  thought about, which is the only evidence a convention is real.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; `ontology.md` §2.3's forward
  reference resolves; six matrix rows now cite the chapters that specify them (which is what made M7
  necessary); `docs/decisions/INDEX.md` carries the new record; `TOOLBOX.md`'s matrix row names the link
  rule; the glossary grew `5` terms with its index re-derived; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `docs/TASK_TREE.md` updated in this commit.

### `G0-CONTRACT.7` — the standards registry, and the discipline that keeps a citation honest

- [x] **ROOT CAUSE (WHY + WHERE)** — the G0 exit clause requires the "measurement/POM standards identified
  (ISO 8559, ASTM D5219, EN 13402, ASTM D5585)" and `git ls-tree --name-only HEAD docs/book/src/spec/` →
  eight entries, none of them a standards chapter, `rc=0`. Three chapters had already routed their
  standards claims to it: `ontology.md` §8 ("the standards' texts have not been read in this repository"),
  `reference-skirt.md` §11 ("the measurement-standards chapter owns reconciling them") and `size-sets.md`
  §8 — three deferrals to a document that did not exist, which is the shape a claim takes when it is
  waiting to be asserted by nobody. Meanwhile six designations were already in use across the book with no
  registry, no status and no owner: `grep -roE 'ISO [0-9]+|ASTM D[0-9]+|EN [0-9]+|AAMA' docs/book/src |
  wc -l` → `33` occurrences in `9` files.
- [x] **ADDRESSED (verified)** — `docs/book/src/spec/standards.md` (`204` lines / `13 849` B, widest line
  `187`) registers all six designations with role, what is adopted, what is deliberately not, a status from
  the closed three-word vocabulary and a named owner; states the two rules that follow (no clause or table
  of a standard appears without `read-in-repo` status, and a standard is data with provenance, never
  authority); gives the four requirements the model places on any standard; dispositions all five
  deferrals in a ledger; and names the verification plan's real dependency (`.14` names the reviewer,
  procurement obtains the texts, and until then no claim can become `read-in-repo`). The registry claim is
  derived: `bash docs/tasks/artifacts/standards/run_standards_census.sh` →
  `standards census: 6 registered / 6 designations used / 0 failure(s)`, `exit=0`, with a per-designation
  list of every file that uses it; `bash docs/tasks/artifacts/standards/run_standards_probes.sh` →
  `probes: 6 pass / 0 fail`, including an arm that smuggles "per ISO 4915" into a chapter and requires the
  census to name it.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `11 suite(s) green`; `make book` → `exit=0` with `9` spec pages rendered;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 69 files measured`, `exit=0`, the
  chapter at `153` lines inside its `400`-line per-part health; the other two censuses still green —
  `run_glossary_census.sh` → `275 terms / 8 parts / 144 tokens / 0 failure(s)` and
  `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`; `make check` unaffected (no
  Rust touched).
- [x] **FIX** — wrote the chapter and the census; normalized the registry's first cells to the designations'
  base forms (`ASTM D6673`, not `ASTM D6673-10`) because the census matches those keys against the whole
  book; **the containment ceiling refused the first draft** — the six-column registry table had rows of
  `410` B against a `320` B maxline ceiling and the deferral ledger `398` B, so the registry became four
  columns plus one bounded subsection per standard, and the ledger became bounded prose entries, exactly
  the remedy `G0-CONTRACT.2` recorded when the same ceiling caught its tolerance table; taught S3 to read a status token that carries its source in parentheses, which the first cut
  rejected as an invented status; fixed a `grep -c … || printf 0` that printed `0⏎0` — the same trap
  DEV_NOTES recorded on 2026-09-04, caught here by its own output.
- [x] **The glossary census caught two prose spans wearing token formatting before the chapter shipped** —
  `` `blocked` `` (a task-tree status, not a product identifier) and `` `AAMA` `` inside a sentence
  describing the census's match shapes. C1 reported both as machine tokens nothing declared
  (`2 failure(s)`), and the fix was to de-tokenize the prose rather than to exempt it: the convention says a
  backticked span is an identifier a program reads, and these were not. That is the third time the token
  census has corrected a chapter at authoring time rather than in review, which is the evidence that the
  convention is load-bearing rather than decorative.
- [x] **LOCKSTEP** — `SUMMARY.md` and `spec/index.md` link the chapter; `docs/TASK_TREE.md`'s frontier cell
  and execution-order line are corrected (the immediate half of D34, whose durable half is `PLANNING.5`);
  `TOOLBOX.md` gains the standards census and its probe suite; `knowledge-map/subsystems.md` lists the
  chapter; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and this tree updated in this commit.

### `G0-CONTRACT.13d` — one waistband, and the instrument that keeps it one

- [x] **REPRODUCE / ISSUE** — defect D27: §4 published the cut width of ONE band folded lengthwise
  (`2 × wb_width + sa_waist + sa_wb_bottom` = 10.0 cm) while §6 listed a faced two-piece band
  (`waistband_outer`, `waistband_inner`, `waistband_interfacing`, each piece 6.0 cm) and §8 sewed only the
  outer one, so §12's count of 6 was the rejected reading's. Reproduced by the instrument this leaf lands,
  run over the chapter as committed: `git show HEAD:docs/book/src/spec/reference-skirt.md >
  target/scratch/before.md && FIXTURE_CHAPTER=$PWD/target/scratch/before.md bash
  docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` →
  `fixture derivation: 16 derived rows / 2 closure checks / 6 pieces / 7 mismatch(es)`, `exit=1`, with the
  decisive line `B1: §6 describes a faced two-piece band (per-piece cut width 6.0 cm) but
  `waistband_cut_width` publishes 10.0 cm — §4 and §6 are different garments, which is defect D27` and six
  `P1` refusals naming every piece §8 accounted for (none). It lived that way for nine commits:
  `git log --oneline 45e0a46..HEAD | wc -l` → `9`.
- [x] **ROOT CAUSE (WHY + WHERE)** — two causes, and only the second was visible before this leaf.
  (1) *The chapter*: a construction decision was never made. §1 said "cut twice plus interfacing", §4's
  formula described a folded band, §6 listed a faced pair — three cells of three different tables, each
  internally correct, and no arithmetic in any one of them could contradict another. The census that
  existed for this chapter read tokens, not tables: `git ls-files docs/tasks/artifacts | grep -c fixture`
  → `0`, `rc=1` (no tracked instrument over the fixture at `HEAD`), and the arithmetic that found D33 and
  then D27 was typed into leaf prose — `grep -c 'python3 -c' docs/tasks/PLANNING.md` → `2` — which is
  re-runnable by nobody and is the leg-3 breach this repository already committed once as D20.
  (2) *The invariant*: the chapter had no rule a piece must satisfy to be in the piece list, so an inner
  band nobody sewed was a prose gap rather than a refusal.
- [x] **ADDRESSED (verified)** — the fixture is one garment and says so with numbers.
  `bash docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` →
  `fixture derivation: 20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`, `exit=0`, against
  `7 mismatch(es)` over the same chapter at `HEAD` (above): §4 gained `waistband_fold_position` (5.0 cm),
  `waistband_finished_length` (77.0 cm), `wb_interfacing_width` (4.0 cm), `wb_interfacing_length` (77.0 cm)
  and the band's two closure checks — `waistband_length_closure` `74.0 = 74.0` and
  `waistband_width_closure` `8.0 = 8.0`, both printed as `CLOSES`; §6 lists five pieces; §8's attachment
  account names all five (`accounted: 5`); §12's count agrees (`published count: 5`); and `B1` reports
  `a single band folded lengthwise · cut width 10.0 cm · agrees with §6's 1 band piece(s)`. The instrument
  is a tracked producer with ground truth: `TMPDIR=$PWD/target/scratch bash
  docs/tasks/artifacts/reference_fixture/run_fixture_probes.sh` → `probes: 9 pass / 0 fail`, `exit=0`.
  The decision is sourced, not preferred — five references read on this machine on `2026-09-30` (reachability
  re-measured first: `curl -sS -m 12 -o /dev/null -w '%{http_code}' https://en.wikipedia.org/wiki/Waistband`
  → `200`), the decisive fact being anicka.design's drafting sequence, which gives the **straight** band one
  rectangle with a fold line and reserves "cut two pieces — one for the outer waistband, and one for the
  inner waistband" to the **curved** band; recorded with every URL, the date read, the `read-external` label,
  the one disagreement between sources (`wb_width`: 2–5 cm versus a 3 cm maximum for a straight band, kept
  `assumed` rather than silently adjusted) and the re-open condition in
  `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`. The three neighbouring censuses
  are still green, which is the check that this did not contradict another chapter:
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `276 terms / 8 parts / 145 tokens /
  0 failure(s)`, `exit=0`; `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` →
  `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0` (its `interfacing` row still says the fixture carries
  an interfacing piece, and still does); `bash docs/tasks/artifacts/standards/run_standards_census.sh` →
  `6 registered / 6 designations used / 0 failure(s)`, `exit=0`. `make book` → `INFO HTML book written to
  …/docs/book/book`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0` (eleven before this leaf, the twelfth being the new one);
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 74 files measured`,
  `exit=0`; `bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` → `probes: 7 pass / 0 fail`,
  `exit=0`. No Rust changed (`git diff --cached --name-only | grep -c '\.rs$'` → `0`, `rc=1`), so
  `make check` is unaffected and the crate tests were last green at `.18`. The probe suite was shown to
  *discriminate* rather than merely to pass, by neutering one rule at a time in a scratch copy of the tool:
  with `B1` removed, `BAND-PAIR` goes red → `probes: 8 pass / 1 fail`; with `P2`'s comparison removed,
  `COUNT` and `BAND-PAIR` go red → `probes: 7 pass / 2 fail`. Containment: the chapter is `379` lines /
  `26 599` B / widest `161` B against a `book_collection` per-part health of `400` / `24 576` / `200` — one
  axis over health and inside every ceiling (`700` / `40 960` / `320`) — and the decision record is `145`
  lines / `11 303` B / widest `353` B against a `decisions_collection` health of `120` / `8 192` / `320` and
  a ceiling of `200` / `16 384` / `600`. The glossary's derived index grew one line with the new term and is
  now `406` lines against its `400`-line health, which is the growth `G0-CONTRACT.7`'s decision anticipated
  and whose remedy (`--emit-index` into two letter halves at ~500) is already recorded. Nothing was trimmed
  to hit a health target, because the bytes are the sourcing the ruling asked for; every overrun is recorded
  here rather than paid for in vaguer prose, which is the remedy `G0-CONTRACT.1` and `.7` both recorded.
- [x] **FIX** — chose the construction and made the chapter one garment: §1 ("cut once and folded
  lengthwise"), §4 (four new rows, the disputed marker removed), §4.1 (two closure checks → four), §5
  (step 8 rewritten, step 9 added for the interfacing, the mirror step renumbered — nothing referenced
  step 9, checked: `grep -rn 'step [0-9]' docs/book/src/`), §6 (five pieces, the provisional paragraph
  replaced), §7 (`sa_cb` and `sa_waist` name the band edges they also serve, the band's free edge
  described, an allowance-free row for the interfacing), §8 (`waist` span names `waistband`, the attachment
  account added, the ends recorded as an edge finish), §10 (two new exercised properties), §11 (the D27
  bullet is now the resolution with its sources and what stays `assumed`), §12 (count 5, three new
  obligations). Built the instrument and its probe suite; added the glossary entry that owns `fused`.
  Also fixed in this commit, because it refused this slice's own changelog entry and the probe was what
  was wrong (defect **D39**): `run_changelog_ledger_probes.sh` read work-unit ids with `[0-9]+[a-c]?`, so
  `STITCHCAD-G0-0013d` parsed as `STITCHCAD-G0-0013` and collided with the sealed entry of that name. The
  class is `[a-z]?` in all six places and a new GREEN arm `SUFFIX` pins it — measured sensitive by
  reverting the class in a scratch copy → `probes: 5 pass / 2 fail`.
- [x] **The acceptance clause "no piece without a span" is met as *no piece without an account*, and that
  is recorded rather than quietly reinterpreted.** A fused interfacing is sewn by nobody, so the literal
  clause is unsatisfiable for the construction the sources prescribe; the invariant that holds is a span
  *or* a declared non-sewn attachment from a closed list that today contains exactly `fused` (a sewn-in
  interlining is sewn, so it would need a span). Rule `P1` checks that, and its `NO-ACCOUNT` arm removes
  the interfacing's row and requires the refusal.
- [x] **The token census corrected the chapter a fourth time at authoring time, and the fix was again the
  convention rather than an exemption** — `run_glossary_census.sh` reported `4 failure(s)` on the first
  draft: `` `Fold` `` (a column name of this chapter's own table, i.e. prose), `` `fused` `` (a machine
  enum value a program reads — so it became a glossary term with one owner), and the two names of the
  rejected reading, `` `waistband_outer` `` and `` `waistband_inner` `` (identifiers that no longer exist,
  so the history sentence names them in prose and the layer-C record keeps the tokens for anyone tracing
  the defect). After the entry and a re-derived index: `276 terms / 8 parts / 145 tokens / 0 failure(s)`,
  `exit=0`.
- [x] **LOCKSTEP** — D27 closed, D35 and D38 logged, D36, D37 and D39 logged and fixed, D34's recurrence
  recorded in `PLANNING.md`; `docs/TASK_TREE.md`'s three stale frontier cells and its execution-order line
  corrected; `TOOLBOX.md` gains both instruments; `docs/decisions/INDEX.md` carries the new record and the
  derived Knowledge Map was regenerated (`make gate` refused until it was); `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and `DEV_NOTES.md` updated in this commit. Lesson promotion: **promoted** — the new
  record carries an `answers:` line, which is what `LESSON-PROMOTION` asks for.

Gate-level closure is recorded by `G0-CONTRACT.15`; each leaf carries its own evidence in the
Verification Log, and `.18` (the code leaf) additionally fills a `### G0-CONTRACT.18` checklist
subsection with real tool output in the same commit as the change. This tree file carries no
unticked placeholder boxes: the spine's acceptance gate judges the FIRST matching box in a file, so
a placeholder shadows real evidence and falsely rejects honest work (defect D15, measured by the
`SPINE.7` probe; local mitigation `SPINE.8`).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-30` | `G0-CONTRACT.13d` | fixture derivation at `HEAD` vs the working tree; its probes + two neutered-rule meta-checks; the three neighbouring censuses; `make book`/`gate`/`probes`; containment | before `16 rows / 2 checks / 6 pieces / 7 mismatch(es)` `exit=1`, after `20 / 4 / 5 / 0` `exit=0`; `probes: 9 pass / 0 fail`, and neutering `B1` or `P2` reddens the arms that need them; `276 terms`, `105 rows`, `6 registered`, all `0 failure(s)` |
| `2026-09-29` | tree seeded | `scripts/check_doctrines.sh` | `=== all doctrines green ===`, `rc=0` |
| `2026-09-29` | `G0-CONTRACT.2` | `wc -lc` and per-line max on the chapter; `make book`; arithmetic re-derivation; `check_live_doc_size.sh`; `make gate`; `make check` | `291` lines / `17180` bytes / maxline `114`; `exit=0`, chapter rendered; 5 arithmetic claims confirmed; `OK — 17 surfaces, 15 routes, 47 files`; `=== all doctrines green ===`; `test result: ok. 1 passed` |
| `2026-09-29` | `G0-CONTRACT.18` | `cargo test --all`; `make wasm`; `cargo fmt --check`; `cargo clippy -D warnings`; `make gate`; `make probes` | 30 tests / 0 failed; wasm cross-build green; fmt clean; clippy clean; `=== all doctrines green ===`; `7 suite(s) green` |
| `2026-09-29` | `G0-CONTRACT.3` | `wc -lc` + per-line max on the chapter; `make book`; `check_live_doc_size.sh`; `make gate`; `make check`; `make probes` | `295` lines / `16187` B / maxline `198`; `exit=0` and rendered; `OK — 17 surfaces, 15 routes, 50 files`; all gates green |
| `2026-09-29` | `G0-CONTRACT.13` | arithmetic re-derivation incl. the allocation balance; `wc -lc` + per-line max; `make book`; `check_live_doc_size.sh`; `make gate` | balance `28.0 = 28.0`; `245`/`14538`/maxline `118`; `exit=0`; `OK — 17 surfaces, 15 routes, 51 files`; all green |
| `2026-09-29` | `G0-CONTRACT.18` (CI verdict, observed after the exceptional push) | `git push origin main`; GitHub Actions runs API for `head_sha=119946b` | `051a075..119946b  main -> main`, ahead `0`; **`rust` completed `success`** (first execution of the new wasm32 smoketest step) and **`doctrines` completed `success`**; `runs: 2`, both concluded |
| `2026-09-29` | coverage gaps closed | roadmap clause census (§4.3, §4.4, §7.3, §7.6) | 3 clauses were unowned → `.16`, `.17`, `.18` |
| `2026-09-29` | `G0-CONTRACT.13b` | token census over the fixture at `HEAD` vs the working tree; `python3` re-derivation of both waistband readings; `make book`; `make gate` | undeclared tokens `19` → `6`, the six being glossary vocabulary; folded band `10.0` cm vs faced `6.0` cm per piece, so §4 and §6 are different garments (D27); no §4 result changed; `exit=0` both |
| `2026-09-30` | `G0-CONTRACT.8` | `grep -n license Cargo.toml`; `grep -rn 'slvs\|GPL' crates/`; `grep -n -i licen docs/tasks/G1-SLICE.md`; `make gate` | `license = "MIT OR Apache-2.0"`, `rc=0`; no `slvs` or GPL reference in any crate, `rc=1`; the licence census is already `G1-SLICE.15`; `=== all doctrines green ===` |
| `2026-09-30` | `G0-CONTRACT.7` | standards census and its probes; glossary and matrix censuses; `make book`; `make gate`; `make probes`; containment | `6 registered / 6 designations used / 0 failure(s)`; `probes: 6 pass / 0 fail`; `275 terms / 0 failure(s)` after two prose spans were de-tokenized; `105 rows / 0 failure(s)`; `exit=0`, 9 spec pages; chapter `204` lines / `13 849` B, widest `187` |
| `2026-09-30` | `G0-CONTRACT.6` | glossary, feature-matrix (now with M7 link resolution) and matrix probe suites; `make book`; `make gate`; containment | `275 terms / 8 parts / 0 failure(s)`; `105 rows / 29 diagnostics / 0 failure(s)`; `probes: 11 pass / 0 fail`; `exit=0` with 8 spec pages; chapter `187` lines / `12 812` B |
| `2026-09-30` | `G0-CONTRACT.5` | `python3` re-derivation of the graded fixture (15 quantities, both routes); glossary and feature-matrix censuses; `make book`; `make gate`; containment | side-seam length `43.104524` cm by both paths, difference `0.00e+00`; graded `waist_closure` `19.500 = 19.500`; `270 terms / 0 failure(s)`; `105 rows / 0 failure(s)`; `exit=0`, 7 spec pages; chapter `254` lines / `19 496` B |
| `2026-09-30` | `G0-CONTRACT.13c` | `python3` re-derivation of all 18 §4 rows and of both waist readings; glossary census; `make book`; `make gate` | `0` mismatches; finished waist `74.0` cm (was `46.0` as written); `waist_closure` `18.5 = 18.5`; dart centre `11.25`; `265 terms / 0 failure(s)`; `exit=0` both |
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
| `G0-CONTRACT.13c` | `STITCHCAD-G0-0013c (leaf G0-CONTRACT.13c): the fixture's waist, corrected` | D33 fixed; §4 gains `waist_closure`; the general rule is a layer-C record |
| `G0-CONTRACT.5` | `STITCHCAD-G0-0005 (leaf G0-CONTRACT.5): both instantiation paths` | the loss stated in three parts; the fixture grades exactly, and the chapter says why that proves nothing general |
| `G0-CONTRACT.6` | `STITCHCAD-G0-0006 (leaf G0-CONTRACT.6): size-set ownership decided` | a layer-C record plus the chapter; quantities stay out of the size set |
| `G0-CONTRACT.7` | `STITCHCAD-G0-0007 (leaf G0-CONTRACT.7): the standards registry` | six designations, all `cited-from-roadmap`; no quoted clause anywhere in the book |
| `G0-CONTRACT.8` | `STITCHCAD-G0-0008 (leaf G0-CONTRACT.8): ADR-0001, license × solver` | permissive dual-licensed core, `slvs` rejected, BSL-1.1 kept a separate category, re-open condition in three parts |
| `G0-CONTRACT.13d` | `STITCHCAD-G0-0013d (leaf G0-CONTRACT.13d): one waistband, and the instrument that keeps it one` | D27 closed; five pieces; the band's two closure checks; `run_fixture_derivation.sh` + its probe suite |
| `G0-CONTRACT.4b`, `.9`–`.12`, `.14`–`.17` | `pending` | — |

## Changelog

- `2026-09-30`: `.13d` landed — D27 is closed: the fixture's waistband is ONE straight band, cut once and
  folded at its midpoint, plus one interfacing piece cut at the band's finished dimensions and fused inside
  the seam lines. Five pieces, not six; `waistband_outer` and `waistband_inner` are gone; §4 gained four
  rows (`waistband_fold_position`, `waistband_finished_length`, `wb_interfacing_width`,
  `wb_interfacing_length`) and §4.1 gained the band's two closure checks; §8 accounts for every piece by a
  span or by the closed list's one non-sewn method, `fused`. The choice is sourced rather than preferred —
  five references read on this machine, URLs and dates in
  `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`, the decisive fact being that
  the two-piece cut belongs to a **contoured** band while this fixture's is straight at the natural waist.
  The agreement between §4, §6, §8 and §12 is now derived by a tracked producer
  (`run_fixture_derivation.sh`: `7` mismatches over the chapter at `HEAD`, `0` after) with a probe suite
  shown sensitive by neutering one rule at a time. D35 logged and deliberately not decided (may a span name
  one piece twice); D36 logged and fixed; D34's recurrence recorded.
- `2026-09-30`, after `.8`: the director ruled that the engineer decides and acts on the four findings
  surfaced this session — D27, D32, `.14`'s drafting and the containment derivation — at signoff grade, with
  external research permitted. Recorded as
  `docs/decisions/decision_director-ruling-2026-09-30-four-findings.md`, which also states what stays
  reserved (naming humans; amending `ROADMAP.md`). Leaves `.13d` and `.4b` created to own D27 and D32;
  `.14`'s blocker narrowed to the naming alone. The frontier takes those four before `.9`.
- `2026-09-30`: `.8` landed — ADR-0001 is one record, not two: the core is dual `MIT OR Apache-2.0`, so
  `slvs` (GPLv3) is rejected and `sc-sketch` gets a custom or least-squares kernel. The exchange rate is
  stated (an optional annotation layer is not worth the licence of the product), BSL-1.1 is kept out of the
  permissive category, no CLA is needed to contribute, an iterative kernel's determinism obligation is named
  before the kernel exists, and the single re-open condition is written in three parts now so the G1 spike
  cannot be argued into one later.
- `2026-09-30`: `.7` landed — the standards registry exists: six designations (ISO 8559, ASTM D5219,
  EN 13402, ASTM D5585, ASTM D6673, AAMA) each with its role, what is adopted, what is not, a status from
  the closed vocabulary and a named owner. All six are `cited-from-roadmap`, so the book quotes no clause
  of any standard — which is the rule, not a gap. The five deferrals from the ontology, the fixture, the
  size-sets chapter, the glossary and the matrix are dispositioned in a ledger, the verification plan names
  its procurement dependency, and `run_standards_census.sh` refuses any future chapter that cites a
  standard the registry does not carry.
- `2026-09-30`: `.6` landed — roadmap §3.4's open question is closed: a `SizeSet` is its own object,
  referenced by the `Design` at a revision, overridable per Factory Profile through a recorded
  transformation that produces a *resolved* set, and carrying no quantities (those are order data, §7.5).
  The chapter states the ten fields, why a label is a name and an order is authored rather than sorted, the
  base-size rules, breaks, multi-dimensional axes, and what the model must express for each of the five
  designation systems — with EN 13402 and ASTM D5585 content explicitly *not* asserted and handed to `.7`.
- `2026-09-30`: `.5` landed — both instantiation paths are normative: inputs, process, outputs, authority
  and oracle per path; grade points as `PointRef`s; allowances re-derived rather than graded (with the
  corner consequence stated); `stack point` / `fixed perimeter` / `smoothing` given StitchCAD semantics and
  their `.rul` encoding marked for confirmation at G3; the information loss in three parts; an equivalence
  contract that reports and never reconciles; eight extreme-size checks run after target-system
  reconstruction. The graded reference skirt agrees between paths to `0.00e+00`, and §4 records that this is
  the fixture's affinity, not a general result — the tolerance is exercised at G3.
- `2026-09-30`: `.13c` landed — the fixture's waist geometry is corrected (D33): the side point moves from
  `quarter_waist − ss_suppress` (15.5 cm) to `quarter_hip − ss_suppress` (22.5 cm), so the finished waist is
  the declared 74.0 cm instead of 46.0 cm; both dart centres move 7.75 → 11.25 cm; §4 gains `waist_closure`,
  the constructed oracle the allocation balance could not be. All 18 derived rows re-derive with 0
  mismatches, and the rule generalises as
  `docs/decisions/decision_fixture-oracles-derive-the-finished-dimension.md`.
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
