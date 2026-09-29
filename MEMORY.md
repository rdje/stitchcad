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
- **Active tree:** `SPINE` → frontier leaf `SPINE.4.3` (`pending`).
- **Next action:** write `scripts/check_live_doc_size.sh` and register it in the project doctrine slot:
  read `.doctrine/live_document_size/{surfaces,routes}.tsv`, re-measure every surface (lines, bytes,
  max content line, and for collections file count / per-part / aggregate), refuse on an unclassified
  surface, a missing owner or ceiling, an absolute or off-volume path in the data plane, an overflow
  past an inclusive ceiling, a widened debt baseline, a route to an unclassified destination, and a
  field-count mismatch; warn at 80 % of a health target. Add a probe suite with a RED arm per refusal
  class. Then commit `STITCHCAD-SPINE-0004c (leaf SPINE.4.3)`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
