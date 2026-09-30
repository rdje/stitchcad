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
| [`part7.md`](docs/history/stitchcad-changelog-part7.md) | slices 34–35, `STITCHCAD-G0-0013d` … `STITCHCAD-G0-0008` | 105 lines, 9793 bytes, `sha256:ff62d418…` |
| [`part8.md`](docs/history/stitchcad-changelog-part8.md) | slices 41–42, `STITCHCAD-G0-0014` … `STITCHCAD-G0-0004b` | 106 lines, 9766 bytes, `sha256:2c7ee35a…` |

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

## STITCHCAD-G0-0009 - the formula language, and the drafting system named with it (leaf `G0-CONTRACT.9`)

ADR-0003 had two halves and the repository held neither: roadmap §5 requires the recipe's expression language
"specified HERE, not later", and §11's G0 exit clause names it. Both are written now, and every number in the
chapter is computed by a tracked evaluator rather than typed beside it.

- **the language, in three parts** - `docs/book/src/spec/formula-language.md` carries the contract (eight
  kinds, nine name origins, declaration-order evaluation, exact rational arithmetic with two declared rounding
  points, twelve diagnostics, four structural limits, the exclusions); `formula-language/grammar.md` the
  syntax (the EBNF, literals and their seven unit tokens, the display and canonical forms, the operator,
  function and selector tables); `formula-language/examples.md` the evidence (17 bindings, 4 assertions, 13
  refusals over the reference skirt). 308 / 247 / 92 lines, each inside the `book_collection` per-part health
  of 400 / 24 576: the single file this replaced was 599 lines / 37 317 B, which is 87 % of the ceiling on the
  day it was born, so the containment doctrine's own remedy for a partitioned surface applied - and the parts
  table is censused in both directions, so a fourth file cannot appear unlisted.
- **the instrument** - `run_formula_language_census.sh` carries a reference evaluator that reads the chapter's
  OWN tables (kinds, unit ratios, the product law, the signatures) and type-checks every example with them:
  `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, twelve names cross-checked against the fixture
  chapter (the D27 class, derived rather than read), the fixture's four oracles holding as `assert`
  statements, and each refusal raising the token its row names. `run_formula_language_probes.sh` ->
  `15 pass / 0 fail`, with a CONTROL arm keeping an unrelated prose edit green so the RED arms are not
  vacuous.
- **the named drafting system** - Aldrich's metric pattern cutting. The four rejected candidates carry what
  was actually read on this machine: Seamly2D's repository is GPL-3.0, so its blocks would be a projection of
  GPL code and no independent oracle; Müller & Sohn was the least verifiable from here (an interstitial, no
  bibliographic record); Armstrong's record carries no metric claim; FreeSewing is an archived monorepo of
  individually authored designs. The method is adopted and the text is not, nothing has been read yet, so
  every number that will come from it is `unverified-with-owner` - and `G3-GRADING.16`, created by this slice,
  ships the blocks, because a decision with no owner is a wish.
- **decisions** - `docs/decisions/decision_adr-0003-construction-recipe-and-formula-language.md` records
  fourteen language decisions with the alternative each rejected, the 20x margin the node limit is derived
  against, and the three conditions that would re-open it.
- **glossary** - twelve new terms and two entries updated (`reference drafting` and `block (pattern)` now cite
  the decision instead of parking it in a leaf), the A-Z index re-derived: `289 terms / 8 parts / 153 tokens /
  0 failure(s)`. The census caught a collision while it was being written: the grammar's metavariable `T` was
  already the `T-notch` entry's token, so one token had two meanings and the rule that forbids it was
  satisfied. Notation is now italic and carries its own table - a code span is a claim that the span is a
  machine token.
- **D48 logged and fixed** - `docs/tasks/G3-GRADING.md` carried two `## Acceptance Checklist` headings (D15's
  class, by heading instead of by box) and a children range that still said `.14` while `.15` was in the file.
  **D36's third instance** recorded and removed in the same pass: `LIVE_STATUS.md` listed D47 open after
  `SPINE.20` had closed it, and its probe-suite count is now the command's rather than a hand-kept number.
- **both ledgers rolled over in the commit that crossed them** - `devnotes-part3` (50 lines / 4723 bytes) and
  `changelog-part8` (slices 41-42) sealed under the descriptor contract, each digest reproduced by
  `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `16 suite(s) green`; `make book` ->
  exit=0; fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`; matrix `105 rows /
  29 diagnostics / 0 failure(s)`; standards `6 registered / 0 failure(s)`; uncertainty `107 markers /
  13 files / 0 unowned`; containment `OK - 17 surfaces, 15 routes, 90 files measured`

## STITCHCAD-G0-0014c - a delegated decision is bounded, and what the project does not know is derived (leaves `G0-CONTRACT.14c`, `G0-CONTRACT.19`)

The director's instruction was to decide and act on the three findings. The first was that the engineer proposed
the roadmap amendment and then applied it; the second was that three seats are empty. Both are now governed
rather than reported.

- **governance §6.1, decisions made under delegation** — five rules for the case this project actually runs in:
  record the author and the applier and say so when they are the same party; the author of a decision may never
  approve the evidence that decision requires (where no independent reviewer exists the claim stays
  **unapproved**, and is recorded as unapproved); the consequences become instruments others can run; the record
  states what would reverse it and who may; and a delegation to decide is not one to upgrade evidence. A
  disclosure decays with the conversation it was made in - a rule is read by whoever acts next
- **the rule bites in three verifiable places**: the roadmap's Appendix A v0.3 entry now states that author and
  applier were the same party (`grep -c 'same one' ROADMAP.md` -> `1`); `G3-GRADING.14`'s acceptance withholds
  the exit review from the criterion's author; and `decision_self-application-under-delegation.md` carries the
  reasoning with the v0.3 instance as its founding measurement. The bound is on CERTIFICATION, not on action -
  waiting for an approver would stall the project, and letting the author certify would make every gate a
  formality
- **the revision-aware baseline caught this slice's own roadmap edit**, which is the mechanism working rather
  than a nuisance: four lines of disclosure grew v0.3 from 947 to 951 lines and `check_live_doc_size.sh`
  refused it (`transition debt WIDENED on lines (951 > baseline 947)`). Handled by the rule - re-based to v0.3's
  final state with the authority cited in the row's notes - and the limit it exposed is recorded rather than
  hidden: `at=<revision>` binds a baseline to a revision MARKER, not to a commit, so the real control is that
  every re-base is visible in a diff beside the content that moved it
- **G0-CONTRACT.19, the uncertainty census**: `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh`
  -> `uncertainty census: 88 markers / 10 files / 0 unowned / 0 failure(s)`, enumerating every marker in the
  book's own vocabulary (`assumed`, `unknown`, `unverified-with-owner`, `read-in-repo`,
  `cited-from-roadmap`, `read-external`, `known`, `derived`, `(proposed)`, `vacant`) per file and per resolving
  authority, and refusing a blocking marker whose verification-status section names no resolver. So the day a
  name arrives, the work it unblocks is one command away, and an edit that quietly drops an `assumed` from a
  fixture constant changes a count somebody reads
- **six probe arms** -> `probes: 6 pass / 0 fail`: an owned-claim control, an unowned synthetic chapter refused
  by name, a chapter added AFTER the census was written refused (the rule is about the population, not today's
  files), a definition left alone (U1 reads verification-status sections, because deciding that any mention is a
  claim would be a classifier guessing at meaning), and an absent book refusing with exit=2
- **building it found two defects in existing instruments, both fixed here**: the glossary census's `resolve()`
  deleted one `/x/../` per gsub pass and so reported `../../governance.md` - a file that exists - as a dead
  reference (it now normalises segment by segment -> `277 terms / 8 parts / 146 tokens / 0 failure(s)`); and
  this census's first authority list counted a bare gate id as an owner, so "frozen as a golden at G2" satisfied
  it and the UNOWNED arm passed for the wrong reason until the list was narrowed to parties that can RESOLVE a
  claim. A census whose own probe arm passes for the wrong reason is the vacuous-green class, fifth instance
- the token census fired a sixth time at authoring time: `vacant` is a status an instrument greps for, so it
  became a glossary term (`vacant seat`, ⚠, owning `vacant`) with the A-Z index re-derived
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `15 suite(s) green`; `make book` ->
  exit=0; containment `OK - 17 surfaces, 15 routes, 89 files measured`; matrix 105 rows, standards 6
  registered, fixture 20 rows / 4 checks / 5 pieces, coverage 13 trees / 3 siblings - all 0 failure(s)

## STITCHCAD-SPINE-0020 - the table convention becomes a gate (leaf `SPINE.20`)

`SPINE.15` settled D22 against a rendered page and left the answer enforced by nothing: the convention lived in
`COMMIT.md` prose while the only gate that reads table cells asserts the opposite. A rule that lives in a doc is
a suggestion, and what this one prevents is a silently dropped column in the book the director reads.

- **`TABLE-CODE-PIPE`** (`scripts/check_table_code_pipes.sh`, registered in the project slot, mirrored in
  `DOCTRINE_ENFORCEMENT.md`): a staged `.md` table row carrying a raw pipe inside a code span is refused, with
  the file, the line, the offending span and the escaped form to write instead. Fence-aware (a quoted row inside
  a code block is documentation, not a table), code-span aware for any backtick run, and scoped to table rows -
  prose has no cell to split, and an ordinary separator is the arity checker's business, not this one's
- **seven self-test arms** -> `table-code-pipe --self-test: 7 arms, 0 failed`: raw pipe refused, escaped
  accepted, ordinary separator accepted, fenced quotation accepted, double-backtick span refused, prose
  accepted, indented row refused. And it fires in the real hook path, demonstrated rather than assumed: staging
  a scratch file with a bad row made the enforcer print `PROJECT TABLE-CODE-PIPE: BREACH (exit=1)` before the
  file was unstaged and deleted
- **absolute, not a ratchet, and measured before choosing**: `bash scripts/check_table_code_pipes.sh --all` ->
  `table-code-pipe --all: 88 tracked .md files, 0 offending table rows`, exit=0, so nothing had to be
  grandfathered. The `--all` mode exists so that claim is re-runnable instead of a memory of the slice that
  measured it - the precedent is `check_gap_claims.sh --all`, and the breach it avoids is D20
- **the inherited checker is untouched** (`git diff --stat HEAD -- scripts/check_table_arity.sh` -> empty): a
  defect in NEUTRAL spine code is fixed in the project slot and reported upstream with its evidence, never
  patched locally where `scripts/update_scaffold.sh` would silently overwrite or diverge it. D47 closes; the
  divergence is recorded in `DOCTRINE_ENFORCEMENT.md` and in
  `decision_table-cells-escape-pipes-render-to-settle.md`
- the doctrine and probe counts in `LIVE_STATUS.md` are re-derived, not incremented: `scripts/check_doctrines.sh
  | grep -c '✅'` -> `13` printed rows (12 universal including the conditionally appended `KNOWLEDGE-MAP`, plus
  the project row), `check_doctrines.project.sh` -> `3 project doctrine(s) green`, `find docs/tasks/artifacts
  -name 'run_*probe*.sh' | wc -l` -> `14`
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `14 suite(s) green`; `make book` ->
  exit=0; containment `OK - 17 surfaces, 15 routes, 89 files measured`; every census `0 failure(s)`.
  `scripts/` changed, so the immediate push is owed and the observed CI verdict goes into the leaf

## STITCHCAD-SPINE-0015 - the table convention is settled by a rendered page (leaf `SPINE.15`)

D22 asked whether a GFM renderer splits a table cell on a raw `|` inside a code span, and was logged as a
*question* because the two instruments in the tree disagreed: the inherited `check_table_arity.sh` self-tests
that a code-span pipe is NOT a separator, while `DOCTRINE_ENFORCEMENT.md` warns that GFM silently drops extra
cells. Reading a specification cannot settle that, so a page was rendered.

- **the answer, from a rendered page**: a raw pipe inside a code span SPLITS the cell - a 3-column row whose
  first cell carried a code span with a raw pipe came back as `A raw pipe in a code span: ` + "`x" + ` | ` + "y`" + ` | `2`,
  with the rightmost cell (`3`) **silently dropped**; the same row with the pipe escaped kept all three cells
  and rendered a literal pipe. The oracle is tracked and re-runnable:
  `docs/tasks/artifacts/table_render/run_table_render_probes.sh` -> `probes: 3 pass / 0 fail`, printing the
  rendered rows, and refusing with exit=2 when `mdbook` is absent rather than reporting green over a page
  nobody rendered. Its third arm pins the divergence - the inherited checker's own self-test still asserts the
  opposite, so the gate under-reports what the renderer does
- **the first cut of the oracle was wrong in an instructive way**: it asserted "2 cells" from a 2-column page,
  and a 3-column page split into 3 cells with the last one dropped. Same defect, different shape, and an arm
  anchored on a count goes red on the truth - so the arm now asserts the PROPERTY (first cell truncated at the
  pipe, rightmost cell not the one written)
- **the convention is written where authors look** (`COMMIT.md`, per the leaf's acceptance): escape every pipe
  in a table cell including inside code spans; a cell is not a paragraph, so content that outgrows its column
  becomes a bounded subsection; and a maxline target is a shape budget whose 80% warning means AT BUDGET
- **both remaining prose-derived maxline targets are re-derived** with the `SPINE.4.4` instrument rather than
  guessed: `decisions_collection` `320` -> **382 B** (binding shape: the index's record/type/hook row, 17 rows)
  and its `maxline=491` debt **cleared**; `tasks_collection` `400` -> **443 B** (binding shape: the verification
  log's date/leaf/checks/result row, 52 rows). `check_live_doc_size.sh` -> `OK - 17 surfaces, 15 routes,
  87 files measured`, exit=0, at `353 B = 92% of 382` and `443 B = 100% of 443`
- **the target moved AND the rows moved**, because doing only one is the mistake: the `491` B class/behaviour/
  files row that carried the debt became two bounded bullets (the house remedy), nine index hooks that had
  grown into three-line summaries are one line each again, and four verification-log rows above the derived
  budget were tightened. Raising a number to fit verbosity launders the verbosity; trimming rows to fit a
  guessed number launders the guess
- **the inherited checker is untouched** (`git diff --stat HEAD -- scripts/check_table_arity.sh` -> empty): a
  NEUTRAL spine file is reported upstream, never patched locally. The local gap is **D47** - nothing here
  mechanically refuses the row the renderer truncates - owned by a new leaf **`SPINE.20`**, with the render
  probe as its ground truth. A convention in `COMMIT.md` is what authors read; a gate is what holds when they
  do not
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `14 suite(s) green`; `make book` ->
  exit=0; `check_live_doc_size.sh --self-test` -> `15 arms, 0 failed`; every book census and the coverage
  census at `0 failure(s)` / `0 orphan(s)`

## STITCHCAD-G0-0004c - the envelope criterion is law, not a proposal (leaves `G0-CONTRACT.4c`, `SPINE.4.5`, `PLANNING.6`, `G0-CONTRACT.14b`)

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

