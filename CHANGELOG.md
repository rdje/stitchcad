# CHANGELOG.md

Newest first: one section per completed slice, in commit order. Older slices live in sealed, immutable
segments under `docs/history/`, each named below with its identity and retrieval path.

# Sealed archive — earlier slices

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`part1.md`](docs/history/stitchcad-changelog-part1.md) | slices 1–15, `STITCHCAD-PLANNING-0001` … `STITCHCAD-SPINE-0004b` | 365 lines, 30452 bytes, `sha256:f4aec75a…` |
| [`part2.md`](docs/history/stitchcad-changelog-part2.md) | slices 16–20, `STITCHCAD-SPINE-0014` … `STITCHCAD-G0-0002` | 152 lines, 12811 bytes, `sha256:5783ac36…` |
| [`part3.md`](docs/history/stitchcad-changelog-part3.md) | slices 21–24, `STITCHCAD-G0-0013` … `STITCHCAD-G0-0018` | 147 lines, 12289 bytes, `sha256:14ad5278…` |
| [`part4.md`](docs/history/stitchcad-changelog-part4.md) | slices 25–29, `STITCHCAD-G0-0004` … `STITCHCAD-SPINE-0017` | 177 lines, 15440 bytes, `sha256:a8cc1de6…` |
| [`part5.md`](docs/history/stitchcad-changelog-part5.md) | slices 30–31, `STITCHCAD-G0-0005` … `STITCHCAD-G0-0013c` | 82 lines, 7505 bytes, `sha256:18548ff7…` |
| [`part6.md`](docs/history/stitchcad-changelog-part6.md) | slices 32–33, `STITCHCAD-G0-0007` … `STITCHCAD-G0-0006` | 93 lines, 8284 bytes, `sha256:3148dd0f…` |

**Correction (D30).** part1's own descriptor says its coverage runs "through `STITCHCAD-SPINE-0004c`".
It does not: part1's newest entry is `STITCHCAD-SPINE-0004b`, and `SPINE-0004c` is sealed in part2.
Sealed segments are immutable, so the correction is recorded here and in part2's descriptor rather than
by editing part1.

The bedrock scaffold's own changelog — the provenance of this repository's discipline spine — is sealed
in [`docs/history/bedrock-scaffold-changelog.md`](docs/history/bedrock-scaffold-changelog.md).

The live window below holds the most recent slices. When it passes its health target (400 lines /
32 768 bytes) again, the oldest entries are sealed the same way, and
`bash docs/tasks/artifacts/changelog/run_changelog_ledger_probes.sh` proves the order, the uniqueness and
the digests afterwards.

## STITCHCAD-G0-0004c - the envelope criterion is law, not a proposal (leaves `G0-CONTRACT.4c`, `SPINE.4.5`, `PLANNING.6`)

The director delegated the three findings outright, so the amendment `.4b` had prepared as a *proposal* is ruled
approved and applied - through the roadmap's own machinery, and with two mechanisms the application exposed as
missing.

- **roadmap v0.3**: §11 G3 gains an *envelope coverage* exit criterion - every garment §3.2 names drafts,
  grades and exports at that gate or an earlier one; the A-line skirt at G2, the bodice and sleeve at G3, a
  classic collar with stand, fall and roll line, trousers carrying at least one pocket and one closure whose
  buttonhole length is derived from its button - and "a garment the envelope names and no exit criterion proves
  is a gate failure, not a scope note". The old note that an intermediate "may be inserted without shame" is
  rewritten so an intermediate can never substitute for a criterion. Marked where the roadmap's revision policy
  requires: the title, the status block, an Appendix A disposition entry naming its source and the defect it
  closes, and the end line. `947` lines / `52 818` B, +28 / +1 997
- **the criterion arrives with owners**, because a roadmap clause no leaf owns is D32 one level up:
  `G3-GRADING.5` became required (trousers + pocket + derived buttonhole, with a mutation test that editing the
  button changes the hole), `G3-GRADING.15` was created (the collar, its roll line never exported as a cut
  line), and `.14`'s exit review now fails if a §3.2 garment has no leaf's evidence. The four matrix cells
  dropped `(proposed)` -> census `105 rows / 29 diagnostics / 0 failure(s)` with `D32 rows: 0` and
  `proposed cells: 0`; the A3 advisory and the `(proposed)` mechanism stay for the next one, and its probe arm
  was rebuilt to pin A3 in BOTH directions (0 real, 1 injected) instead of asserting a count that no longer
  exists
- **SPINE.4.5, the mechanism the application needed**: the roadmap's transition-debt baseline was measured at
  exactly the file's size, so v0.3 could only land by hand-widening a number the doctrine forbids widening. The
  debt column now accepts `at=<revision>` and the checker REFUSES a baseline whose revision the file's first
  line no longer declares - a revision may re-base its baseline, but only in the commit that revises, which is
  where the authority has to be anyway. Declaring the right revision does not license growth past the baseline.
  This is the containment adoption note's deferred trigger 3 (a stored copy of a mechanically owned value needs
  an executed freshness oracle) fired and discharged locally: six lines in the existing awk evaluator, not the
  2 100-line neutral interpreter. `--self-test` `11` -> `15` arms; the probe suite gained `REAL-3`, which stales
  the real registry to `at=v0.2` and requires the refusal -> `probes: 5 pass / 0 fail`
- **PLANNING.6 / D44, found because a census was finally run**: the tree-coverage census defined a tree by
  FILENAME, so the evidence siblings the containment registry prescribes were reported as lane-less orphans -
  `15 trees / 1 unowned / 4 orphan(s)`, red for two committed slices, because `make probes` globs
  `run_*probe*.sh` and nothing else ran it. A tree is now recognised structurally by its `- Tree ID:` line in
  both populations AND in the advisory loop (three copies of one assumption; the advisory one hid `G0-CONTRACT`
  itself behind its sibling and no exit code could show it), a non-tree file must be linked from a tree or it is
  a stray, and `run_tree_coverage_probes.sh` puts the census under `make probes` with seven arms ->
  `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `probes: 7 pass / 0 fail`
- **G0-CONTRACT.14b, the second finding**: no human is named and none is invented. Governance §8 now states
  which seats are held ACTING by the director (project owner, procurement - authority he already holds, so the
  arrangement adds a record and no power) and which is openly VACANT (the sewing/factory domain expert, because
  competence cannot be acted). An acting holder may not confirm the fixture's `assumed` constants, sign a
  golden's semantic half, rule a safety term, or approve a byte-changing profile - so the first G2 golden stays
  gated on a real name, and §8.2 writes the ask per seat so naming one is a single act. The fixture and
  standards chapters now cite the vacant seat instead of a leaf that cannot name anyone
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `13 suite(s) green`; `make book` ->
  exit=0; glossary `276 terms`, standards `6 registered`, fixture `20 rows / 4 checks / 5 pieces`, ledger
  `9 pass / 0 fail`, all 0 failures; containment OK with the roadmap re-based under its record

## STITCHCAD-SPINE-0004d - the widest-line target is derived from the cell budget (leaf `SPINE.4.4`)

The containment check printed a warning on every run that no defect stood behind: `book_collection: widest
line 272 B = 136% of its 200 B target`, where the 272-byte line is a five-column termbase row and the 200-byte
target was derived from the shape of a prose chapter. The director's ruling of 2026-09-30 made re-deriving it
the fourth of its four items.

- **the target is now a cell budget, measured**: the binding shape is the glossary termbase
  (`Term｜What it means｜Canonical object｜Also called｜Machine token`, 276 data rows, 5 columns), whose
  per-column p95 cells sum to 259 B, plus the GFM separator overhead `3 x columns + 1` = 16 B -> **275 B**.
  Not a round number, and not today's widest line: the registry forbids deriving health from the largest file,
  and for a maximum axis that prohibition bites hardest, because the widest line is one row's accident while a
  per-column budget is what each column must carry
- **the producer is tracked**: `docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` walks every
  table in a population, reports each shape's per-column widest and p95 cells and both derived budgets, and
  REFUSES (exit 1) when its recommendation does not cover the population's widest actual line ->
  `cell budget: 36 shapes / 625 data rows measured / recommended maxline health 275 B`, exit=0. Its
  `--self-test` pins the arithmetic on a synthetic table whose budget is known by construction ->
  `probes: 7 pass / 0 fail`
- **the ceiling rises by record, from 320 to 440 B**: the shape's own worst legitimate row (every column at
  its widest at once) is 379 B, which the old ceiling would have refused, and 440 = 1.6 x health is the ratio
  every other row in the registry uses. Authorised by
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`, which also records the two
  rejected alternatives with their arithmetic - raising health to 379 to silence the warning (then the
  denominator is the worst case, and a prose chapter could carry a 500-byte line), and splitting the
  collection into prose and table rows (measured: `PARTOF[$path] = sid` means the LAST matching row wins, and
  one registry field holds one glob, so a split needs `:(glob)` pathspec magic and three rows; deferred with
  the trigger that would reopen it)
- **the warning survives, and now means something**: 272 B is 99% of 275 B. No health target at or below the
  old 320 B ceiling could have silenced a 272-byte row - it would need more than 340 B - which is a structural
  property of a maximum axis, recorded so nobody re-derives it as a surprise. The remedy that would clear it
  is a table-authoring convention, which is `SPINE.15`'s beside defect D22, not a bigger number
- **D42 fixed on the way**: SPINE's 17 completed checklists moved byte-identically into
  `docs/tasks/SPINE-evidence.md`, taking the tree from 1096 lines / 88341 B to 552 / 42028 - the convention
  `G0-CONTRACT.4b` recorded, performed by the leaf D42 named
- gates: `check_live_doc_size.sh` -> `OK - 17 surfaces, 15 routes, 80 files measured`, exit=0, and its
  `--self-test` -> `11 arms, 0 failed`; `run_live_doc_size_probes.sh` -> `probes: 4 pass / 0 fail`;
  `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`. `.doctrine/` changed, so
  the immediate-push exception fires: this slice pushes at once and records the observed CI verdict in the
  leaf

## STITCHCAD-G0-0014 - the governance model, and the three empty seats (leaf `G0-CONTRACT.14`)

Roadmap §11's G0 exit requires a governance model drafted with the project owner named and the
sewist-vs-programmer review paths defined; §14's mitigation for "community fork over governance" is "governance
doc at G0, while the room is empty". `git ls-tree --name-only HEAD docs/book/src/` -> `SUMMARY.md`,
`introduction.md`, `spec`: the doc did not exist. It does now, in full, and the only part still missing is the
part that is not this repository's to supply.

- **two review paths, and every change class lands on exactly one**: code review judges whether the
  implementation does what the spec says; domain review judges whether the artifact is right for a cutting room
  (roadmap §12: "domain review is not code review"). A classification table assigns eight change classes, so a
  change goes to whoever is competent rather than to whoever answers first
- **the two-step rule is conjunctive**: anything that alters exported bytes needs the domain expert AND the
  maintainer, and the tooling reports which half is missing rather than "partially approved" - a state a
  receiver can misread is worse than a refusal
- **a role is the decision it may make, not the person holding it**: seven roles, each with the authority it
  needs and whether an agent may hold it. Approval is human-only and never manufacturable by a graph mutation
  (§7.8); the G7 reviewer gets a checkable independence criterion - not an author of the code, the spec or the
  evidence under review
- **conflict resolution in four steps**: classify the question (most conflicts are two correct answers to
  different questions) -> make a contested default a Factory Profile parameter with the dissent recorded,
  because the model already carries both readings -> escalate by review round, not by date, since this project
  has no calendar -> treat a fork as a legitimate outcome, and make the rulings carry their evidence, which is
  the only thing a fork cannot copy
- **goldens are a release-contract event**: two signatures (mechanical + semantic), stale-ification per §9, and
  a hard precondition - no golden over an `assumed` constant, which means the unnamed domain expert gates the
  FIRST G2 golden. The dependency is written where a plan will hit it
- **procurement fallbacks state their cost**: evaluation seats, the physical plotter, the standards texts and
  the pilot partner each carry a fallback and what it costs in evidence quality (a partner run is layer-4
  evidence with recorded product, version and settings - slower, fewer targets), because an unstated cost is
  how a slip becomes a silent downgrade of the release claim
- **the three empty seats are in one table** (project owner, domain expert, procurement owner) plus the G7
  reviewer's independence rule, with what waits for each. Until a name exists the state is the ontology's
  `unknown`: not silently defaulted, and blocking what it governs. `.15` records the clause as `met - model
  drafted, named owner pending` or `not met`, never as met on the chapter's strength alone
- six of the chapter's rules are project decisions rather than citations, and are recorded as such in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`; the chapter's own §10 says
  which clauses are which, so a reader never mistakes an invention for a requirement
- the slice also obeys the evidence-split convention `.4b` recorded, moving `.4b`'s checklist into
  `docs/tasks/G0-CONTRACT-evidence.md` (10 completed checklists) and leaving the tree at 744 lines / 64445 B
- **the rollover this append triggered is performed in the same commit**: the live window had crossed its
  health target (410 lines / 36632 B against 400 / 32768), so slices 30-31 are sealed into
  `docs/history/stitchcad-changelog-part5.md` (82 lines / 7505 B / `sha256:18548ff7…`), proved byte-identical
  to `git show HEAD:CHANGELOG.md` rather than to memory, and the window is back inside health at 329 / 29295.
  `run_changelog_ledger_probes.sh` -> `probes: 8 pass / 0 fail`, its DESCRIPTOR rule reproducing all seven
  sealed segments
- gates: `make gate` -> `=== all doctrines green ===`; `make book` -> exit=0 with `governance.html` rendered as
  the book's first non-specification part; `make probes` -> `12 suite(s) green`; glossary `276 terms`, matrix
  `105 rows`, standards `6 registered`, fixture `20 rows / 4 checks / 5 pieces` - all `0 failure(s)` /
  `0 mismatch(es)`; containment OK, 79 files measured; no `.rs` or `.sh` staged

## STITCHCAD-G0-0004b - every envelope feature has a gate that proves it (leaf `G0-CONTRACT.4b`)

Five rows of the supported-envelope matrix said `unnamed (D32)`: roadmap §3.2 promises a classic collar and
trousers, ontology §4.7 models buttons and pockets, and no gate's exit criteria proved any of them - while
the census stayed green, because the gap was an advisory. The director ruled on 2026-09-30 that the engineer
decides it, and reserved one thing: `ROADMAP.md` is his to amend.

- **the assignment**: collar, trousers, button/buttonhole and pocket -> **G3**; buttons also -> **G5**, whose
  exit already requires a tech pack with a notions list; fly construction -> **G7**, whose exit already
  requires a supported-envelope statement *with named limitations*, so the deferred row needs no amendment
  and is treated exactly like the lining row
- **the reasoning is one rule**: permission is not a criterion. G3's note that an intermediate "may be
  inserted without shame" schedules nothing, so the proposal adds ONE exit criterion over §3.2's whole
  garment list - closing the class rather than the four instances - and names the failure mode: a garment the
  envelope names and no exit criterion proves is a gate failure, not a scope note
- **a proposal is labelled as one**: the four cells that depend on the amendment say `(proposed)`, the exact
  text is quoted current-vs-proposed with its line numbers (`ROADMAP.md:701`-`711`) in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`, and the record states what
  happens on approval and on rejection. `git diff --name-only HEAD -- ROADMAP.md | grep -c .` -> `0`: the
  roadmap is untouched. A provisional commitment that reads like a settled one is the same gap in better
  clothes, so the census gained an **A3** advisory printing all four cells on every run, with a probe arm
  that removes the markers and requires the count to fall
- **derived, not asserted**: `run_feature_matrix_census.sh` -> `105 rows / 29 diagnostics / 0 failure(s)`
  with `D32 rows: 0` (was `5`, re-measured against a synthetic root built from `HEAD`) and `proposed cells:
  4`; `run_feature_matrix_probes.sh` -> `probes: 12 pass / 0 fail`
- **D41, found by re-deriving the evidence for D32**: the entry cited `0` for `sed -n '/### G3 /,/### G4 /p'
  ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`, and that command yields `1` - the range includes
  G3's complexity note, which says "a shirt/trousers intermediate". The conclusion was right (over the exit
  bullet alone, `sed -n '702,708p'`, the count is `0`) and the citation was wrong, in three live places and
  one sealed segment. All three live citations are corrected with the reason; the sealed `part4` line is
  immutable, so this entry is its superseding record. The trap generalises: a section range silently includes
  that section's notes, and `G0-CONTRACT.15` will re-derive every gate clause the same way
- **the dev-notes rollover this slice's own append triggered**: the four oldest lessons are sealed into
  `docs/history/stitchcad-devnotes-part1.md` (`66` lines / `5 589` B / `sha256:d3b94e9a…`), proved lossless
  against `git show HEAD:DEV_NOTES.md` rather than against memory, and the live window is back inside its
  health at `150` lines / `13 183` B. The ledger probe's `DESCRIPTOR` rule now runs over EVERY
  `docs/history/*.md` segment (REAL: `11` -> `13` verdicts) with a `DEVNOTES-DIGEST` arm pinning it ->
  `probes: 8 pass / 0 fail`. Its Coverage and pointer legs stay changelog-only, which is **D40**, owned by a
  new leaf `SPINE.19`, rather than left implicit in a green run
- **the token census fired a fifth time at authoring time**: `` `lining` `` and `` `proposed` `` wore token
  formatting in prose; both were de-tokenized, not exempted -> `276 terms / 8 parts / 145 tokens /
  0 failure(s)`
- **the containment ceiling refused this slice, and the registry's own remedy was the fix**: appending the
  `.4b` checklist put `docs/tasks/G0-CONTRACT.md` at `1182` lines / `104 182` B against a `tasks_collection`
  per-part byte ceiling of `98 304`, so `make gate` blocked with `104182 bytes exceed the byte ceiling`.
  A ceiling rises only by a recorded authority and never to land content, so the nine completed-leaf
  checklists moved to a new sibling `docs/tasks/G0-CONTRACT-evidence.md` (`494` lines / `45 505` B) and the
  tree file is back inside its health at `729` / `62 087`. The convention is recorded in the tree's decisions
  with the gate that forces it: `check_task_acceptance.sh` judges EVERY staged `docs/tasks/*.md`, so the leaf
  being landed keeps its checklist in the tree file - which also means that file's first matching box is now
  always the current leaf's, closing D15's facet 1 by structure instead of by care. `SPINE.md` is past the
  same threshold and owes the same split: **D42**, owned by `SPINE.4.4`
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`; `make book` ->
  exit=0; standards census -> `6 registered / 6 designations used / 0 failure(s)`; containment OK with the
  matrix's two over-health axes recorded in the leaf

## STITCHCAD-G0-0013d - one waistband, and the instrument that keeps it one (leaf `G0-CONTRACT.13d`)

The reference fixture described two different garments at once for nine commits: §4 published the cut width
of ONE band folded lengthwise while §6 listed a faced two-piece band and §8 sewed only the outer one
(defect D27). The director ruled on 2026-09-30 that the engineer decides it; this slice decides it, and
makes the agreement between the chapter's tables mechanical rather than a matter of reading.

- **the decision**: one straight band, cut once, folded lengthwise at its midpoint (5.0 cm from either long
  edge), plus one interfacing piece cut at the band's finished dimensions (77.0 x 4.0 cm, no allowance on
  any edge) and fused inside the seam lines. Five pieces, not six - `waistband_outer` and `waistband_inner`
  no longer exist anywhere in the book
- **sourced, not preferred**: five references read on this machine on 2026-09-30, each cited by URL and date
  in `docs/decisions/decision_reference-fixture-waistband-straight-folded.md` under a `read-external` label
  that upgrades no claim about a standard. The decisive fact is that the drafting literature gives the
  straight band ONE rectangle with a fold line and reserves "cut two pieces - one for the outer waistband,
  and one for the inner waistband" for the **contoured** band, while this fixture's waist sits at the natural
  waist where a straight band belongs. The sources disagree about band height (2-5 cm against a 3 cm maximum
  for a straight band), so `wb_width` = 4.0 cm stays `assumed` with the conflict recorded rather than quietly
  resolved in favour of the convenient source
- **every piece is accounted for**: §8 gains an attachment account - a span, or a declared non-sewn method
  from a closed list that holds `fused` alone. "Every piece has a span" is false for a fused interfacing,
  and believing it is what let an inner band nobody sewed pass for nine commits
- **the numbers are derived, not read**: §4 gains `waistband_fold_position` 5.0, `waistband_finished_length`
  77.0, `wb_interfacing_width` 4.0, `wb_interfacing_length` 77.0 and the band's two closure checks
  (`waistband_pattern_length - 2 x sa_cb - wb_extension = garment_waist` -> `74.0 = 74.0`, and
  `waistband_cut_width - sa_waist - sa_wb_bottom = 2 x wb_width` -> `8.0 = 8.0`), so §4.1 carries four
  oracles: two for the body, two for the band
- **a tracked producer at last**: `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`
  evaluates every §4 formula over the tables above it, checks the four closures, the piece account, §12's
  published count against §6's list, and that §4's band width describes the same construction as §6's band
  pieces (rule `B1`). Over the chapter as committed: `16 derived rows / 2 closure checks / 6 pieces /
  7 mismatch(es)`, exit=1, naming D27 by shape. Over the chapter now: `20 / 4 / 5 / 0 mismatch(es)`, exit=0.
  Until this slice the arithmetic that found D33 and then D27 lived as `python3 -c` strings inside task
  leaves - re-runnable by nobody, which is the leg-3 breach this repository already committed once as D20
- **the probes are shown to discriminate, not merely to pass**: nine arms (one GREEN, eight RED) ->
  `probes: 9 pass / 0 fail`, and neutering one rule at a time in a scratch copy reddens exactly the arms
  that depend on it (`B1` removed -> `8 pass / 1 fail`; `P2` removed -> `7 pass / 2 fail`)
- **the token census corrected the chapter a fourth time at authoring time**: four undeclared tokens on the
  first draft - `Fold` (a column name of this chapter's own table), `fused` (a machine enum value a program
  reads, so it became a glossary term with one owner), and the two names of the rejected reading. The fix was
  the convention rather than an exemption: prose lost its backticks and the dead piece names survive in the
  layer-C record for anyone tracing the defect -> `276 terms / 8 parts / 145 tokens / 0 failure(s)`
- **not decided, deliberately**: whether a `SeamSpan` may name the same piece on both sides - a folded band's
  short ends are stitched across, which joins one piece to itself. Logged as D35 with `G1-SLICE.3` as owner;
  §8 records the ends as an edge finish instead of settling a type invariant by accident
- **the ledger probe refused an honest changelog, and the probe was the thing that was wrong** (D39): its
  work-unit id shape was `[0-9]+[a-c]?`, so this slice's own id `STITCHCAD-G0-0013d` parsed as
  `STITCHCAD-G0-0013` and collided with the sealed entry of that name -> `NO-DUP FAIL both live and sealed:
  STITCHCAD-G0-0013`, a duplicate that did not exist, plus an `ORDER FAIL` comparing the wrong commit. The
  same collapse hides a real mis-ordering between `-0013d` and `-0013e`, so one regex produced a false red
  AND a possible false green. Fixed to `[a-z]?` in all six places and pinned by a new GREEN arm `SUFFIX`,
  shown sensitive by reverting the class in a scratch copy -> `probes: 5 pass / 2 fail`; after the fix
  `probes: 7 pass / 0 fail`
- **also in this commit**: D36 logged and fixed (`LIVE_STATUS.md`'s spine row reported 13 universal gates and
  7 probe suites where `make gate` prints 12 universal plus the project slot's 2 and `make probes` finds 12);
  D37 logged and fixed (`DEV_NOTES.md` described itself below every lesson, at line 149 of 156); D38 logged
  (the census's open/closed counts are prose, so a naive derivation reports 33 closed and misses D34 —
  `PLANNING.5`'s goal is extended to derive them); D34's recurrence recorded - the commit that created `.13d`
  and `.4b` moved the frontier without writing `docs/TASK_TREE.md`, so the index named a leaf four commits
  done, and all three stale rows are corrected
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`; `make book` ->
  exit=0; feature-matrix census -> `105 rows / 29 diagnostics / 0 failure(s)`; standards census ->
  `6 registered / 6 designations used / 0 failure(s)`; containment OK with the chapter and the record both
  over health and inside every ceiling, recorded in the leaf; no product code touched

## STITCHCAD-G0-0008 - ADR-0001: licence and solver are one decision (leaf `G0-CONTRACT.8`)

`docs/decisions/decision_adr-0001-license-and-solver.md` settles roadmap §5's ADR-0001 **before any solver
code exists**, which is when §5 says it must be settled: an external contributor arriving at G1 inherits a
decision, not a default.

- **the decision**: the core is dual `MIT OR Apache-2.0` (re-read from `Cargo.toml`, not recalled:
  `grep -n license Cargo.toml` -> `license = "MIT OR Apache-2.0"`), so the SolveSpace solver `slvs`
  (GPLv3) is **rejected**, and `sc-sketch`'s optional local constraints get a custom kernel or a numeric
  least-squares solver written for this repository. Licence and solver are one record because neither is
  decidable alone - the solver choice IS the licence choice
- **the exchange rate is stated, not asserted**: linking `slvs` copylefts the workspace, which decides who
  may embed the product - and the persona roadmap §1.2 names is a patternmaker producing a package for a
  *named factory*, with factories and partners embedding the tool. What `slvs` would buy is an *optional
  annotation layer* (ADR-0003 makes the recipe primary and sketch constraints local), while the production
  solver is custom anyway (§6.1's deterministic finite-domain CP engine, so native and WASM share one
  determinism argument). Trading the licence of the whole product for the second solver of an optional
  feature is the bad exchange this record refuses
- **BSL-1.1 is kept out of the permissive category**, as §5 demands: a use restriction is a commercial term
  needing the project owner's own review, and a downstream embedder cannot treat BSL code as MIT. Adopting
  it would be a new ADR superseding this one
- **consequences written down before they are needed**: no CLA to contribute; no GPL or GPL-incompatible
  dependency in a shipped crate, enforced by the re-runnable census `G1-SLICE.15` already owns; Z3 stays a
  CI-only differential oracle behind `csp-z3` (its MIT licence would permit shipping it - the reason it is
  not shipped is the C++ dependency and the browser profile, and this record does not reopen that); and an
  iterative least-squares kernel is NOT covered by the numerical contract's exactness argument, so if it
  ever lands it must quantize into the internal units with a declared tolerance class, a fixed iteration
  bound and a declared convergence test - never "until it looks converged"
- **one re-open condition, in three parts stated now** so the G1 spike cannot be argued into satisfying one
  of them later: the spike shows a capability the custom kernel cannot reach within scheduled effort on a
  realistic pattern set; a named persona or partner wants it; and the project owner accepts the licence
  consequence in writing. A partial re-open is allowed in one direction only - `sc-sketch` licensed apart
  from the core if it stays an optional crate nothing in the default build links, the shape §6.1 uses for Z3
- the rollover this append triggered is performed in the same commit: slices 25-27 sealed into
  `docs/history/stitchcad-changelog-part4.md` with a digest-verified descriptor, the pointer table rebuilt
  from every segment on disk so it cannot drift, all pre-existing entry bodies byte-identical, and
  `run_changelog_ledger_probes.sh` green on the result
- gates: `make gate` -> "=== all doctrines green ==="; `make probes` -> "11 suite(s) green"; `make book` ->
  exit=0; no product code touched and no book chapter added (this leaf's deliverable is the record)
- LIVE_STATUS: the G0 frontier moves to .9, ADR-0003 and the formula language

