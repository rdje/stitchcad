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
- **Active tree:** `SPINE` → frontier leaf `SPINE.8` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) because D15's mitigation must land before any code leaf, and `PLANNING.3` stages a
  census script.
- **Next action:** implement `FRESH-ACCEPTANCE-EVIDENCE` in `scripts/check_doctrines.project.sh`
  (or a `scripts/check_fresh_acceptance_evidence.sh` it calls): a staged CODE change must have its
  ticked ROOT CAUSE / ADDRESSED / NO REGRESSION bullets **added in this commit's diff**, with
  `--self-test` arms including a control seen RED; register it in the project slot; then commit
  `STITCHCAD-SPINE-0008 (leaf SPINE.8)`.
- **Order after that:** `SPINE.9` (scaffold-sync guard, D17) → `SPINE.10` (repo-volume scratch,
  D16) → `PLANNING.3` (G5–G7, V1, V2 + coverage census) → `SPINE.1`–`.5` → `G0-CONTRACT.1`–`.18`
  → `G1-SLICE` …
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
