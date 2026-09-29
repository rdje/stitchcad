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
- **Active tree:** `SPINE` → frontier leaf `SPINE.10` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean); it resumes after `SPINE.10`.
- **Next action:** keep project-owned scratch on the repository volume (defect D16) — add a
  `make probes` target that runs every probe suite under `docs/tasks/artifacts/` with
  `TMPDIR="$PWD/target/scratch"`, without editing any inherited suite; verify with
  `TMPDIR=… mktemp -d` printing a repository-volume path and all suites reporting
  `probes: N pass / 0 fail`; then commit `STITCHCAD-SPINE-0010 (leaf SPINE.10)`.
- **Order after that:** `PLANNING.3` (G5–G7, V1, V2 + coverage census) → `SPINE.1`–`.5` →
  `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
