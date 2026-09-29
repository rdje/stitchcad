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
- **Active tree:** `PLANNING` → frontier leaf `PLANNING.4` (`pending`), the last one; then this tree
  closes and the frontier belongs to `G0-CONTRACT`.
- **Next action:** close defect D24 with a layer-C decision record —
  `docs/decisions/decision_product-work-takes-the-frontier.md` (indexed, with an `answers:` line, and
  reachable from `CLAUDE.md`): product specification and code take the frontier; spine/governance work
  happens only when it blocks product work or a defect is live; the symptom is a run of commits none of
  which touches the product. Then commit `STITCHCAD-PLANNING-0004 (leaf PLANNING.4)`.
- **Then product work, in this order:** `G0-CONTRACT.2` (units & tolerance spec) → `.18` (`sc-units` +
  `sc-core` skeletons and the G0 CI workflow — the first product code) → `.3` (ontology) → `.13`
  (reference skirt with numbers) → the rest of G0.
- **Roadmap capture is complete and derived:** `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh`
  → `census: 10 lanes / 13 trees / 0 unowned / 0 orphan(s) / 0 dead link(s)`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
