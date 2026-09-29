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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.2` (`pending`). `PLANNING` is closed
  (all four leaves done); `SPINE` is parked with `.5`, `.13`, `.15` open — none blocks product work.
- **Next action:** write the units & tolerance specification — `docs/book/src/spec/units-and-tolerances.md`
  from roadmap §4.2: fixed-point micrometres as the single internal unit, the five tolerance classes
  and how each is derived, the curve set (line / circular arc / cubic Bézier, NURBS deferred), robust
  predicates, and the offset error-budget contract. Add it to `docs/book/src/SUMMARY.md`, verify with
  `mdbook build docs/book`, and commit `STITCHCAD-G0-0002 (leaf G0-CONTRACT.2)`.
- **Then:** `G0-CONTRACT.18` (`sc-units` + `sc-core` skeletons and the G0 CI workflow — the first
  product code) → `.3` (ontology) → `.13` (reference skirt with numbers) → the rest of G0.
- **Sequencing rule (`docs/decisions/decision_product-work-takes-the-frontier.md`):** product work
  takes the frontier; spine slices only when they block it, a defect is live, or the director asks.
  Census: `git log --oneline | grep -cE 'leaf (G[0-7]|V[12])'` must start growing.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
