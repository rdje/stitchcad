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
- **Active tree:** `SPINE` → frontier leaf `SPINE.11` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) and resumes after the `SPINE` hygiene leaves.
- **Next action:** rebuild the evidence-signature corpus as a **tracked** instrument under
  `docs/tasks/artifacts/evidence_signatures/` (corpus + runner) so the published `12 of 36`
  awk-versus-grep measurement in `SPINE.8`'s record is re-derivable by one command (defect D20, a
  leg-3 breach of the standard just adopted). Wire it into `make probes`, add a `TOOLBOX.md` row,
  then commit `STITCHCAD-SPINE-0011 (leaf SPINE.11)`.
- **Order after that:** `SPINE.4` (live-doc containment + the derived README caps and destination
  registry the adopted policy requires) → `SPINE.5` (toolbox/knowledge map) → `PLANNING.3` (G5–G7,
  V1, V2 + coverage census) → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **Push cadence:** 400 commits between pushes (`COMMIT.md` → Push cadence); count with
  `git rev-list --count origin/main..HEAD`. The authorised first push is made and CI is green
  (`doctrines`, `rust`: both `success`); the cadence is in force, so the next push is at 400.
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
