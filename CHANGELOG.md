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
| [`part9.md`](docs/history/stitchcad-changelog-part9.md) | the two oldest live entries, `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d` — no slice range, because the earlier ranges have no producer (D51) | 90 lines, 8386 bytes, `sha256:8e4081d4…` |

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

## STITCHCAD-G0-0011 - the spike's rule is written before its measurement (leaf `G0-CONTRACT.11`)

ADR-0002 was the one ADR whose evidence does not exist yet, and nothing in the repository constrained what a
G1 spike would be allowed to conclude. The roadmap itself warns why that matters: "Custom wgpu, not DOM
canvas" is a hypothesis to test, not an axiom. So the rule was written first.

- **the record** - `docs/decisions/decision_adr-0002-ui-stack-and-canvas-spike-protocol.md` (116 lines):
  the chrome (Tauri + TypeScript/React, Slint as the named fallback, Flutter still rejected), the egui/iced
  dev shell and the TypeScript domain-logic ban are `active`; canvas hosting is `proposed`, in the status
  line rather than a footnote, because a decision whose evidence does not exist yet is still a decision
  *structure*. It carries the three topologies with the hypothesis each tests, a corpus declared before
  anybody measures it (16 pieces, 400 boundary vertices each, 200 pick probes, 200 snap probes, 1 000
  fidelity round trips) with a re-run trigger if a G3 garment exceeds it, seven gates each naming what it
  protects, and six rules: eligibility, correctness outranking speed, scoring inside a declared margin with
  a total tiebreak order, a one-renderer preference where that is free, escalation with a bounded fallback,
  and a verdict a human may overrule only by a recorded decision naming the rows.
- **the data plane and the instrument** - `docs/tasks/artifacts/canvas_spike/` holds the gates, the
  applicability table and the rule parameters as three TSVs, plus a `results.tsv` that is empty on purpose:
  `run_spike_verdict.sh` prints `PENDING` with exit=0 until `G1-SLICE.13` measures, refuses a data set that
  cannot produce a verdict, and otherwise prints the rule that decided each profile. Tightening a threshold
  is therefore a diff a reviewer sees, which the GATES-READ arm pins by changing the verdict.
- **the probe suite** - `run_spike_verdict_probes.sh` -> `13 pass / 0 fail` over twelve synthetic result
  sets whose verdicts are known in advance: a tie broken by memory, the fastest topology losing to
  `snap_exact = no`, a faster native-only winner giving way to one renderer for both profiles (R4), a
  profile with no survivor escalating to the `dev-shell` fallback (R5), an unmeasured gate, an undeclared
  topology, a missing row, a control and a missing plane. Six arms failed first against a correct
  instrument because each omitted a row the applicability table declared - recorded as the slice's lesson.
- **the consumer** - `G1-SLICE.13`'s acceptance now names the instrument by path, the corpus by its
  declared numbers, and requires the hardware, OS versions and corpus script identity in the results file,
  because a verdict is scoped to them.
- **the dev-notes ledger rolled over** in the commit whose append crossed it (`devnotes-part4`, 42 lines /
  3706 bytes, digest reproduced by `run_changelog_ledger_probes.sh` -> `9 pass / 0 fail`).
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `18 suite(s) green`; `make book` ->
  exit=0; all seven book censuses unchanged; containment `OK - 17 surfaces, 15 routes, 99 files measured`

## STITCHCAD-G0-0010 - the dialects are a closed registry, not a format with flags (leaf `G0-CONTRACT.10`)

ADR-0004 was the last of the four ADRs with neither a record nor a chapter, and three chapters already leaned
on the missing one: sixteen glossary entries named the leaf as their specifier, the ontology pointed at "the
interchange-dialects chapter", and the instantiation paths deferred their three modes to it.

- **the chapter** - `docs/book/src/spec/interchange-dialects.md` (303 lines / 21 650 B): six axes an export
  target is a tuple over, each naming the party that resolves it; a **closed** registry of four targets, so a
  tuple nobody validated is refused naming the nearest one; the seventeen-layer table in both naming modes
  with this project's object mapping, and the named mode's loss of separation declared instead of discovered
  by a partner; cut-as-1 against sew-as-1 as a profile mapping recorded in three places and never a writer
  default; one BLOCK per piece with SST and PST mandatory on the ASTM path; one polyline-only entity set for
  both releases, arcs travelling exactly as bulges and Beziers tessellating at T2's chordal bound; three
  grading carriages, each its own artifact and validation; HPGL and PDF; and the receiver-config record that
  turns a dispute into a comparison of fields. D6673-10's withdrawal is recorded with the convention
  implemented as de-facto and no conformance claim anywhere.
- **the instrument** - `run_interchange_census.sh` reads the layer list out of `ROADMAP.md` itself, so
  `17 layers / 4 targets / 12 entities / 0 failure(s)` is a closure against the roadmap and not against a
  list kept beside it; the axes and the registry columns are checked in both directions, the entity policy's
  floor and ceiling are pinned, and every diagnostic and link resolves. `run_interchange_probes.sh` ->
  `13 pass / 0 fail`, including ROADMAP-GROWS, which adds a layer to a *copy* of the roadmap and requires
  the refusal - a probe has no business editing a file the director owns.
- **what the chapter deliberately does not specify** - SST and PST field content. The syntax is
  case-sensitive and receiver-specific, so a field table written at G0 would be a guess in a normative font;
  G6 is the only oracle, and layers 84-87 stay absent for the same reason rather than carrying placeholder
  curves no design authored.
- **decisions** - `docs/decisions/decision_adr-0004-interchange-dialects.md` records ten decisions with the
  alternative each rejected, what was read (Wikipedia's DXF article: the published specification is
  incomplete, which is why the oracle is a receiver) and what was attempted and not read.
- **glossary** - five new terms, sixteen entries repointed from this leaf to a chapter clause, the A-Z index
  re-derived: `294 terms / 8 parts / 155 tokens / 0 failure(s)`.
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `17 suite(s) green`; `make book` ->
  exit=0; fixture, formula-language, matrix, standards, uncertainty and coverage censuses all green;
  containment `OK - 17 surfaces, 15 routes, 91 files measured`

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

