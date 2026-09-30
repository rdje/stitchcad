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
| 15c | `G0-CONTRACT.14c` | `done` | the case this project runs in is governed: a delegated decision records that its author applied it, its consequences become instruments, and its author never approves its evidence |
| 15d | `G0-CONTRACT.19` | `done` | what the project does not know is one command away: the uncertainty census enumerates every marker with its resolving authority and refuses an unowned one |
| 15b | `G0-CONTRACT.4c` | `done` | the director delegated the finding outright, so `.4b`'s proposal is ruled approved and applied: roadmap **v0.3** carries the envelope-coverage criterion, the four cells are committed gates, and `G3-GRADING.5`/`.15` own the garments |
| 16 | `G0-CONTRACT.9` | `done` | ADR-0003 landed in full: the formula language v1 as three censused parts (contract, grammar, examples), a reference evaluator that reads the chapter's own tables, Aldrich's metric pattern cutting named as the reference drafting system, and `G3-GRADING.16` created to ship its blocks |
| 17 | `G0-CONTRACT.10` | `done` | ADR-0004 landed: six axes, a closed registry of four targets, the seventeen-layer table in both naming modes read against the roadmap itself, one polyline-only entity set with arcs exact as bulges, three grading carriages, and the receiver-config record |
| 18 | `G0-CONTRACT.11` | `done` | ADR-0002 recorded as a decision structure: the chrome, the dev shell and the TypeScript ban are `active`, canvas hosting is `proposed`, and the corpus, the seven gates and the six-rule verdict are written down before anybody measures — with an instrument that derives the outcome from a TSV data plane |
| 19 | `G0-CONTRACT.12` | `done` | the release contract landed: nine manifest fields each with its source, identity as the manifest's digest, completeness against the declared construction, the six acceptance states in the roadmap's order with the evidence each needs, scope that narrows by itself, human-only approval, and §8.2's policy matrix tuned and recorded |
| 20 | `G0-CONTRACT.16` | `pending` | **next** — the ONE message system (Fluent or ICU, not both) plus the externalization architecture: the CI lint on an inline user-facing string, the termbase per language with the safety terms first, pseudolocalization, and locale-independent canonical files |
| 21 | `G0-CONTRACT.17`, `.15` | `pending` | the command-layer contract with undo/redo granularity, then the `.15` exit review |

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

### `G0-CONTRACT.12` — the release contract is the roadmap's §9, compared rather than restated

- [x] **REPRODUCE / ISSUE** — roadmap §9 was the last G0 clause with a promised chapter and no chapter.
  `git ls-files 'docs/book/src/spec/release*' | wc -l` → `0`; the spec index carried the promise as an
  unlinked row, `git show HEAD:docs/book/src/spec/index.md | grep -c '^| Release and approval |'` → `1`,
  `rc=0`; and the vocabulary was parked against this leaf,
  `git show HEAD:docs/book/src/spec/glossary/profiles-and-release.md | grep -c 'G0-CONTRACT\.12'` → `13`
  (twelve entries plus the part's own header note saying "until that chapter lands, the roadmap clause is
  cited").
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n '^## 9. Signoff' ROADMAP.md` → `626`, `rc=0`, and the exit
  clause that makes it a G0 deliverable: `grep -n 'approval states & release contract' ROADMAP.md` →
  `670:  decided; approval states & release contract (§9) specified; ADR-0001`, `rc=0`. §9 is a list — nine
  manifest fields, six acceptance states, three artifact classes — and a list realised in prose drifts: a
  field is renamed, a rung is merged, a class is forgotten, and every gate stays green because nothing
  compares the two documents. So the cause is not only an unwritten chapter but the absence of an
  instrument that would notice the difference.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh`
  → `release-contract census: 9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`, `exit=0`, where
  the `9` and the `6` are parsed out of roadmap §9 (parenthesis-aware, because one field carries a nested
  comma) and the `8 × 3` matrix is compared against §8.2's header and example rows and against the five
  states ontology §5 declares. Its discrimination is proved:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/release_contract/run_release_contract_probes.sh` →
  `probes: 14 pass / 0 fail`, including LADDER-ORDER (two rungs swapped, both orders printed),
  FIELD-INVENTED (a field wearing the roadmap's authority without its wording), DISPOSITION-UNUSED (a
  vocabulary word no cell carries) and CONTROL. The chapter is `257` lines / `19 010` B, widest line
  `256` B, inside the `book_collection` per-part health of `400` / `24 576` / `275`; `make book` →
  `INFO HTML book written to …`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `19 suite(s) green`; the eight neighbouring censuses unchanged: glossary `298 terms / 8 parts /
  156 tokens / 0 failure(s)` after four new terms, twelve repointed entries and a re-derived index,
  interchange `17 layers / 4 targets / 12 entities / 0 failure(s)`, formula language `17 bindings /
  4 assertions / 13 refusals / 0 mismatch(es)`, fixture `20 derived rows / 4 closure checks / 5 pieces /
  0 mismatch(es)`, matrix `105 rows / 0 failure(s)`, standards `6 registered / 0 failure(s)`, uncertainty
  `127 markers / 15 files / 0 unowned`, coverage `10 lanes / 13 trees / 0 unowned`; `bash
  scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 102 files measured`, `exit=0`. Two probe
  arms failed first against a correct census and were fixed in the arms, not the tool: one quoted a
  sentence as it read in a draft rather than as it wraps in the file, and one replaced every occurrence of
  a disposition token — deleting its declaration along with its uses, which is the "remove the property,
  not one instance" trap inverted into "remove the rule too".
- [x] **FIX** — wrote the chapter (nine manifest fields each with its source, identity as the manifest's
  digest with no in-place amendment, seven completeness checks against the declared construction, the six
  states in the roadmap's order with the evidence and granter each needs, five scope axes with the
  intersection rule, human-only approval, §8.2's matrix tuned in a recorded table with a closed
  four-word disposition vocabulary and the dependency-closure rule, eight diagnostics); the decision record
  with six rejected alternatives; the census and its 14-arm probe suite; four glossary terms plus twelve
  repointed entries and a corrected `spi` routing (it pointed at this leaf and belongs to `G5-SHELLS.13`).
  **D49's trigger fired and was discharged here rather than deferred:** the evidence sibling had reached
  `1000` lines / `94 086` B against a `98 304`-byte ceiling, so ten completed checklists
  (`G0-CONTRACT.2` … `.4b`) were sealed into `docs/history/stitchcad-g0-contract-evidence-part1.md`
  (`560` lines / `52 573` B, digest reproduced by `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`)
  and the live sibling fell to `449` / `42 065`. The seal then exposed **D52**: three completed leaves'
  ROOT CAUSE boxes (`.14`, `.14b`, `.19`) asserted their evidence with no invocation, which the acceptance
  gate refused once the seal changed which box comes first — all three now carry the command and its real
  output, `.19`'s directory count is corrected from twelve to the `13` the command prints, and the sibling's
  header states the rule (re-run the enforcer after moving checklists, before committing).
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two instrument
  rows), `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in
  this commit. Lesson promotion: **promoted** — the new record carries an `answers:` line.

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
| `2026-09-30` | `G0-CONTRACT.12` | the release-contract census and its 14-arm probe suite; the chapter's size and widest line; the glossary census after four terms and twelve repointed entries; `make gate`/`probes`/`book`; the eight neighbouring censuses; containment; the sibling's sealing and its digest | `9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`; `14 pass / 0 fail`; `257` lines / `19 010` B / widest `256`; `298 terms / 0 failure(s)`; `19 suite(s) green`; `449` / `42 065` after sealing `560` / `52 573` |
| `2026-09-30` | `G0-CONTRACT.11` | the spike verdict instrument on its empty data plane and on twelve synthetic result sets; the record's size; `make gate`/`probes`/`book`; the seven book censuses; containment | `PENDING`, `exit=0`; `13 pass / 0 fail`; `78` lines / `7 244` B; `18 suite(s) green`; all green; `OK — 99 files measured` |
| `2026-09-30` | `G0-CONTRACT.10` | the interchange census and its 13-arm probe suite; the chapter's size and widest line; the glossary census after five terms and 16 repointed entries; `make book`/`gate`/`probes`; the four neighbouring censuses; containment | `17 layers / 4 targets / 12 entities / 0 failure(s)`; `13 pass / 0 fail`; `303` lines / `21 650` B / widest `227`; `294 terms / 0 failure(s)`; `17 suite(s) green`; detail in the checklist |
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
| `G0-CONTRACT.15`–`.17` | `pending` | — |

## Changelog

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

- `2026-09-30`: `.14c` and `.19` landed, acting on the director's instruction to decide and act on the session's
  three findings. `.14c` governs self-application under delegation — governance §6.1's five rules (record the
  author and the applier; the author never approves the evidence their decision requires; consequences become
  instruments; the record states its reversal; a delegation does not upgrade evidence), the roadmap's v0.3
  disposition entry now discloses that one party proposed and applied it, and `G3-GRADING.14` may not be signed
  by the criterion's author. `.19` makes the vacancy's consequences enumerable: the uncertainty census lists
  every marker the book carries with the authority that resolves it and refuses a blocking marker whose status
  section names nobody, with six probe arms — and building it found two defects in existing instruments, the
  glossary's link resolver (one `gsub` could not normalise `../../`) and this census's own first authority list
  (a bare gate id counted as an owner, which made the rule nearly unfailable).

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
