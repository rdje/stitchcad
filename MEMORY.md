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

- **Project:** StitchCAD — a sewing CAD with a headless Rust core: construction-recipe designs,
  versioned evidence-bearing Factory Profiles, deterministic artifact export, agent-first via MCP.
  `ROADMAP.md` v0.2, DRAFT until G0 exits.
- **Active tree:** `PLANNING` → frontier leaf `PLANNING.3` (`pending`). `SPINE` is parked with `.5`,
  `.13`, `.15` open; none blocks product work.
- **Next action:** finish the roadmap → tree capture: seed `G5-SHELLS`, `G6-CONFORMANCE`,
  `G7-RELEASE`, `V1-ASSEMBLY`, `V2-SIM` (leaves at roadmap §11 exit-criterion granularity, each citing
  its clause), add the coverage census tool `docs/tasks/artifacts/planning/run_tree_coverage_census.sh`
  proving every §11 gate/track is owned and every tree cites a lane, publish the coverage map, register
  all five in `docs/TASK_TREE.md`, then commit `STITCHCAD-PLANNING-0003 (leaf PLANNING.3)`.
  **Measured gap right now:** 5 of 10 roadmap lanes have no tree (`G5`, `G6`, `G7`, `V1`, `V2`).
- **Then product work, in this order:** `G0-CONTRACT.2` (units & tolerances spec) → `.18` (`sc-units` +
  `sc-core` skeletons and the G0 CI workflow — the first product code) → `.3` (ontology) → `.13`
  (reference skirt with numbers) → the rest of G0.
- **Sequencing rule (director, 2026-09-29):** the roadmap must be fully captured first, then product
  specification and code take the frontier. Spine work happens only when it blocks product work or a
  defect is live; the displacement that produced 15 spine commits before any product commit is logged
  as D24 in `docs/tasks/PLANNING.md`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
