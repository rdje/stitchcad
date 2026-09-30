# G0-CONTRACT: product & semantic contract (roadmap gate G0)

## Metadata

- Tree ID: `G0-CONTRACT`
- Status: `done` for everything this repository can do — every engineering clause of gate G0 is met and
  derived; the gate itself stays **open** on one clause the director's ruling of `2026-09-30` accepts as
  open (evaluation-seat procurement), and its **closure is unapproved** because the reviewing party
  authored most of what it reviews (governance §6.1 rule 2). The frontier moves to `G1-SLICE`.
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

The **verdicts are not kept in this table**: `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh`
derives them from `ROADMAP.md` §11's own clause list and from `docs/tasks/artifacts/g0_exit/g0_exit_clauses.tsv`,
running the check each clause cites. A second hand-kept verdict column would be the drift this tree spent a
gate removing.

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
  Status: `done`
  Goal: record ADR-0003 (construction recipe primary) **and specify the formula language v1**:
  operators, units inside expressions, conditionals (multi-size branching), name binding
  (measurements, prior points/lengths/angles, profile parameters), evaluation order, error and
  dimension rules.
  Acceptance: the language is implementable from the chapter alone — grammar, type/unit rules,
  determinism requirement, worked examples over the reference skirt; the record names what is
  deliberately excluded (NURBS-class expressions, implicit solving).
  Verification: recorded below and in the acceptance checklist — the chapter is three parts
  (`308` / `247` / `92` lines, `20 593` / `12 690` / `7 116` B, each inside the `book_collection`
  per-part health of `400` / `24 576`); its census reports `17 bindings / 4 assertions / 13 refusals /
  0 mismatch(es)` with `names shared with the fixture: 12 · disagreements: 0`; its probe suite is at
  `15 pass / 0 fail`; the glossary grew to `289 terms / 8 parts / 153 tokens / 0 failure(s)`; every
  other census and gate is green.
  Commit: `STITCHCAD-G0-0009`

- ID: `G0-CONTRACT.10`
  Status: `done`
  Goal: record ADR-0004 (interchange dialects) and specify the StitchCAD interchange profiles:
  AAMA named layers × ASTM numbered layers, cut-as-1 × sew-as-1, R12 × R13, BLOCK-per-piece,
  SST/PST, grading modes, tessellation policy — with ASTM D6673-10's withdrawal recorded.
  Acceptance: the layer table is reproduced with both naming modes; each mode is a named,
  separately validated export target; no claim of a universal package; every external claim
  carries a verification status.
  Verification: recorded below and in the acceptance checklist — the chapter is `299` lines /
  `21 314` B, inside the `book_collection` per-part health of `400` / `24 576`; its census reads the
  roadmap's own layer bullet and reports `17 layers / 4 targets / 12 entities / 0 failure(s)`; its
  probe suite is at `13 pass / 0 fail`; the glossary is at `294 terms / 8 parts / 155 tokens /
  0 failure(s)` with `16` entries repointed from this leaf to a chapter clause; every other census
  and gate is green.
  Commit: `STITCHCAD-G0-0010`

- ID: `G0-CONTRACT.11`
  Status: `done`
  Goal: record ADR-0002 (UI stack + canvas hosting) as a decision **structure**: chrome choice,
  the three canvas topologies, the egui/iced dev-shell ruling, the TypeScript domain-logic ban,
  and the exact G1 spike protocol whose evidence settles canvas hosting.
  Acceptance: the spike's measurements, pass/fail criteria and decision rule are written now,
  so the G1 outcome cannot be argued after the fact.
  Verification: recorded below and in the acceptance checklist — the protocol is a decision record
  plus a data plane (`spike_gates.tsv`, `spike_topologies.tsv`, `spike_rule.tsv`, an empty
  `results.tsv`) and a verdict instrument that prints `PENDING` until `G1-SLICE.13` measures; its
  probe suite is at `13 pass / 0 fail` and includes an arm that tightens a threshold in the TSV and
  requires the verdict to change; every gate and census is green.
  Commit: `STITCHCAD-G0-0011`

- ID: `G0-CONTRACT.12`
  Status: `done`
  Goal: specify approval states and the release contract (§9): manifest contents, approval
  binding to package identity, stale-ification, package-completeness checking, the graduated
  acceptance states, and the human-only approval rule for agents.
  Acceptance: every manifest field is named with its source; the graduated states are ordered
  with the evidence each requires; the scoped-approval and scope-narrowing rules are stated.
  Verification: recorded below and in the acceptance checklist — the chapter is `257` lines /
  `19 010` B, inside the `book_collection` per-part health of `400` / `24 576`; its census parses
  roadmap §9 and §8.2 and the ontology's state table and reports `9 manifest fields / 6 states /
  8 matrix rows / 0 failure(s)`; its probe suite is at `14 pass / 0 fail`; the glossary is at
  `298 terms / 8 parts / 156 tokens / 0 failure(s)` with `12` entries repointed from this leaf to a
  clause; every other census and gate is green.
  Commit: `STITCHCAD-G0-0012`

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

- ID: `G0-CONTRACT.14c`
  Status: `done`
  Goal: govern the case this project actually runs in — one party proposing and applying a decision under
  delegation. Finding 1 of the `2026-09-30` session was that the engineer proposed the `ROADMAP.md` §11 G3
  amendment in `.4b` and applied it in `.4c`; that was disclosed in a report, and a disclosure decays while a
  rule does not.
  Acceptance: governance states the rule for decisions made under delegation; the roadmap's own disposition
  entry discloses that author and applier were the same party; the gate that consumes the amendment may not be
  signed by its author; a decision record carries the reasoning and the founding instance.
  Verification: recorded below and in the acceptance checklist — governance §6.1 carries five rules, the
  roadmap's Appendix A v0.3 entry names the self-application, `G3-GRADING.14`'s acceptance withholds approval
  from the criterion's author, and `make gate` is green with the containment baseline re-based to v0.3's final
  state under the revision-aware rule `SPINE.4.5` built.
  Commit: `STITCHCAD-G0-0014c`

- ID: `G0-CONTRACT.19`
  Status: `done`
  Goal: make "what this project does not know yet" a derived list rather than an impression — finding 2's
  second half. The domain seat is vacant, so everything waiting on it must be enumerable by one command, and an
  edit that quietly drops an `assumed` marker must change a count somebody reads.
  Acceptance: a census enumerates every uncertainty marker the book's own vocabulary uses, with the authority
  that resolves each; it refuses a blocking marker whose verification-status section names no resolver; a probe
  suite proves it notices an unowned claim and a chapter added later, and proves it leaves a definition alone;
  governance and the fixture chapter point at it; both are in `TOOLBOX.md`.
  Verification: recorded below and in the acceptance checklist — `uncertainty census: 88 markers / 10 files /
  0 unowned / 0 failure(s)`, `exit=0`, and `probes: 6 pass / 0 fail` including an owned-claim control arm.
  Commit: `STITCHCAD-G0-0019`

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
  Status: `done` for everything this repository can do — the review is derived, complete and re-runnable;
  the **gate stays open** on clause G0-12 and its **closure is unapproved** (governance §6.1 rule 2, and the
  director's ruling of `2026-09-30` which accepts both facts rather than waiting on them)
  Goal: G0 exit review — walk the gate's exit clause list, cite the deliverable and its
  re-derivable check for each, update `LIVE_STATUS.md` and the roadmap status line, and hand
  the frontier to `G1-SLICE`.
  Acceptance: every clause is `met` with a cited artifact or `not met` with a named blocker;
  no clause is marked met on prose alone.
  Verification: recorded below and in the acceptance checklist —
  `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh` → `G0 EXIT: 18 met / 1 not met / 19 clauses —
  the engineering clauses are met and the gate stays open on 1 human act(s) this repository cannot
  perform`, `exit=0`, in `2` seconds, with every `met` backed by a check the review *ran* (eleven of them
  censuses, two of them `cargo test` and `make wasm`); its probe suite is at `10 pass / 0 fail`, including
  ROADMAP-GROWS, which adds a clause to a copy of §11 and requires the review to refuse. The roadmap's
  status line is unchanged and that is the finding: it reads "remains DRAFT until the G0 exit criteria are
  met", and one criterion is not met.
  Commit: `STITCHCAD-G0-0015`

- ID: `G0-CONTRACT.16`
  Status: `done`
  Goal: choose the ONE message system (§7.6: Fluent **or** ICU — not "Fluent or ICU"; if both ends
  are needed, a designed bridge) and specify the externalization architecture: the CI lint that
  fails on an inline user-facing string, the glossary/termbase per language (safety-relevant terms
  first: notch types, sew/cut line aliases factories use in rejection emails), pseudolocalization,
  and locale-independent canonical files (decimal-comma input ≠ stored meaning).
  Acceptance: a decision record names the chosen system, the rejected one and the reason; the
  architecture chapter states the lint rule, the termbase format and the RTL rule (mirrored layout,
  never mirrored geometry); stable diagnostic codes + typed arguments + units are the API contract,
  with localized prose as a presentation field only.
  Verification: recorded below and in the acceptance checklist — Fluent is chosen at both ends with
  both stacks' licences and activity `read-external` in the chapter's §2 table; the chapter is `250`
  lines / `17 650` B, inside the per-part health; its census derives the message inventory from four
  chapters' token tables and the crate's own error enum and reports `8 families / 64 message ids /
  0 failure(s)`; its probe suite is at `11 pass / 0 fail`; the glossary grew to `305 terms / 9 parts
  / 156 tokens / 0 failure(s)`; every other census and gate is green.
  Commit: `STITCHCAD-G0-0016`

- ID: `G0-CONTRACT.17`
  Status: `done`
  Goal: the command-layer contract (§4.4) — the typed command set, atomic groups, preview/commit,
  revision preconditions, idempotency, structured errors, progress for long operations, and the
  **undo/redo semantics that §4.4 requires to be defined at G0** (granularity per command group),
  plus the UI↔API↔MCP workflow-parity invariant and the shape of the coverage table that proves it.
  Acceptance: each command class states its granularity, reversibility and precondition; the parity
  table's columns and its generation rule are specified so G1 can populate it mechanically;
  agent authority levels (§7.8: inspect / propose / commit / generate / approve) are defined here as
  command-layer concepts, not as tool descriptions.
  Verification: recorded below and in the acceptance checklist — the chapter is `264` lines /
  `18 959` B with a widest line of `174` B, inside the per-part health; its census parses the
  roadmap's own §4.4 command list and §7.8 level list and reports `17 commands / 5 classes /
  5 levels / 0 failure(s)`; its probe suite is at `14 pass / 0 fail`; the glossary is at `308 terms
  / 9 parts / 156 tokens / 0 failure(s)` with `19` entries repointed from this leaf to a clause;
  every other census and gate is green.
  Commit: `STITCHCAD-G0-0017`

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
| 15c | `G0-CONTRACT.14c` | `done` | the case this project runs in is governed: a delegated decision records that its author applied it, its consequences become instruments, and its author never approves its evidence |
| 15d | `G0-CONTRACT.19` | `done` | what the project does not know is one command away: the uncertainty census enumerates every marker with its resolving authority and refuses an unowned one |
| 15b | `G0-CONTRACT.4c` | `done` | the director delegated the finding outright, so `.4b`'s proposal is ruled approved and applied: roadmap **v0.3** carries the envelope-coverage criterion, the four cells are committed gates, and `G3-GRADING.5`/`.15` own the garments |
| 16 | `G0-CONTRACT.9` | `done` | ADR-0003 landed in full: the formula language v1 as three censused parts (contract, grammar, examples), a reference evaluator that reads the chapter's own tables, Aldrich's metric pattern cutting named as the reference drafting system, and `G3-GRADING.16` created to ship its blocks |
| 17 | `G0-CONTRACT.10` | `done` | ADR-0004 landed: six axes, a closed registry of four targets, the seventeen-layer table in both naming modes read against the roadmap itself, one polyline-only entity set with arcs exact as bulges, three grading carriages, and the receiver-config record |
| 18 | `G0-CONTRACT.11` | `done` | ADR-0002 recorded as a decision structure: the chrome, the dev shell and the TypeScript ban are `active`, canvas hosting is `proposed`, and the corpus, the seven gates and the six-rule verdict are written down before anybody measures — with an instrument that derives the outcome from a TSV data plane |
| 19 | `G0-CONTRACT.12` | `done` | the release contract landed: nine manifest fields each with its source, identity as the manifest's digest, completeness against the declared construction, the six acceptance states in the roadmap's order with the evidence each needs, scope that narrows by itself, human-only approval, and §8.2's policy matrix tuned and recorded |
| 20 | `G0-CONTRACT.16` | `done` | one message system chosen with its evidence read: Fluent at both ends, ICU4X rejected on the boundary rather than on quality; the lint, the termbase, pseudolocalization, the locale-independent canonical file, the RTL geometry rule and three review tiers, with the message inventory derived from four chapters and the crate |
| 21 | `G0-CONTRACT.17` | `done` | the command layer landed: five classes and seventeen commands in one machine-readable table, undo at the atomic group restoring semantics rather than contours, preview/commit with a re-checked precondition, revision preconditions and idempotency keys, the five authority levels as core-enforced permissions with `approve` unholdable by an agent, and a parity table whose columns and generation rule are normative while its rows stay G5's evidence |
| 22 | `G0-CONTRACT.15` | `done` | the exit review is a command, not a paragraph: 18 of 19 clauses met with a check the review ran, one `not met` and accepted open by the director's ruling, and the gate's closure recorded **unapproved** under governance §6.1 because its reviewer authored most of what it reviews |
| 23 | — | — | **gate G0 has no further leaf.** The frontier moves to `G1-SLICE.1` (the workspace crate layout), which is where the specification meets Rust for the first time since `sc-units`. G0's one open clause travels with the ruling that accepted it, and `G2-2D.15` owns the physical evidence that would close the domain questions |

## Decisions

Earlier entries — leaves `.1`, `.4`, `.6`, `.7`, `.13d`, `.4b`, `.14`, `.4c` and the two that opened this
section — are sealed in
[`docs/history/stitchcad-g0-contract-decisions-part1.md`](../history/stitchcad-g0-contract-decisions-part1.md)
(112 lines, 10079 bytes, `sha256:8d22b367…`), under the remedy defect D49 names for a tree file's ledger tail.

- `2026-09-30`, leaf `.9`: **the formula language's tables are its test suite.** The census's reference
  evaluator reads the chapter's own kind, unit-ratio, product-law and signature tables and type-checks every
  worked example with them, so a construct no table declares cannot be evaluated and a signature written
  wrongly reddens the run instead of the product. The price was regularity: `sqrt` needed two rows
  (`area`→`length`, `ratio`→`ratio`) and `min` a variadic marker, because a prose signature
  ("≥ 2 values of one kind") is not parseable. Instrument written second, so two tables were rewritten once.
- `2026-09-30`, leaf `.9`: **a code span is a claim that the span is a machine token**, so grammar notation
  is set in italics and carries its own declared table. Measured, not theorised: the metavariable `T` was
  already the token the glossary's `T-notch` entry owns, so one token carried two meanings while the
  "one token, one meaning" census stayed green — the token *was* owned — and its sibling `N`, owned by
  nobody, is the only reason the collision surfaced at all.
- `2026-09-30`, leaf `.9`: **a chapter born at 87 % of its ceiling is partitioned, not trimmed.** The
  language landed as one file at `599` lines / `37 317` B against a `book_collection` per-part health of
  `400` / `24 576` and a ceiling of `700` / `40 960`, with every table normative and the movable prose
  already in the decision record. The remedy is the containment doctrine's own for a `partitioned_canonical`
  surface — contract / grammar / examples at `308` / `247` / `92` lines, each inside health — and the parts
  table is censused in both directions so a fourth file cannot appear unlisted.
- `2026-09-30`, leaf `.9`: **the reference drafting system is adopted as a method, never as a text**, its
  numbers enter per step as `read-external` or stay `unverified-with-owner`, and the fixture is deliberately
  NOT a transcription of it so the G2 goldens keep an independent subject. The blocks have an owner
  (`G3-GRADING.16`, created by this leaf) and a named re-open condition (the domain expert substitutes a
  system, which re-derives every block and re-cites every number).
- `2026-09-30`, leaf `.9`: **a recipe carries its own oracles.** `assert` is a statement form, a failed
  assertion refuses the whole recipe, and the fixture's four closure checks are written in the language as
  well as in its chapter — D33's lesson inside the language rather than beside it.

- `2026-09-30`, leaf `.10`: **an export target is a tuple over declared axes, and the registry is closed.**
  A writer flag nobody validated is a claim about a receiver nobody tested, so a target exists only as a row
  with a G6 validation behind it, and a tuple outside the registry is refused naming the nearest target. The
  axes and the registry columns are censused in both directions, so neither a silent axis nor an undeclared
  column can appear.
- `2026-09-30`, leaf `.10`: **the layer table is read against `ROADMAP.md`, not transcribed beside it.**
  Seventeen layers and seven AAMA names are parsed out of the roadmap's own bullet on every run, which is
  what makes "every layer the convention names is dispositioned" a derived claim — and the instrument's first
  cut read one wrapped line of that bullet, found `3` layers, and reported `21` chapter breaches that were
  the instrument measuring a paragraph's first line.
- `2026-09-30`, leaf `.10`: **one entity set for both releases, arcs exact as bulges, Béziers tessellated.**
  A per-release entity set is two writers and two ways to disagree with a legacy importer; a standalone
  `ARC` is refused although its geometry would be exact, because two families for one boundary is the same
  disagreement wearing a different entity. The census pins the floor and the ceiling of the set, so a quiet
  widening reddens.
- `2026-09-30`, leaf `.10`: **what a receiver owns, this chapter does not invent.** SST and PST are required
  and their field content is deliberately unspecified: the syntax is case-sensitive and receiver-specific, so
  a field table written at G0 would be a guess in a normative font, and G6 is the only oracle. The same rule
  keeps layers 84–87 absent rather than filled with placeholder curves no design authored.

- `2026-09-30`, leaf `.11`: **a decision may be recorded before its evidence exists, if the record says
  which half is waiting.** ADR-0002 is `active` for the chrome, the dev shell and the TypeScript ban and
  `proposed` for canvas hosting, in the status line rather than in a footnote — so a reader cannot mistake
  an unmeasured choice for a settled one, and `G1-SLICE.13` has a state to move.
- `2026-09-30`, leaf `.11`: **the decision rule is an instrument over a data plane, not prose.** The gates,
  the applicability table and the rule's parameters are three TSVs; `run_spike_verdict.sh` applies R1–R6 to
  whatever `results.tsv` holds and prints the rule that decided each profile. Tightening a threshold is
  therefore a diff a reviewer sees, which the GATES-READ probe arm pins by changing the verdict.
- `2026-09-30`, leaf `.11`: **the corpus is declared before the measurement, with a re-run trigger.** 16
  pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000 fidelity round trips — a corpus
  invented during a spike is a corpus chosen to suit a result, and the trigger (a G3 garment exceeding the
  bound) is what keeps the verdict about this product rather than a smaller one.
- `2026-09-30`, leaf `.11`: **the canvas question is decided twice, once per runtime profile**, because a
  native-only topology can win one and be ineligible for the other; R4 prefers a single renderer only where
  that is free (inside the margin in every profile), and where it is not, the record that closes the ADR must
  state the cost of two.
- `2026-09-30`, leaf `.11`: **an empty results file is a verdict, not an error.** `PENDING` with `exit=0` is
  the honest state at G0 and the reason the record's canvas half is `proposed`; a partial one is a refusal,
  because a verdict on unmeasured gates is exactly what the protocol exists to prevent.

- `2026-09-30`, leaf `.12`: **a package is never edited and an approval binds to a digest.** A similarity
  threshold for stale-ification was rejected because a threshold is a guess about which differences a
  factory cares about, made by the party that wants the approval to survive. Recorded in
  `docs/decisions/decision_release-package-identity-and-scope.md` with the five other alternatives it
  rejected.
- `2026-09-30`, leaf `.12`: **a claim's scope is the intersection of its evidence, and it narrows by
  itself.** Widening is a new evidence record, never an inference, which is what makes G7's
  supported-envelope statement a projection of acceptances rather than an impression of them.
- `2026-09-30`, leaf `.12`: **§8.2's example matrix is tuned in a table, not silently.** Notch geometry in a
  draft export became `sidecar` instead of "default + visible badge", because a default is a value
  substituted for an observation and roadmap §8.3 forbids it; the chapter's tuning table quotes the
  roadmap's row beside what it became, so a reviewer can see the change rather than trust it.
- `2026-09-30`, leaf `.12`: **the four dispositions are a closed vocabulary and `permit` is not one of the
  matrix's cells.** The matrix is consulted for the `unknown` state only — the other four states have fixed
  rules — so a cell saying `permit` would claim an unknown is simply fine, and the census refuses one.

- `2026-09-30`, leaf `.16`: **the message system is chosen on the boundary, not on quality.** Both
  candidates are alive and permissively licensed, so the deciding fact is that Fluent has an implementation
  on each side of the Rust/TypeScript boundary reading ONE catalogue format, while ICU4X would leave the
  browser on the platform's Intl — two dialects, which is the failure §7.6's "not 'Fluent or ICU'" exists to
  prevent. The bridge stays in reserve for a measured plural-rule or locale-data gap.
- `2026-09-30`, leaf `.16`: **the message id is the diagnostic token.** No numeric ids and no slugs derived
  from English copy, so there is no second numbering to keep in step and a token change is an API break
  rather than a copy edit. The inventory that follows is derived from the chapters and from
  `crates/sc-units/src/error.rs`, never counted by hand — and deriving it found two errors in the chapter's
  first draft: a `geom_*` token folded into the `env_*` family, and `UnitError` carrying five variants where
  the chapter said four.
- `2026-09-30`, leaf `.16`: **review tiers are absolute where a mistranslation reaches fabric.** `strict`
  and `safety` ship only at 100 % translated *and* reviewed, and a percentage threshold was rejected
  because "90 % of the safety tier" means one message in ten may say the wrong thing about a cut line. The
  consequence is stated rather than hidden: with the domain seat vacant, no pack can ship today.
- `2026-09-30`, leaf `.16`: **RTL is a test, not an intention.** An RTL render and an LTR render of one
  design must produce one canonical byte string, and a difference is `i18n_geometry_mirrored` — because
  "mirrored layout, never mirrored geometry" is unfalsifiable as a sentence and trivially falsifiable as a
  comparison.

- `2026-09-30`, leaf `.17`: **the undo granularity is the atomic group, and an undo restores semantics.**
  Per-command undo was rejected because "add a dart" is a centre, two legs, an intake and a truing, and
  undoing three of the four leaves a piece nobody drafted; contour-level undo was rejected because two
  designs with identical contours and different recipes are different designs (ontology §9), so restoring
  pixels can silently change which design is on screen. Evaluations and artifacts are **discarded** rather
  than undone, and the history is not canonical content.
- `2026-09-30`, leaf `.17`: **authority is a permission on a command class, enforced in the core.** A
  per-tool permission list was rejected because a tool manifest is data an adapter ships, and the core is the
  only place three adapters cannot disagree. A sixth level is refused by the census as a governance change
  rather than a chapter edit, and `approve` stays unholdable by an agent — the release chapter's
  `release_approver_not_human` is the same rule seen from the package.
- `2026-09-30`, leaf `.17`: **the parity table is generated, and empty at G0 on purpose.** Its columns, its
  closed cell vocabulary and the rule that a workflow is marked present only when a test completes it through
  that adapter are normative now; its rows are G5's evidence. A table filled in before the adapters exist is
  a claim about unwritten code — the same reason the canvas spike's `results.tsv` is empty.
- `2026-09-30`, leaf `.17`: **one number is deliberately not written.** The undo depth is declared to exist,
  to be bounded and to report when it drops the oldest group, but its value belongs with the resource bounds
  roadmap §10 requires at G1, where a measurement exists to derive it from. A bound guessed at G0 is a
  migration later.

- `2026-09-30`, leaf `.15`: **a gate review is a command, not a paragraph.** The leaf's acceptance forbids
  marking a clause met on prose alone, and a review *written* as prose is exactly that, so the review parses
  the roadmap's own clause list, runs the check each clause cites, and prints the verdict. Two properties
  follow: a clause cannot be dropped by forgetting to mention it (the closure is both directions), and a
  reviewer who did not write the chapters can reproduce the whole gate in two seconds.
- `2026-09-30`, leaf `.15`: **`not met` with a named blocker is a verdict, not a failure of the review.**
  G0-12 stays open because no evaluation seat is being procured, and the ruling that accepts this names what
  it costs — no receiver reads our artifacts back, so the interchange claims stay `cited-from-roadmap` and
  G6's validation falls to the partner-run fallback. A gate that reports 18 of 19 honestly is more useful
  than one that reports 19 of 19 generously.
- `2026-09-30`, leaf `.15`: **the gate's closure is unapproved, and that is recorded rather than worked
  around.** Governance §6.1 rule 2 withholds approval of a decision's evidence from its author; the reviewing
  party authored sixteen of the nineteen deliverables. The mitigation §6.1 prescribes is in place — the
  consequences are instruments anybody can run — so independence is *available* to the next reader instead of
  being held by anyone now. What the review therefore certifies is that the checks pass, not that the gate is
  approved.
- `2026-09-30`, leaf `.15`: **a ruling that names a cost must arrive with an owner.** The director's ruling
  makes the residual dependency a measurement rather than a credential, so `G2-2D.15` was created in the same
  commit to own the physical-evaluation protocol for the reference skirt: what to cut, sew and measure, which
  `assumed` constant each measurement falsifies, and what the project does when a measurement disagrees with
  the drafting.

## Open Questions

- Cut-on-fold vs paired front for the reference skirt: roadmap §11 G0 allows either. Decided
  in `G0-CONTRACT.13` with the reason recorded there (it changes the piece list and the
  notch/fold semantics the fixture must exercise).
- Which named drafting system ships as the v1 reference block set (roadmap ADR-0003: "one
  documented system, decided at G0") — **decided in `G0-CONTRACT.9`**: Aldrich's metric pattern
  cutting, with the blocks owned by `G3-GRADING.16`, the content `unverified-with-owner` until the
  source is procured, and substitution by the named domain expert as a recorded re-open condition.
- Whether shipping reference blocks *derived from* a commercial drafting source needs a licence
  clearance — flagged to the project owner's seat by `G0-CONTRACT.9` and deliberately not answered
  there; it is a legal question this repository has no authority over, and `G3-GRADING.16` cannot
  start transcribing until it is.
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

### `G0-CONTRACT.17` — the command layer is a contract, and the roadmap's own two lists are parsed to prove it

- [x] **REPRODUCE / ISSUE** — §4.4 required undo/redo semantics at G0 and the repository had none.
  `git ls-files 'docs/book/src/spec/command*' | wc -l` → `0`; the spec index carried the promise unlinked,
  `git show HEAD:docs/book/src/spec/index.md | grep -c '^| Command layer |'` → `1`, `rc=0`; and the whole
  vocabulary part was parked against this leaf,
  `git show HEAD:docs/book/src/spec/glossary/commands-and-authority.md | grep -c 'G0-CONTRACT\.17'` → `20`
  entries whose canonical object was a leaf rather than a clause, under a header note reading "until that
  chapter lands, the roadmap clause is cited".
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n 'Undo/redo semantics are defined at G0' ROADMAP.md` →
  `285:not one tool per getter. Undo/redo semantics are defined at G0 (granularity`, `rc=0`, i.e. §4.4 makes
  the granularity a G0 deliverable and not an implementation detail; §7.8 adds the five authority levels and
  §10 the audit trail. The clause had an owner and no artifact, so the cause is a leaf not taken — and the
  risk in leaving it is that undo granularity decided by an implementation is decided by whatever data
  structure was convenient, which is the D35 class (a contract settled by accident) at the level of the whole
  editing model.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/command_layer/run_command_layer_census.sh` →
  `command-layer census: 17 commands / 5 classes / 5 levels / 0 failure(s)`, `exit=0`, where the five
  commands come out of roadmap §4.4's own backticked list and the five levels out of §7.8's slash-separated
  one — the latter wrapped mid-item, which the first cut did not normalise and so parsed zero levels and
  reported five invented ones. Every §1.1 row's class, authority and reversibility is checked against the
  vocabularies §1 and §7 declare, derived from those tables rather than listed beside them. Discrimination is
  proved: `TMPDIR=$PWD/target/scratch bash
  docs/tasks/artifacts/command_layer/run_command_layer_probes.sh` → `probes: 14 pass / 0 fail`, including
  LEVEL-EXTRA and LEVEL-MISSING as separate arms (the two directions of K3 proved independently),
  HUMAN-ONLY (softening "is human-only" to "is usually a person" is refused), and CONTROL. Sizes: the chapter
  is `264` lines / `18 959` B, widest `174` B, and the record `71` / `6 024`, both inside their per-part
  health; `make book` → `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `21 suite(s) green`; the neighbouring censuses unchanged: glossary `308 terms / 9 parts / 156 tokens /
  0 failure(s)` after three new terms, nineteen repointed entries and a re-derived index, i18n
  `8 families / 64 message ids / 0 failure(s)`, release `9 manifest fields / 6 states / 8 matrix rows /
  0 failure(s)`, interchange `17 layers / 4 targets / 12 entities / 0 failure(s)`, formula language
  `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, fixture `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`, matrix `105 rows / 0 failure(s)`, standards `6 registered / 0 failure(s)`;
  `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 113 files measured`, `exit=0`. Two
  census bugs were found and fixed while building it: the wrapped authority list above, and a K5 population
  that counted `command_id` — a field of the command shape declared in §2 — as an undeclared diagnostic.
- [x] **FIX** — wrote the chapter (five classes with the vocabularies they own, seventeen commands in one
  machine-readable table, the seven fields of a command's shape, the undo rules including the three an
  implementation gets wrong by accident, preview/commit, revision preconditions and idempotency, structured
  errors and cancellable progress, the five authority levels with what each may not do, the parity table's
  eight columns and its generation rule, eight diagnostics); the decision record with ten rejected
  alternatives; the census and its 14-arm suite; three glossary terms and nineteen repointed entries plus
  four cross-references in three other chapters. **D49's third trigger discharged here:** the tree had
  reached 94 % of its byte ceiling, so the decisions of leaves `.1`–`.4c` were sealed into
  `docs/history/stitchcad-g0-contract-decisions-part1.md` (`112` lines / `10 079` B, digest reproduced by
  `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`, `33` segment verdicts) and `.16`'s checklist moved
  to the evidence sibling as the convention requires.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two instrument rows),
  `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in this
  commit. Lesson promotion: **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.15` — the gate review is a command, and it reports one clause this repository cannot close

- [x] **REPRODUCE / ISSUE** — gate G0 had nineteen obligations and no verdict for any of them.
  `git ls-files 'docs/tasks/artifacts/g0_exit/*' | wc -l` → `0`; the leaf was `pending` and the tree's
  acceptance table mapped each clause to a leaf and a deliverable but carried no verdict column —
  `git show HEAD:docs/tasks/G0-CONTRACT.md | grep -c 'not met'` → `1`, and that one is the acceptance rule's
  own wording, not a finding. So the gate's state existed only as an impression: twenty leaves marked `done`,
  and nothing that answered "is G0 met?".
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n '^### G0 — Product' ROADMAP.md` → `666`, `rc=0`, whose
  `**Exit:**` bullet is a semicolon-separated list of eleven obligations plus a `Fixture:` bullet, i.e. a
  population with a layout. A review written as prose over such a list drifts the way every other list in
  this repository drifted, and the leaf's own acceptance forbids the failure mode directly: "no clause is
  marked met on prose alone". So the cause is not that the review was unwritten but that a *written* review
  would have been the very thing the acceptance rule refuses — the review had to be derived and executed.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh` →
  `G0 EXIT: 18 met / 1 not met / 19 clauses — the engineering clauses are met and the gate stays open on
  1 human act(s) this repository cannot perform`, `exit=0`, in `2` seconds. It parses §11's exit bullet into
  fragments and the fixture bullet as a thirteenth, requires every fragment to be dispositioned by a row of
  `g0_exit_clauses.tsv` and every row's key to appear in a fragment, then **runs** each row's check: eleven
  censuses, `cargo test -q -p sc-units`, `make wasm`, `mdbook build`, and five artifact greps. The one
  `not met` is G0-12, evaluation-seat procurement, which the director's ruling of `2026-09-30` accepts as
  open with its cost named (no receiver ever reads our artifacts back, so the interchange claims stay
  `cited-from-roadmap` and G6 falls to the partner-run fallback). G0-13 is `met` as drafted with its
  qualification printed: the governance model and both review paths exist, the project owner is the director
  acting, and the domain seat is vacant. Discrimination is proved:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/g0_exit/run_g0_exit_review_probes.sh` →
  `probes: 10 pass / 0 fail`, including ROADMAP-GROWS (a clause added to a *copy* of §11 is refused),
  CHECK-FAILS (a failing instrument reads as an unmet clause and `GATE FAILS`, not as a broken review),
  NO-BLOCKER and CONTROL.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `22 suite(s) green`; every clause's own census re-run inside the review and green: matrix `105 rows /
  0 failure(s)`, glossary `308 terms / 9 parts / 0 failure(s)`, release `9 manifest fields / 6 states /
  8 matrix rows`, formula language `17 bindings / 4 assertions / 13 refusals`, interchange `17 layers /
  4 targets / 12 entities`, standards `6 registered`, fixture `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`, i18n `8 families / 64 message ids`, command layer `17 commands / 5 classes /
  5 levels`; `bash scripts/check_live_doc_size.sh` → `OK`, `exit=0`. Two instrument bugs were found and
  fixed while building it, both the same class as the ones this gate keeps producing: the exit-bullet parser
  looked for `**Exit:**` *after* stripping markdown emphasis, so it read nothing and reported fifteen rows as
  unmatched; and one roadmap clause spans two `;`-separated fragments, so a row needed a key list rather than
  a key.
- [x] **FIX** — built the review as a data plane plus an executor (`g0_exit_clauses.tsv`: 19 rows, each with
  its source, key, deliverable, check, authority, blocker and note) and its 10-arm probe suite; recorded the
  director's ruling of `2026-09-30` (no domain expert, no independent reviewer, the project proceeds with
  claims marked unapproved) as
  `docs/decisions/decision_director-ruling-2026-09-30-no-seats-proceed-unapproved.md` with its boundary
  table and the amendment that the residual dependency is a measurement rather than a credential; created
  `G2-2D.15` to own that measurement — the physical-evaluation protocol for the reference skirt — because a
  ruling that names a cost without an owner is a wish; and left the roadmap's status line **unchanged**, which
  is the honest outcome: line 9 reads "remains DRAFT until the G0 exit criteria are met" and one criterion is
  not met.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's status and decisions and logs;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two
  instrument rows), `docs/decisions/INDEX.md`, `docs/tasks/G2-2D.md` (the new leaf),
  `knowledge-map/subsystems.md` and the regenerated Knowledge Map in this commit. Lesson promotion:
  **promoted** — the ruling record carries an `answers:` line.

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
| `2026-09-30` | `G0-CONTRACT.15` | the exit review and its 10-arm probe suite; every clause's own census inside the review; `make gate`/`probes`; containment; the roadmap's status line | `18 met / 1 not met / 19 clauses`, `exit=0`, `2` s; `10 pass / 0 fail`; all eleven censuses green; `OK`; line 9 unchanged and still DRAFT |
| `2026-09-30` | `G0-CONTRACT.17` | the command-layer census and its 14-arm suite; the chapter's and record's sizes; the glossary census after three terms and nineteen repointed entries; `make gate`/`probes`/`book`; the eight neighbouring censuses; containment; the decisions seal and its digest | `17 commands / 5 classes / 5 levels / 0 failure(s)`; `14 pass / 0 fail`; `264` / `18 959`; `308 terms / 9 parts`; `21 suite(s) green`; sealed `112` / `10 079` |
| `2026-09-30` | `G0-CONTRACT.16` | the i18n census and its 11-arm suite; the chapter's and record's sizes; the glossary census after a ninth part; `make gate`/`probes`/`book`; the eight neighbouring censuses; containment; the tree's sealing | `8 families / 64 message ids / 0 failure(s)`; `11 pass / 0 fail`; `250` / `17 650`; `305 terms / 9 parts`; `20 suite(s) green`; tree `81 811` B after sealing `84` / `7 941` |
| `2026-09-30` | `G0-CONTRACT.12` | the release-contract census and its 14-arm suite; the chapter's size; the glossary census; `make gate`/`probes`/`book`; the eight neighbouring censuses; containment; the sibling's sealing | `9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`; `14 pass / 0 fail`; `257` / `19 010`; `298 terms`; `19 suite(s) green`; sibling `449` / `42 065` after sealing `560` / `52 573` |
| `2026-09-30` | `G0-CONTRACT.11` | the spike verdict instrument on its empty data plane and on twelve synthetic result sets; the record's size; `make gate`/`probes`/`book`; the seven book censuses; containment | `PENDING`, `exit=0`; `13 pass / 0 fail`; `78` lines / `7 244` B; `18 suite(s) green`; all green; `OK — 99 files measured` |
| `2026-09-30` | `G0-CONTRACT.10` | the interchange census and its 13-arm suite; the chapter's size; the glossary census; `make book`/`gate`/`probes`; the four neighbouring censuses; containment | `17 layers / 4 targets / 12 entities / 0 failure(s)`; `13 pass / 0 fail`; `303` / `21 650`; `294 terms`; `17 suite(s) green`; detail in the checklist |
| `2026-09-30` | `G0-CONTRACT.9` | the formula-language census and its 15-arm probe suite; the three parts' sizes and widest line; the glossary census; `make book`/`gate`/`probes`; the four neighbouring censuses; containment | `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, `12` fixture-shared names and `0` disagreements; `15 pass / 0 fail`; `16 suite(s) green`; detail in the checklist |
| `2026-09-30` | `G0-CONTRACT.14b` | the chapter's size and widest line; `make book`; the four book censuses; `grep` for the stale deferrals; `make gate`; containment | `233` lines / `18 465` B / widest `185`; `exit=0`; `276 terms`, `6 registered`, `105 rows`, `20 rows / 4 checks / 5 pieces` — all `0 failure(s)`; `0` stale deferrals left in `spec/`; all doctrines green; `OK — 84 files measured` |
| `2026-09-30` | `G0-CONTRACT.4c` | the roadmap's markers; matrix census + probes; coverage census + its new suite; containment and its `--self-test`; `make gate`/`probes`/`book` | v0.3 at `947` lines / `52 818` B, criterion at line `712`, disposition logged; `105 rows / 0 failure(s)`, `D32 rows: 0`, `proposed cells: 0`; `13 trees / 2 sibling(s) / 0 unowned`; `probes: 12 / 7 / 5 / 9`, `15 arms`, all `0 fail`; `13 suite(s) green` |
| `2026-09-30` | `G0-CONTRACT.14` | chapter size and widest line; `make book`; the three book censuses; the fixture derivation; `make gate`/`probes`; containment | `198` lines / `15 593` B / widest `199`; `exit=0` and `governance.html` present; `276 terms`, `105 rows`, `6 registered`, all `0 failure(s)`; `20 rows / 4 checks / 5 pieces / 0 mismatch(es)`; all doctrines green; `12 suite(s) green`; `OK — 77 files measured` |
| `2026-09-30` | `G0-CONTRACT.4b` | matrix census + probes; ledger probes (DESCRIPTOR generalized); glossary and standards censuses; the rollover's losslessness; `make gate` | `105 rows / 0 failure(s)`, `D32 rows: 0`, `proposed cells: 4`; `probes: 12 pass / 0 fail` and `8 pass / 0 fail`; the sealed tail is byte-identical to `HEAD`, `sha256:d3b94e9a…` reproducing; `276 terms` and `6 registered`, both `0 failure(s)`; all doctrines green |
| `2026-09-30` | `G0-CONTRACT.13d` | fixture derivation at `HEAD` vs the tree; its probes + two neutered-rule meta-checks; the three neighbouring censuses; `make gate`/`probes` | before `16 rows / 2 checks / 6 pieces / 7 mismatch(es)` `exit=1`, after `20 / 4 / 5 / 0` `exit=0`; `probes: 9 pass / 0 fail`, and neutering `B1` or `P2` reddens the arms that need them; `276 terms`, `105 rows`, `6 registered`, all `0 failure(s)` |
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

| `2026-09-30` | `G0-CONTRACT.14c` | governance §6.1 and its size; the roadmap's Appendix A entry and its re-based baseline; `G3-GRADING.14`'s acceptance; `make gate`; containment | five rules in §6.1; the v0.3 entry discloses the self-application; the exit review withholds approval from the criterion's author; `=== all doctrines green ===`; `OK — 89 files measured` with `roadmap` at `lines=951;bytes=53153;at=v0.3` |
| `2026-09-30` | `G0-CONTRACT.19` | the uncertainty census and its probe suite; the glossary census after the new term; `make book`; `make probes` | `88 markers / 10 files / 0 unowned / 0 failure(s)`, `exit=0`; `probes: 6 pass / 0 fail`; `277 terms / 8 parts / 146 tokens / 0 failure(s)`; `exit=0`; `15 suite(s) green` |

| `2026-09-30` | `.14c`/`.19` (CI verdict, observed after the exceptional push) | `make check`/`gate`/`probes`; `git push origin main`; the Actions runs **and jobs** APIs for `head_sha=3d9f2be` | `exit=0` all three; `a743d53..3d9f2be`, ahead `0`; **`doctrines` `success`**, **`rust` `success`** with job `check` green on every step. Read at job level: the `rust` RUN said `in_progress` for ~11 min after its only job had completed |

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
| `G0-CONTRACT.14c` | `STITCHCAD-G0-0014c (leaf G0-CONTRACT.14c): a delegated decision is bounded, not just disclosed` | governance §6.1; the roadmap's v0.3 entry discloses its author-applier |
| `G0-CONTRACT.19` | `STITCHCAD-G0-0019 (leaf G0-CONTRACT.19): what the project does not know is derived` | the uncertainty census plus its six arms; the glossary's link resolver fixed |
| `G0-CONTRACT.9` | `STITCHCAD-G0-0009 (leaf G0-CONTRACT.9): the formula language, and the drafting system named with it` | three censused parts; an evaluator that reads the chapter's own tables; ADR-0003's record; `G3-GRADING.16` owns the blocks; D48 fixed and D36's third instance removed |
| `G0-CONTRACT.10` | `STITCHCAD-G0-0010 (leaf G0-CONTRACT.10): the dialects are a closed registry, not a format with flags` | six axes, four targets, the seventeen-layer table read against the roadmap, one polyline-only entity set, three grading carriages, the receiver-config record |
| `G0-CONTRACT.11` | `STITCHCAD-G0-0011 (leaf G0-CONTRACT.11): the spike's rule is written before its measurement` | ADR-0002 as a decision structure: chrome, dev shell and TS ban `active`, canvas `proposed`; a declared corpus, seven gates, six rules and an instrument that derives the verdict from a TSV data plane |
| `G0-CONTRACT.12` | `STITCHCAD-G0-0012 (leaf G0-CONTRACT.12): the release contract is the roadmap's §9, compared rather than restated` | nine manifest fields with their sources, digest-bound approval, seven completeness checks, the six states in order, scope that narrows by itself, the policy matrix tuned in a recorded table; D49's trigger discharged |
| `G0-CONTRACT.16` | `STITCHCAD-G0-0016 (leaf G0-CONTRACT.16): one message system, and an inventory nothing keeps by hand` | Fluent at both ends with the licences read; the lint, termbase, pseudolocalization, locale-free canonical files, the RTL geometry test and three review tiers; the 64 ids derived from four chapters and the crate; D49's trigger discharged |
| `G0-CONTRACT.17` | `STITCHCAD-G0-0017 (leaf G0-CONTRACT.17): the command layer is a contract, and the roadmap's own lists prove it` | five classes, seventeen commands, undo at the atomic group, five core-enforced authority levels, a generated parity table; D49's third trigger discharged |
| `G0-CONTRACT.15` | `STITCHCAD-G0-0015 (leaf G0-CONTRACT.15): the gate review is a command, and it reports one clause this repository cannot close` | 18 of 19 clauses met with a check the review ran; G0-12 accepted open by ruling with its cost named; the closure recorded unapproved under governance §6.1; `G2-2D.15` created to own the physical evidence |
| — | gate G0 has no further leaf | the frontier moves to `G1-SLICE.1`; the one open clause travels with the ruling that accepted it |

## Changelog

- `2026-09-30`: `.15` landed and **gate G0 has no further leaf**. The exit review is a command:
  `run_g0_exit_review.sh` parses roadmap §11's own exit bullet into thirteen fragments, requires each to be
  dispositioned by a row of its data plane and each row's key to appear in a fragment, then runs the check
  every clause cites — eleven censuses, `cargo test -p sc-units`, `make wasm`, `mdbook build` — and prints
  `18 met / 1 not met / 19 clauses` in two seconds. The one open clause is evaluation-seat procurement, which
  the director's ruling of `2026-09-30` accepts as open with its cost named: no receiver reads our artifacts
  back, so the interchange claims stay `cited-from-roadmap` and G6 falls to the roadmap's partner-run
  fallback. The gate's **closure is unapproved**, because governance §6.1 withholds approval of a decision's
  evidence from its author and the reviewing party wrote sixteen of the nineteen deliverables; the mitigation
  is that every verdict is a command's exit status, so independence is available to whoever reads next. The
  roadmap's status line is unchanged and that is the finding — it remains DRAFT until the exit criteria are
  met, and one is not. The same ruling moved the residual dependency from a credential to a measurement, so
  `G2-2D.15` was created to own the reference skirt's physical-evaluation protocol, and the frontier moves to
  `G1-SLICE.1`, where the specification meets Rust for the first time since `sc-units`.

- `2026-09-30`: `.17` landed — §4.4's contract exists and its two prose lists are parsed rather than copied.
  Seventeen commands sit in five classes, and a class fixes a command's authority, its reversibility and its
  undo granularity, so an adapter cannot re-classify one for convenience. **Undo is the atomic group** and
  restores semantics — recipe, entity identities, revision — rather than contours, because two designs with
  the same contours and different recipes are different designs; evaluations and artifacts are discarded
  rather than undone, and the history is not canonical content, so a reopened project starts fresh at its
  saved revision. Preview needs only `inspect`, and a commit re-checks the precondition the preview passed,
  because three front-ends and an agent edit one design. Mutations carry a revision precondition and an
  idempotency key, so a stale revision is refused naming both and a transport retry is a reported replay
  rather than a second edit. The five authority levels are permissions on classes enforced in the core — a
  tool manifest is data an adapter ships — and `approve` cannot be held by an agent, which is the release
  chapter's `release_approver_not_human` seen from the other side. The parity table's eight columns, its
  closed cell vocabulary and its generation rule are normative while its rows stay empty, because rows at G0
  would be claims about unwritten adapters. The undo depth is the one number deliberately not written: it is
  declared to exist and to be bounded, and its value belongs with the resource bounds G1 measures.

- `2026-09-30`: `.16` landed — §7.6's "one message system, chosen at G0" is chosen, on evidence rather than
  familiarity. **Fluent, at both ends**: `fluent-rs` in the Rust core and `fluent.js` in the TypeScript
  chrome over one catalogue format, both Apache-2.0 and unarchived, read on `2026-09-30` from the GitHub and
  crates.io APIs and tabulated in the chapter. ICU4X is the rejected alternative and stays a named one — it
  is active, Unicode License V3, and built for resource-constrained clients, which is this product's browser
  profile; it loses on the boundary, because the Rust side would be ICU4X and the browser side the platform's
  Intl, i.e. two dialects, which is the failure the clause exists to prevent. The architecture is normative:
  the message id IS the diagnostic token, arguments are typed and carry units, text is a presentation field;
  the termbase is a projection of the glossary with the ⚠ terms first and no guessed translation; the lint
  has five exemptions each with a reason; canonical files carry no locale and a decimal comma changes nothing
  stored; pseudolocalization runs in CI before any translator exists; RTL mirrors layout and never geometry,
  as a byte-comparison test rather than a sentence; and three review tiers make `strict` and `safety`
  absolute — which, with the domain seat vacant, means no pack can ship today, stated rather than hidden. The
  inventory of 64 ids across 8 families is derived from four chapters and from `crates/sc-units/src/error.rs`,
  and deriving it caught two errors in the chapter's first draft. A ninth glossary part (`localization.md`)
  carries the new vocabulary, and D49's trigger fired on the tree file itself at 91 % of its ceiling, so the
  oldest fourteen changelog entries were sealed under the descriptor contract.

- `2026-09-30`: `.12` landed — roadmap §9 is a chapter and §8.2's example matrix is a contract. A release
  package is immutable, its identity is the digest of its canonical manifest (artifact hashes included), an
  approval binds to that identity and to nothing else, and any change produces a new candidate rather than
  an amendment — with no similarity threshold, because a threshold is a guess about which differences a
  factory cares about made by the party that wants the approval to survive. Completeness is checked against
  the declared construction in seven ways, so a geometrically valid file missing a piece is an invalid
  package. The six acceptance states keep the roadmap's order and each names its evidence and its granter; a
  state may not be claimed without the rung below it and does not survive a new identity. A claim's scope is
  the intersection of its evidence over five axes, narrowing by itself and widening only by a new record.
  Approval is a human act, and a package whose approver is not a human identity is invalid. The policy
  matrix is consulted for the `unknown` state alone, its four dispositions are a closed vocabulary with
  `permit` deliberately absent from its cells, and the tuning of the roadmap's example is recorded in a
  table beside the row it changed — notch geometry in a draft export became a provenance `sidecar` because
  §8.3 forbids substituting a plausible value for an observation. The census parses §9, §8.2 and the
  ontology's state table and compares all three with the chapter in both directions. D49's trigger fired on
  the way (the evidence sibling reached 1000 lines and 96 % of its ceiling) and was discharged in this
  commit by sealing ten completed checklists under the descriptor contract.

- `2026-09-30`: `.11` landed — ADR-0002 is recorded as a **decision structure** rather than a decision: the
  chrome (Tauri + TypeScript/React, Slint as the named fallback, Flutter still rejected), the egui/iced dev
  shell and the TypeScript domain-logic ban are `active`, and canvas hosting is `proposed` because its
  evidence is a G1 measurement that does not exist yet. What exists instead is everything that would
  otherwise be argued afterwards: the three topologies with the hypothesis each tests, a corpus declared
  before anybody measures it (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes,
  1 000 fidelity round trips) with a re-run trigger if a G3 garment exceeds it, seven gates each naming what
  it protects, and a six-rule decision procedure — eligibility, correctness outranking speed, scoring inside
  a declared margin with a total tiebreak order, a one-renderer preference where that is free, escalation
  with a bounded fallback when nothing survives, and a verdict derived by an instrument that a human may
  overrule only by a recorded decision naming the rows. The gates, the applicability table and the rule
  parameters are TSVs the instrument reads, so tightening a threshold is a diff a reviewer sees, and
  `results.tsv` is empty on purpose: the honest state at G0 is `PENDING`. `G1-SLICE.13`'s acceptance now
  consumes the instrument by path.

- `2026-09-30`: `.10` landed — ADR-0004 is complete: interchange is specified as **dialects over a
  format**, not as one format with flags. An export target is a named tuple over six declared axes, each
  with the party that resolves it; the registry is closed at four targets, so a tuple nobody validated is
  refused naming the nearest one; the seventeen-layer table carries both naming modes and this project's
  object mapping, with the named mode's loss of separation declared rather than discovered by a partner;
  cut-as-1 against sew-as-1 is a profile mapping recorded in three places and never a writer default; one
  BLOCK per piece with SST and PST mandatory on the ASTM path and their field content deliberately
  unspecified, because a receiver owns it and G6 is the only oracle; one polyline-only entity set serves
  both releases, arcs travelling exactly as bulges and Béziers tessellating at T2's chordal bound; three
  grading carriages, each its own artifact and validation; and a receiver-config record that makes a
  dispute a comparison of fields. D6673-10's withdrawal is recorded with the convention implemented as
  de-facto and no conformance claim anywhere. The chapter's closure is derived against `ROADMAP.md` itself
  by a new census with a 13-arm suite, including an arm that grows the roadmap's convention in a copy and
  requires the refusal. Five glossary terms were added and `16` entries repointed from this leaf to a
  clause, and the two probe-arm defects found on the way are recorded in the checklist rather than fixed
  silently.

- `2026-09-30`: `.9` landed — ADR-0003 is complete, in both halves. The formula language v1 is normative in
  three parts (contract, grammar, examples): eight kinds with no implicit conversion, nine name origins whose
  missing value is always a named diagnostic rather than a default, declaration-order single-pass evaluation
  with no representable cycle, exact rational arithmetic that rounds at exactly two points, twelve
  diagnostics split static from runtime, four structural limits, the display↔machine bijection and an
  S-expression canonical form that is a formula's identity. Every number it publishes is computed: the census
  carries a reference evaluator that reads the chapter's OWN tables and type-checks with them, so 17 bindings,
  4 assertions and 13 refusals are executed on every run, twelve of the bound names are cross-checked against
  the fixture chapter, and a 15-arm probe suite (with a control arm) proves it notices. The other half is the
  named drafting system — Aldrich's metric pattern cutting, chosen against four candidates with what was read
  on this machine recorded per candidate — adopted as a method and never as a text, `unverified-with-owner`
  until procured, with `G3-GRADING.16` created to ship the blocks and the licence question flagged to the
  project owner's seat rather than answered here. The slice also fixed D48 (a duplicate acceptance-checklist
  heading and a stale children range in `G3-GRADING.md`), removed D36's third instance in `LIVE_STATUS.md`,
  and rolled both ledgers over in the commit whose append crossed them.

Older entries — `.14c` and `.19`, `.4c`, `.14` and `.4b`, and below them everything back to the tree's
creation — are sealed in
[`docs/history/stitchcad-g0-contract-changelog-part2.md`](../history/stitchcad-g0-contract-changelog-part2.md)
(48 lines, 4651 bytes, `sha256:00682c0c…`) and, for the oldest of them,
[`…changelog-part1.md`](../history/stitchcad-g0-contract-changelog-part1.md), under the remedy defect D49
names for a tree file's ledger tail.
