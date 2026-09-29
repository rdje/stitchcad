# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here. It answers one question:
> *what is next?* Everything else belongs in layer B (`docs/tasks/`) or C (`docs/decisions/`).

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open the active task-tree below → its Current Frontier → continue from the next action.
3. Latest commit: derive it (`git log --oneline -1`) — do not trust a hand-carried hash that
   the commit recording it would invalidate.

## Current state

- **Project:** StitchCAD — a sewing CAD with a headless Rust core: construction-recipe
  designs, versioned evidence-bearing Factory Profiles, deterministic artifact export
  (DXF/HPGL/PDF/tech pack), agent-first via MCP. `ROADMAP.md` v0.2, DRAFT until G0 exits.
- **Active tree:** `SPINE` → frontier leaf `SPINE.4.2` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) and resumes after the `SPINE` hygiene leaves.
- **Next action:** build the containment data plane — `.doctrine/live_document_size/surfaces.tsv`
  (one row per governed surface: path/glob, lifecycle class, owner, authority, measured
  lines/bytes/max-line, health target, inclusive ceiling, transition-debt baseline) and `routes.tsv`
  (every destination the README, `README_POLICY.md` and the guard's failure guidance route to, with
  owner + lifecycle + pressure control); derive the README caps from the measured survivor and set
  them via `README_LINE_CAP`/`README_BYTE_CAP` instead of the inherited 300/16 384. Classify every
  live surface (record the census command). Commit `STITCHCAD-SPINE-0004b (leaf SPINE.4.2)`.
- **Order after that:** `SPINE.4.3` (the checker + RED arms, closes D13) → `SPINE.5` (toolbox /
  knowledge-map rows, closes D7/D9) → `PLANNING.3` (G5–G7, V1, V2 + coverage census) →
  `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **Open defects:** D7, D9 (`SPINE.5`), D13 (`SPINE.4.2`/`.4.3`), D10 (`G1-SLICE.1`) — census in
  `PLANNING.md`.
- **Push cadence:** 400 commits between pushes (`COMMIT.md` → Push cadence); count with
  `git rev-list --count origin/main..HEAD`. The authorised first push is made and CI is green
  (`doctrines`, `rust`: both `success`); the cadence is in force, so the next push is at 400.
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
