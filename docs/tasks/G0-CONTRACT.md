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
  Verification: recorded below and in the acceptance checklist — the census reports `105 rows /
  29 diagnostics / 0 failure(s)` with `A1 … D32 rows: 0` and the new `A3` advisory listing `4` proposed
  cells; its probe suite is at `12 pass / 0 fail`; the ledger probe, generalized to every sealed segment,
  is at `8 pass / 0 fail`.
  Commit: `STITCHCAD-G0-0004b`

- ID: `G0-CONTRACT.4c`
  Status: `done`
  Goal: rule on and **apply** the D32 amendment `.4b` prepared. The director's first ruling of `2026-09-30`
  reserved amending `ROADMAP.md` to himself, so `.4b` landed it as a *proposal* with four matrix cells marked
  `(proposed)`; his second instruction delegated the finding outright ("make the necessary calls and every
  needed action"), which turns the proposal into a decision this repository can execute. Applying it means the
  roadmap's own machinery — a revision marker, an Appendix A disposition entry, a re-based containment
  baseline — and the trees that must own what the new criterion requires.
  Acceptance: `ROADMAP.md` §11 G3 carries the envelope-coverage criterion and the file is at a new revision
  with its disposition logged; no matrix cell says `(proposed)` and the census's A3 reports `0`; every garment
  the criterion names is owned by a `G3-GRADING` leaf, and that tree's exit review fails without it; the
  containment baseline is re-based under a recorded authority rather than hand-widened; every census and gate
  stays green.
  Verification: recorded below and in the acceptance checklist — `ROADMAP.md` at v0.3 (`947` lines /
  `52 818` B, baseline re-based `at=v0.3`), the matrix census at `105 rows / 29 diagnostics / 0 failure(s)`
  with `D32 rows: 0` and `proposed cells: 0`, and the coverage census green with its new `7`-arm suite.
  Commit: `STITCHCAD-G0-0004c`

- ID: `G0-CONTRACT.14b`
  Status: `done`
  Goal: act on the second finding — the three seats governance §8 lists. The director's first ruling reserved
  **naming humans** to himself and his second delegated the finding, so what is decidable is not who fills a
  seat but what happens while it is empty: which seats are held *acting*, what an acting holder may not do,
  and what the project needs from each name.
  Acceptance: governance §8 states the acting authority and its hard limits, and one seat is openly vacant
  rather than quietly acting; every clause a vacancy governs is `unknown`, not `assumed`; the ask per seat is
  written so naming one is a single act; the chapters that pointed at "`.14` names the reviewer" point at the
  seat's real state; a decision record carries the rule.
  Verification: recorded below and in the acceptance checklist — the chapter's §8 gained two subsections, the
  domain seat is vacant with four named prohibitions, and the fixture and standards chapters cite it instead of
  a leaf that cannot name anyone.
  Commit: `STITCHCAD-G0-0014b`

- ID: `G0-CONTRACT.14`
  Status: `done` for everything this repository can do — the model is drafted, reviewable and adopted; the
  **naming of the three humans stays blocked** on the director and is carried by §8 of the chapter and by
  `.15`'s exit review
  Goal: draft the governance model (§12) — sewist-vs-programmer review paths, domain review of
  profile changes that alter exported bytes, golden-file approval ownership, funding/procurement
  owners — and record the roles that need a named person (evaluation seats, physical plotter,
  project owner), with the partner-run manual test as documented fallback.
  Acceptance: the model is written and reviewable; each role states what authority it needs;
  the named-person gaps are flagged to the director in one place, not discovered later.
  Verification: recorded below and in the acceptance checklist — `docs/book/src/governance.md` is
  `198` lines / `15 650` B with ten sections, every role carrying its authority and its agent-eligibility,
  all four gaps in one table; the book builds and every census stays green.
  Commit: `STITCHCAD-G0-0014`

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
| 14 | `G0-CONTRACT.4b` | `done` | D32 resolved: collar, trousers, buttons and pockets get G3 as their proving gate and the fly gets G7, with the roadmap amendment quoted as a **proposal** the director rules on at `.15` |
| 15 | `G0-CONTRACT.14` | `done` | the governance model is drafted in full — two review paths, roles defined by the decision each may make, goldens signed twice, the procurement fallbacks with their cost in evidence — and the three empty seats are in one table for the director |
| 15b | `G0-CONTRACT.4c` | `done` | the director delegated the finding outright, so `.4b`'s proposal is ruled approved and applied: roadmap **v0.3** carries the envelope-coverage criterion, the four cells are committed gates, and `G3-GRADING.5`/`.15` own the garments |
| 16 | `G0-CONTRACT.9` | `pending` | **next in this tree** — but the ruling of `2026-09-30` puts `SPINE.4.4` (the maxline health of table-shaped book parts) between `.14` and this leaf, so the repository's frontier is `SPINE.4.4` and then `.9`: ADR-0003 plus the formula language v1 — grammar, units inside expressions, conditionals, name binding, evaluation order, error and dimension rules, worked over the reference skirt, and the drafting system that ships as the reference block set |
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

- `2026-09-30`, leaf `.4b`: **permission is not a criterion.** G3's note that an intermediate garment "may
  be inserted without shame" schedules nothing, so the proposed amendment adds one exit criterion over
  roadmap §3.2's whole garment list — closing the class rather than the four instances — and names the
  failure mode: a garment the envelope names and no exit criterion proves is a gate failure, not a scope
  note. Recorded in `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
- `2026-09-30`, leaf `.4b`: a gate this chapter *proposes* is written `(proposed)` in the cell, and the
  census prints every such cell on every run (advisory `A3`). A provisional commitment that reads like a
  settled one is the D32 gap wearing better clothes, so the proposal is as visible as the gap it replaces —
  and a probe arm removes the markers and requires the printed count to fall, because an advisory that reads
  nothing prints the same number either way.
- `2026-09-30`, leaf `.4b`: a **deferred** row names the gate that declares the limitation (G7, per rule 3
  of the matrix's §1), not a gate that proves the feature. That is why the fly needs no amendment while the
  four supported rows do, and why the `lining` row was already right.
- `2026-09-30`, leaf `.4b`: the dev-notes rollover is performed in the commit whose append crossed the
  window's health target, and the archive verifier's `DESCRIPTOR` rule is generalized to every
  `docs/history/*.md` segment in the same commit — a sealed segment whose digest nothing watches is a
  "trust me" with a hash beside it. The Coverage and pointer legs stay changelog-only, which is logged as
  D40 and owned by `SPINE.19` rather than left implicit.

- `2026-09-30`, leaf `.4b`: **a tree past 1000 lines splits its completed-leaf evidence into a sibling
  file**, which is the containment registry's own remedy and not an invention of this slice — and the leaf
  being landed keeps its checklist in the tree file, because `scripts/check_task_acceptance.sh` judges every
  staged `docs/tasks/*.md` and refuses one with no ticked boxes. Measured, not assumed: emptying the tree
  file of checklists would have turned an honest slice into a `TASK-ACCEPTANCE` refusal. Consequence: the
  tree file's first matching box is now always the current leaf's, which is D15's facet 1 closed by
  structure rather than by care. `G0-CONTRACT-evidence.md` is the sibling; `SPINE.md` owes the same split
  (defect **D42**, owned by `SPINE.4.4`).

- `2026-09-30`, leaf `.14`: **a role is the decision it may make, not the person holding it.** Every role in
  the governance chapter carries the authority it needs and whether an agent may hold it, which is what makes
  an empty seat assignable instead of mysterious: the work is specified and only the name is missing. The
  three empty seats are in ONE table (§8) rather than discovered at the gate that needs each of them.
- `2026-09-30`, leaf `.14`: **a contested default becomes a profile parameter, not a verdict.** Where a sewist
  and a programmer genuinely disagree about a default, the model already carries both readings (roadmap §8.3),
  so governance records the dissent instead of picking a winner — and §3 classifies the question first,
  because most such conflicts are two correct answers to different questions.
- `2026-09-30`, leaf `.14`: **a procurement fallback states what it costs in evidence quality.** Roadmap §14
  already makes the partner-run manual test the documented fallback for an eval-seat slip; §7 of the chapter
  adds the cost of each fallback (a partner run is layer-4 evidence: slower, fewer targets), because a
  fallback without a stated cost is how a procurement slip silently downgrades the release claim.

- `2026-09-30`, leaf `.4c`: **a proposal is a state, not a resting place.** `.4b` marked four cells
  `(proposed)` because amending the roadmap was reserved; the moment the reservation was lifted, the honest
  action was to apply it through the roadmap's own revision machinery — version marker, Appendix A disposition
  entry, containment baseline re-based in the same commit — and not to leave a ratified decision wearing a
  provisional label. The `(proposed)` mechanism and the A3 advisory stay in place for the next one.
- `2026-09-30`, leaf `.4c`: **an exit criterion must arrive with owners.** A roadmap clause no leaf owns is
  the D32 defect one level up, so the same commit that added G3's envelope-coverage criterion made
  `G3-GRADING.5` required (trousers with a pocket and a derived buttonhole), created `G3-GRADING.15` (the
  classic collar) and made `.14`'s exit review fail if a §3.2 garment has no leaf's evidence behind it.

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

- `G0-CONTRACT.14`'s model is drafted and `.14b` settled what an empty seat means: the project owner's and
  procurement owner's seats are held **acting** by the director (governance §8.1), and the sewing/factory
  domain expert's seat is **vacant** — no acting holder may confirm the fixture's `assumed` constants, sign a
  golden's semantic half, rule a safety term or approve a byte-changing profile, so the first G2 golden stays
  gated on a real name. `.15` records the G0 governance clause accordingly, and never as met on a borrowed
  signature.
- `G0-CONTRACT.15` no longer carries a proposal to the director: the roadmap amendment `.4b` prepared was
  delegated, ruled approved and applied as **v0.3** by `.4c`. What `.15` owes the director is the naming of the
  three seats governance §8 lists — with §8.2's ask per seat already written — and the ordinary
  clause-by-clause exit review, in which the governance clause is recorded as `.14b` states it.

## Acceptance Checklist

Completed leaves' checklists live in [`G0-CONTRACT-evidence.md`](G0-CONTRACT-evidence.md), split out under
the containment registry's remedy for a tree past 1000 lines. The leaf being landed keeps its checklist
here, because `scripts/check_task_acceptance.sh` judges every staged `docs/tasks/*.md` file and refuses one
with no ticked boxes; the next slice moves it across. Neither file carries an unticked placeholder box
(defect D15).

### `G0-CONTRACT.4c` — the proposal is ruled on and applied, and the criterion arrives with owners

- [x] **REPRODUCE / ISSUE** — two things were true and neither could stand. (1) The envelope was still
  unproved in the roadmap: `sed -n '/^### G3 /,/^- Domain-complexity/p' ROADMAP.md | grep -ci
  'collar\|trousers\|button\|pocket'` → `1` before this slice, and that one hit was the *note* that permitted
  an intermediate, not a criterion; the matrix said so mechanically —
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `A3 advisory … proposed cells: 4`.
  (2) Applying the amendment was impossible without cheating: `wc -lc ROADMAP.md` → `919 50821`, exactly the
  `roadmap` row's transition-debt baseline (`lines=919;bytes=50821`), and `check_live_doc_size.sh` refuses a
  widened baseline — so the only way to grow the roadmap was to edit the number, which is the silent widening
  the rule exists to prevent.
- [x] **ROOT CAUSE (WHY + WHERE)** — the first cause was authority, not analysis: the director's ruling of
  `2026-09-30` reserved amending `ROADMAP.md`, and his second instruction delegated the finding outright, so
  the proposal became a decision this repository could execute. The second cause was a missing mechanism: a
  debt baseline is a *stored copy* of a file's size at a moment, and nothing tied it to the file's revision
  identity, so a legitimate revision and a silent widening looked identical to the checker. Measured, because
  the block is the evidence: `bash scripts/check_live_doc_size.sh` over the amended roadmap →
  `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (947 > baseline 919) — a baseline never grows`,
  `exit=1`, against `grep -o 'lines=[0-9]*;bytes=[0-9]*' .doctrine/live_document_size/surfaces.tsv` →
  `lines=919;bytes=50821`, `rc=0`. That is the containment adoption note's deferred trigger 3 — "a stored copy
  of a mechanically owned value needs an executed freshness oracle" — fired by the first roadmap amendment,
  and `SPINE.4.5` discharges it.
- [x] **ADDRESSED (verified)** — roadmap **v0.3** carries the criterion: `grep -n 'envelope coverage'
  ROADMAP.md` → line `712`, `- **Exit (envelope coverage):** every garment §3.2 names drafts, grades and
  exports at this gate or an earlier one …`, and the same census over the section now returns `3` matching
  lines instead of `1`. The revision is marked where the roadmap's own policy requires —
  `grep -n 'v0.3' ROADMAP.md` → the title (line 1), the status block (line 11, naming the source), the
  Appendix A disposition entry (line 926) and the end line (line 945) — and the file measures
  `wc -lc ROADMAP.md` → `947 52818` (+28 lines / +1 997 B), re-based in the registry as
  `lines=947;bytes=52818;at=v0.3`. The matrix is committed rather than provisional:
  `grep -c 'proposed §11 amendment' docs/book/src/spec/feature-matrix.md` → `0`, and
  `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `D32 rows: 0`
  and `proposed cells: 0`; `run_feature_matrix_probes.sh` → `probes: 12 pass / 0 fail` after its
  `PROPOSAL-VISIBLE` arm was rebuilt to pin A3 in BOTH directions (0 on the real tree, 1 once a marker is
  injected) instead of asserting a count that no longer exists. The criterion arrives with owners:
  `G3-GRADING.5` is required (trousers + pocket + derived buttonhole), `G3-GRADING.15` is created (the classic
  collar), and `.14`'s exit review fails if a §3.2 garment has no leaf's evidence — so the capture claim still
  derives: `run_tree_coverage_census.sh` → `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned /
  0 orphan(s) / 0 dead link(s)`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `13 suite(s) green` (twelve before this slice; the thirteenth is the tree-coverage suite `PLANNING.6` adds),
  with `run_live_doc_size_probes.sh` → `probes: 5 pass / 0 fail` including the new `REAL-3` arm that stales the
  real registry to `at=v0.2` and requires the refusal, and `check_live_doc_size.sh --self-test` →
  `15 arms, 0 failed` (eleven before `SPINE.4.5`); `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`, `exit=0`, the roadmap row inside its
  re-based debt and its `240` B maxline health (widest `220` B); the book's other censuses are green and
  unchanged — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)`, standards
  `6 registered / 6 designations used / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces /
  0 mismatch(es)`; `make book` → `exit=0`. No Rust changed.
- [x] **FIX** — amended §11 G3 through the roadmap's revision policy (criterion added, the "intermediate
  complexity" note rewritten so an intermediate can never substitute for a criterion); logged it in Appendix A
  with its source and the defect it closes; re-based the containment baseline with `at=v0.3` and the authority
  cited in the row's notes; dropped `(proposed)` from the four cells and rewrote §1 rule 2, §9, §12 and §14 so
  no sentence still calls the gap open; marked `decision_d32-proving-gates-proposed-roadmap-amendment.md` as
  **applied** (keeping the rejection path, because a future revision may withdraw the criterion); marked the
  containment adoption record's trigger 3 as fired and discharged; gave `G3-GRADING` the two leaves and the
  acceptance-table row; fixed the coverage census's tree enumeration and put it under `make probes` (D44,
  `PLANNING.6`); and added the revision-aware baseline (`SPINE.4.5`).
- [x] **LOCKSTEP** — `MEMORY.md` (v0.3, the amendment no longer pending), `LIVE_STATUS.md`, `CHANGELOG.md`,
  `DEV_NOTES.md`, `docs/TASK_TREE.md` (the coverage claim now cites both the census and its probe suite),
  `TOOLBOX.md` (two new rows), `docs/decisions/INDEX.md` and the regenerated Knowledge Map; D44 logged and
  fixed in `PLANNING.md`; `.15`'s scope narrows because the proposal it was to carry is applied. Lesson
  promotion: **promoted** — `decision_revision-aware-containment-baseline.md` carries an `answers:` line.

### `G0-CONTRACT.14b` — an empty seat is held acting with limits, or it is vacant

- [x] **REPRODUCE / ISSUE** — governance §8 listed three seats as "blocked on the director" and every chapter
  that needed a domain reviewer pointed at a leaf that cannot name one:
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/` → the standards registry's owner cells and the
  fixture's `assumed`-constant bullets, i.e. three chapters deferring to an event that had no mechanism. The
  director then delegated the finding ("make the necessary calls and every needed action"), which makes the
  vacancy this repository's problem to state precisely rather than to wait on.
- [x] **ROOT CAUSE (WHY + WHERE)** — the chapter was drafted under a ruling that reserved naming humans, so it
  recorded the gap honestly and stopped there: "blocked" is a state, not a plan. What was missing is the
  distinction between two kinds of authority — **office**, which the director already holds and can therefore
  hold *acting*, and **competence**, which nobody in this repository has and no acting arrangement can supply.
  Treating the three seats alike would either stall the project owner's decisions unnecessarily or, worse,
  imply that an engineer can certify a sewing judgement.
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` §8 is rewritten as three subsections: the seat
  table now names who holds each seat (`director, acting (§8.1)` twice, `**nobody — vacant, not acting**` for
  the domain expert); §8.1 states the acting authority and four hard prohibitions on it (no confirming the
  fixture's `assumed` constants, no signing a golden's semantic half, no ruling a safety-relevant term, no
  approving a byte-changing Factory Profile — which under §1's conjunctive rule leaves such a change
  unapproved with the missing half reported); §8.2 states the ask per seat so naming one is a single act.
  `wc -lc docs/book/src/governance.md` → `233` lines / `18 465` B, widest line `185` B, inside the
  `book_collection` per-part health of `400` / `24 576` / `200`; `make book` → `INFO HTML book written to …`,
  `exit=0`. The rule is recorded in `docs/decisions/decision_unnamed-seats-acting-authority.md` (indexed,
  `answers:` line) and the deferring chapters now cite the seat's real state:
  `grep -rc 'vacant' docs/book/src/spec/standards.md docs/book/src/spec/reference-skirt.md
  docs/book/src/spec/glossary.md` → `3`, `2`, `1`, and the stale deferral is gone —
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/ | grep -c .` → `0`, `rc=1`. The standards census still
  resolves every owner cell: `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`,
  `exit=0`; the book's censuses are green — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)` (the new
  prose introduces no undeclared machine token), standards `6 registered / 0 failure(s)`, matrix
  `105 rows / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  `make probes` → `13 suite(s) green`. No Rust and no instrument changed in this leaf.
- [x] **FIX** — rewrote §8 and its status block, §2's "currently" column for the three seats, the fixture
  chapter's two reviewer references, the glossary's two, the standards registry's two owner cells and its
  verification-plan step 1 — each cross-chapter link written as `../governance.md`, because a chapter under
  `spec/` reaches the book's top level one directory up and a dead link here would have been the D28 class
  (caught by building the book, not by reading the path); added the decision record and its index row.
- [x] **The first G2 golden stays gated, and that is the point of the leaf.** The tempting move was to record
  the director as acting domain expert so the schedule clears; the honest one is a vacant seat with a named
  consequence, because a golden frozen over an unreviewed `assumed` constant freezes a guess with the
  confidence of a fact, and the fixture chapter says so where a plan will hit it.
- [x] **LOCKSTEP** — the leaf, this checklist and the tree's blockers; `MEMORY.md`'s blocker bullet;
  `LIVE_STATUS.md`; `CHANGELOG.md`; `docs/decisions/INDEX.md` and the regenerated Knowledge Map. Lesson
  promotion: **promoted** — the record carries an `answers:` line.

Gate-level closure is recorded by `G0-CONTRACT.15`; each leaf carries its own evidence in the
Verification Log, and a leaf that stages code additionally fills a `### <leaf-id>` checklist subsection
with real tool output in the same commit as the change — in this file while it is being landed, then in
[`G0-CONTRACT-evidence.md`](G0-CONTRACT-evidence.md). Neither file carries unticked placeholder boxes:
the spine's acceptance gate judges the FIRST matching box in a file, so
a placeholder shadows real evidence and falsely rejects honest work (defect D15, measured by the
`SPINE.7` probe; local mitigation `SPINE.8`).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-30` | `G0-CONTRACT.14b` | the chapter's size and widest line; `make book`; the four book censuses; `grep` for the stale deferrals; `make gate`; containment | `233` lines / `18 465` B / widest `185`; `exit=0`; `276 terms`, `6 registered`, `105 rows`, `20 rows / 4 checks / 5 pieces` — all `0 failure(s)`; `0` stale deferrals left in `spec/`; all doctrines green; `OK — 84 files measured` |
| `2026-09-30` | `G0-CONTRACT.4c` | the roadmap's own markers; the matrix census + probes; the coverage census + its new suite; containment and its `--self-test`; every other census; `make gate`/`probes`/`book` | v0.3 at `947` lines / `52 818` B with the criterion at line `712` and the disposition logged; `105 rows / 0 failure(s)`, `D32 rows: 0`, `proposed cells: 0`; `10 lanes / 13 trees / 2 sibling(s) / 0 unowned`; `probes: 12 / 7 / 5 / 9 pass, 0 fail`; `15 arms, 0 failed`; all doctrines green; `13 suite(s) green` |
| `2026-09-30` | `G0-CONTRACT.14` | chapter size and widest line; `make book` and the rendered page; the three book censuses; the fixture derivation; `make gate`; `make probes`; containment | `198` lines / `15 593` B / widest `199`; `exit=0` and `governance.html` present; `276 terms`, `105 rows`, `6 registered`, all `0 failure(s)`; `20 rows / 4 checks / 5 pieces / 0 mismatch(es)`; all doctrines green; `12 suite(s) green`; `OK — 77 files measured` |
| `2026-09-30` | `G0-CONTRACT.4b` | matrix census + probes; ledger probes (DESCRIPTOR generalized); glossary and standards censuses; the rollover's losslessness; `make book`/`gate`/`probes`; containment | `105 rows / 0 failure(s)`, `D32 rows: 0`, `proposed cells: 4`; `probes: 12 pass / 0 fail` and `8 pass / 0 fail`; the sealed tail is byte-identical to `HEAD`, `sha256:d3b94e9a…` reproducing; `276 terms` and `6 registered`, both `0 failure(s)`; all doctrines green |
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
| `G0-CONTRACT.4b` | `STITCHCAD-G0-0004b (leaf G0-CONTRACT.4b): every envelope feature has a gate that proves it` | D32 resolved; four cells `(proposed)`; the G3 amendment quoted as a proposal; A3 advisory + its arm; the dev-notes rollover |
| `G0-CONTRACT.14` | `STITCHCAD-G0-0014 (leaf G0-CONTRACT.14): the governance model, and the three empty seats` | two review paths, roles by authority, goldens signed twice, procurement fallbacks with their cost; the naming stays the director's |
| `G0-CONTRACT.4c` | `STITCHCAD-G0-0004c (leaf G0-CONTRACT.4c): the envelope criterion is law, not a proposal` | roadmap v0.3; four cells committed; `G3-GRADING.5`/`.15` own the garments; the baseline is revision-aware |
| `G0-CONTRACT.14b` | `STITCHCAD-G0-0014b (leaf G0-CONTRACT.14b): an empty seat is held acting, or it is vacant` | two seats acting with hard limits, the domain seat openly vacant, the ask per seat written down; the first G2 golden stays gated on a real name |
| `G0-CONTRACT.9`–`.12`, `.15`–`.17` | `pending` | — |

## Changelog

- `2026-09-30`: `.4c` landed — the D32 proposal is ruled approved and applied. Roadmap **v0.3** gives §11 G3
  an *envelope coverage* exit criterion (every garment §3.2 names drafts, grades and exports at that gate or an
  earlier one), logged in Appendix A with its source, and the four matrix cells dropped `(proposed)`. The
  criterion arrives with owners: `G3-GRADING.5` became required and `.15` was created, and `.14`'s exit review
  now fails if a §3.2 garment has no leaf's evidence. Applying it needed a mechanism the data plane did not
  have — the `roadmap` debt baseline was measured at exactly the file's size, so a legitimate revision could
  only land by hand-widening a number — which is `SPINE.4.5`'s revision-aware baseline, and the containment
  adoption note's deferred trigger 3, fired and discharged. `PLANNING.6` fixed the coverage census that the
  evidence siblings had broken (D44) and put it under `make probes`.

- `2026-09-30`: `.14` landed — the governance model is drafted in full: every change class lands on exactly
  one of two review paths, a change that alters exported bytes needs the domain expert **and** the maintainer,
  seven roles are defined by the decision each may make and whether an agent may hold it, a golden is a
  release-contract event with two signatures and is never frozen over an `assumed` constant, a contested
  default becomes a profile parameter rather than a verdict, and every procurement item carries its fallback
  with what the fallback costs in evidence. The three seats that need a named human are in one table for the
  director, and the four questions deliberately left open are named so they arrive as decisions rather than
  emergencies. Six of the chapter's rules are project decisions and are recorded as such in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`. The slice also obeys the
  evidence-split convention `.4b` recorded: `.4b`'s checklist moved to `G0-CONTRACT-evidence.md`.

- `2026-09-30`: `.4b` landed — D32 is resolved: classic collar, trousers, button/buttonhole and pocket are
  assigned to **G3** (buttons also to **G5**, whose tech-pack clause already requires notions) and fly
  construction to **G7**, whose exit already requires named limitations. No row says `unnamed (D32)` any
  more — the census's A1 advisory reports `0` — and the four cells that depend on a roadmap change say
  `(proposed)`, which the new A3 advisory prints on every run with a probe arm that requires it to notice a
  removed marker. The exact amendment (one added G3 exit criterion over §3.2's garment list, plus the
  rewritten complexity note) is quoted current-vs-proposed with line numbers in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`, marked **proposed**: the
  roadmap is the director's to amend and `.15` puts it to him. The slice also performed the dev-notes
  rollover its own append triggered (four lessons sealed byte-identically, digest verified) and generalized
  the ledger probe's DESCRIPTOR rule to every `docs/history/*.md` segment, with a `DEVNOTES-DIGEST` arm
  pinning the generalization; D40 records the two legs that stay changelog-only, owned by `SPINE.19`.

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
