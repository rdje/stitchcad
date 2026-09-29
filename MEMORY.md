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
- **Active tree:** `SPINE` → frontier leaf `SPINE.9` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) so the acceptance-gate work landed first; it resumes after `SPINE.10`.
- **Next action:** guard `scripts/update_scaffold.sh` against defect D17 — its NEUTRAL list names
  `docs/TASK_TREE.md`, `TOOLBOX.md`, `README_POLICY.md`, `docs/tasks/TEMPLATE.md` (lines 27, 28, 31,
  33) although the template tells projects to fill them in, so a scaffold sync would clobber the
  task-tree index. Split the list, back up + skip project-content files unless an explicit flag is
  passed, prove it with a dry-run probe, record `decision_scaffold-sync-protects-project-content.md`
  (already cross-linked), then commit `STITCHCAD-SPINE-0009 (leaf SPINE.9)`.
- **Order after that:** `SPINE.10` (repo-volume scratch, `make probes`) → `PLANNING.3` (G5–G7, V1,
  V2 + coverage census) → `SPINE.1`–`.5` → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
