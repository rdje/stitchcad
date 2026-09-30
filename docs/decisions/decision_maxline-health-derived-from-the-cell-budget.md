# A maxline health target is derived from the cell budget of the binding shape, not from prose and not from today's widest line

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — it changes one health target and one ceiling in
  `.doctrine/live_document_size/surfaces.tsv` (row `book_collection`), and a ceiling changes only by a
  recorded authority, which this record is
- **Owner / source:** leaf `SPINE.4.4`, created by the director's ruling of `2026-09-30`; measured with
  `docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh`, the tracked producer of every number below

answers: "how is a maxline health target derived?" · "why is the book's widest-line target 275 B and not 200 B?" · "why did the book_collection ceiling change?" · "what do I do when the checker warns about a table row?" · "may I raise a target to silence a warning?" · "why not split prose and table rows?"

## The decision

For a collection whose parts carry reference tables, the **maximum-content-line health target is the cell
budget of the binding shape**: the sum of its per-column 95th-percentile cell widths plus the separator
overhead, which for a GFM row is `3 × columns + 1` bytes (a pipe and a space each side of every cell, plus
the closing pipe). The **ceiling** is the same shape's `max-sum` — every column at its widest at once, which
no single row has to reach — raised to the ceiling-to-health ratio the rest of the registry uses (1.5×–2.5×).

Applied to `book_collection`, measured rather than chosen:

| Quantity | Value | Where it comes from |
| --- | --- | --- |
| binding shape | `Term｜What it means｜Canonical object｜Also called｜Machine token` | the glossary termbase: 276 data rows, 5 columns |
| per-column p95 cells | 21 + 106 + 62 + 50 + 20 = 259 B | the tool's per-column distribution |
| separator overhead | 16 B | `3 × 5 + 1` |
| **health maxline** | **275 B** (was 200) | 259 + 16 |
| shape max-sum | 379 B | every column at its widest at once |
| **ceiling maxline** | **440 B** (was 320) | `1.6 × 275`, and it covers the 379 B worst legitimate case |
| widest line today | 272 B | `docs/book/src/spec/glossary/measurements-and-fit.md` |

Re-derive all of it with one command: `bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` →
`cell budget: 36 shapes / 625 data rows measured / recommended maxline health 275 B`, `exit=0`, where
`exit=1` means the recommendation does not cover the population's widest actual line.

## Why

- **The old target was derived from the wrong shape.** `200` B fits a prose paragraph; no five-column
  termbase row can meet it. Every glossary part exceeded it (widest 272 B) while staying far inside the
  ceiling, so the check printed a warning with no defect behind it — and a warning nobody can act on trains
  the next reader to skip the ones that mean something, which is the same failure as a gate that cries wolf.
- **A cell budget is a contract; today's widest line is not.** The registry forbids deriving health "from
  today's largest file", and for a max axis that prohibition bites hardest: the widest line is one row's
  accident. A per-column budget is what each column *must* carry — a term, one sentence of meaning, a link
  to the clause that specifies it, the synonyms a factory uses, the token — so it survives the row it was
  measured from.
- **The ceiling was below the shape's own worst legitimate case.** A row where all five columns peak is
  379 B, which the old 320 B ceiling would have refused — and G0 still owes chapters whose tables are
  reference material (the formula language, the interchange layer table, the release manifest). Refusing a
  legitimate row of the shape the target was derived from is a ceiling that is wrong, not a row that is fat.
  The ratio, 440 = 1.6 × 275, is the registry's own convention rather than a new idea.

## Why not the two obvious alternatives

- **Raise the health target until the warning stops — rejected, and the arithmetic is why.** The checker
  warns at 80 % of health, so silencing a 272 B row needs health above 340 B. Setting health to the max-sum
  (379 B) would do it, but then the denominator is the *worst case* rather than the steady state, and a prose
  chapter could carry a 500-byte line without failing anything. The warning is not the defect; a target
  derived from the wrong shape was.
- **Split the book into a prose row and a table row — measured, deferred, and the measurement is recorded so
  it is not repeated.** The checker claims a file per row with `PARTOF[$path] = sid`, so the LAST matching
  row wins and an overlapping split double-counts the maxima: a termbase row would still set
  `book_collection`'s widest line unless the collection's glob stopped matching it. One registry field holds
  one glob, and `git ls-files -- 'docs/book/src/*.md'` crosses `/`, so excluding a subdirectory needs either
  `:(glob)` pathspec magic (verified to work here: `git ls-files -- ':(glob)docs/book/src/*.md'` lists only
  the three top-level chapters) with three partitioning rows and their own aggregate and file-count
  ceilings, or a `kind=list` row of exact paths that refuses every new chapter until it is registered. Both
  are real options and both are a bigger change than this leaf; the trigger for revisiting is a second shape
  whose budget differs from the termbase's by more than ~20 %, at which point one target genuinely cannot
  serve both.

## How to apply

- **When the warning fires on a table row:** split the row into a bounded subsection, or trim the cell — the
  remedies `G0-CONTRACT.2` and `.7` both used when the same ceiling caught their tables. Do not raise the
  target; a target rises only when the *shape* changes, and then only by re-running the tool.
- **When a new table shape appears** (a new column contract, a new reference part): re-run
  `run_cell_budget_census.sh`. It reports every shape's budget and names the binding one, so the derivation is
  a measurement rather than an argument.
- **The persistent warning is owned, not ignored.** At 272 B the collection sits at 99 % of its derived
  health, so the 80 % band still prints. That is now a true statement — *the termbase is at its budget* — and
  the remedy that would clear it is a table-authoring convention (keep cells short enough that the axis has
  room), which is `SPINE.15`'s, alongside defect D22's oracle. The registry already carries this shape of
  honest warning for `decisions_collection` and `tasks_collection`.

Related: [[decision_live-document-containment-proportionate-adoption]] ·
[[decision_director-ruling-2026-09-30-four-findings]] · `.doctrine/live_document_size/surfaces.tsv` (the
`book_collection` row and its header's derivation notes) · `docs/tasks/PLANNING.md` (defect D22).
