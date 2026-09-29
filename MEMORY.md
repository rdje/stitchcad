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
- **Active tree:** `PLANNING` → frontier leaf `PLANNING.3` (`pending`).
- **Next action:** create `docs/tasks/G5-SHELLS.md`, `docs/tasks/G6-CONFORMANCE.md`,
  `docs/tasks/G7-RELEASE.md`, `docs/tasks/V1-ASSEMBLY.md`, `docs/tasks/V2-SIM.md` (leaves at
  roadmap §11 exit-criterion granularity, each citing its clause), add the coverage census tool
  `docs/tasks/artifacts/planning/run_tree_coverage_census.sh`, publish the roadmap → tree
  coverage map, register everything in `docs/TASK_TREE.md`, then commit
  `STITCHCAD-PLANNING-0003 (leaf PLANNING.3)`.
- **Order after that:** `SPINE.7` → `SPINE.8` (prove then locally close defect D15 — must land
  before any code leaf) → `SPINE.1`–`.5` → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
