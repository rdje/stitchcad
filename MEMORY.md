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
- **Active tree:** `PLANNING` → frontier leaf `PLANNING.2` (`pending`).
- **Next action:** create `docs/tasks/G1-SLICE.md`, `docs/tasks/G2-2D.md`,
  `docs/tasks/G3-GRADING.md`, `docs/tasks/G4-PROFILES.md` — leaves at roadmap §11
  exit-criterion granularity, each citing its clause — register all four in
  `docs/TASK_TREE.md`, then commit `STITCHCAD-PLANNING-0002 (leaf PLANNING.2)`.
- **Order after that:** `PLANNING.3` (G5–G7, V1, V2 + coverage map) → `SPINE.1`–`.5` →
  `G0-CONTRACT.1`–`.15` (specification only; no product code before G1).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
