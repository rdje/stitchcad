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
- **Active tree:** `SPINE` → frontier leaf `SPINE.4` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) and resumes after the `SPINE` hygiene leaves.
- **Next action:** adopt live-document size containment (defect D13) — inventory every governed
  surface (`MEMORY.md`, `README.md`, `ROADMAP.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `LIVE_STATUS.md`,
  `docs/TASK_TREE.md`, `docs/tasks/*`, `docs/decisions/*`, `docs/book/src/*`) with a lifecycle class,
  a measured health target and an enforcement ceiling; derive the README caps the adopted
  `README_POLICY.md` now requires instead of the inherited 300/16 384 defaults; add the destination
  registry its routing-pressure section requires; and enforce it all with one deterministic check in
  the project slot (RED arm demonstrated). Then commit `STITCHCAD-SPINE-0004 (leaf SPINE.4)`.
- **Order after that:** `SPINE.5` (toolbox/knowledge-map rows, closes D7/D9) → `PLANNING.3` (G5–G7,
  V1, V2 + coverage census) → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **Open defects:** D7, D9 (`SPINE.5`), D13 (`SPINE.4`), D10 (`G1-SLICE.1`) — census in `PLANNING.md`.
- **Push cadence:** 400 commits between pushes (`COMMIT.md` → Push cadence); count with
  `git rev-list --count origin/main..HEAD`. The authorised first push is made and CI is green
  (`doctrines`, `rust`: both `success`); the cadence is in force, so the next push is at 400.
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
