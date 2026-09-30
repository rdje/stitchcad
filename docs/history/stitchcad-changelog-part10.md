# Sealed archive — StitchCAD changelog, two spine slices on the table convention

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the
live window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 73 lines, 6652 bytes, `sha256:062ccfa313ec6e40e3627d24d3fe20d458385c52bddc562f269a2e120745d175`
- **Sealed by:** leaf `SPINE.21` (the append that crossed the rollover milestone performed the rollover,
  as the doctrine requires).
- **Coverage:** `STITCHCAD-SPINE-0020` and `STITCHCAD-SPINE-0015`, newest first, exactly as they stood in
  `CHANGELOG.md`. No slice range is declared, for the reason `stitchcad-changelog-part9.md` records: the
  ranges in the earlier descriptors have no producer and disagree with a derivation from git (defect D51).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`.
- **Write policy:** none — sealed segments are immutable.

---

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
