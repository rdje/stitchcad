# A rendering question is settled by rendering, and a table cell's pipe is always escaped

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — the convention is in `COMMIT.md` where authors look, its oracle is a tracked probe,
  and the inherited checker that disagrees with the oracle is untouched and reported upstream (defect **D47**,
  owned by `SPINE.20`)
- **Owner / source:** leaf `SPINE.15`, settling defect **D22** — two tracked instruments in this repository
  disagreed about what a markdown table row *means*, and neither could settle it

answers: "must I escape a pipe inside a code span in a table cell?" · "what does a renderer do with a raw pipe in a table cell?" · "how do I settle a disagreement between two instruments?" · "why does the arity gate not catch this?" · "how wide may a table row be, and who decides?"

## The decision

1. **Escape every pipe in a table cell, code spans included.** Write `` `x \| y` ``, never `` `x | y` ``. A raw
   pipe is a cell separator to the renderer whether or not it sits between backticks.
2. **A cell is not a paragraph.** When a cell outgrows its column's budget, the content moves into a bounded
   subsection under the table and the row keeps a summary — the remedy `G0-CONTRACT.2` and `.7` both used when
   the containment ceiling caught their tables.
3. **A widest-line target is a shape budget, and its warning means *at budget*.** It is derived from the
   binding table shape's per-column cell budget
   (`decision_maxline-health-derived-from-the-cell-budget.md`), never guessed and never raised to fit a row.
   Raise the derivation or tighten the row; do both when both are wrong.
4. **A question about what a renderer does is settled by rendering.** Not by reading the specification, and not
   by asking which of two instruments is more authoritative.

## The evidence, from a rendered page

`bash docs/tasks/artifacts/table_render/run_table_render_probes.sh` → `probes: 3 pass / 0 fail`, which builds a
scratch mdBook, parses the HTML and prints the rows it found:

```
· 3 cell(s): Case|Second|Third
· 3 cell(s): A raw pipe in a code span: `x|y`|2
· 3 cell(s): B escaped pipe in a code span: x | y|2|3
```

The source row was `| A raw pipe in a code span: \`x | y\` | 2 | 3 |` under a three-column header. The renderer
broke the code span open at the pipe, shifted the cells, and **dropped the rightmost cell** — `3` is not in the
output, and nothing anywhere reported its loss. The escaped row kept all three cells and rendered a literal
pipe. So `DOCTRINE_ENFORCEMENT.md`'s warning ("GFM silently DROPS extra cells and PADS missing ones") is what
happens, and the inherited `scripts/check_table_arity.sh` — whose own self-test asserts "a pipe inside a code
span is not a separator" — under-reports it. That checker is NEUTRAL spine code, so it is reported upstream and
never patched here; the local enforcement gap is **D47**, owned by `SPINE.20`.

## Why the oracle is a probe and not a memory

- **The first cut of the oracle asserted a count and was wrong on the truth.** From a two-column page the split
  row yields two cells; from a three-column page it yields three with the last one dropped. An arm anchored on
  "2 cells" goes red when the renderer does something *worse* than expected, which is the same trap as a RED arm
  that removes one instance of a property instead of the property. The arm now asserts the property: the first
  cell truncated at the pipe, and the last cell not the one that was written.
- **It refuses rather than passing without its tool.** No `mdbook` → `exit=2`, because a green verdict over a
  page nobody rendered is the vacuous-green class this repository has measured five times.
- **One renderer is one leg, and the missing leg is stated.** This proves mdBook's HTML backend
  (pulldown-cmark with GFM tables) — the renderer this project's own book ships through. GitHub's current
  renderer, a factory PLM viewer and a printed page were not tested. The convention does not wait for them,
  because escaping a pipe is safe under every renderer including the ones that would have protected it.

## How to apply

- **Writing a table:** escape pipes; keep each cell to what its column must carry; move a paragraph into a
  subsection. If `check_live_doc_size.sh` warns on a widest line, read it as *at budget* and tighten the row.
- **Deriving a widest-line target:** `CELL_BUDGET_GLOB='<glob>' bash
  docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` → it reports every shape's per-column widest and
  p95 cells and refuses to recommend a target that does not cover the population's widest actual line.
- **When two instruments disagree:** find the artifact they both claim to describe and measure it. The
  disagreement is a fact about one of them, and reading either one's source is not evidence about the artifact.

Related: [[decision_maxline-health-derived-from-the-cell-budget]] ·
[[decision_revision-aware-containment-baseline]] · `COMMIT.md` (Table authoring) ·
`docs/tasks/artifacts/table_render/run_table_render_probes.sh` · `docs/tasks/PLANNING.md` (defects D22, D47).
