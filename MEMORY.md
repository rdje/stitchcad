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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.4`. Done: `.1` glossary (239 terms, 8
  parts, census-derived), `.2` units & tolerances, `.3` ontology, `.13`+`.13b` reference skirt, `.18`
  `sc-units` + `sc-core` + G0 CI.
- **Next action:** write `.4`, the feature matrix — `docs/book/src/spec/feature-matrix.md`: one row per
  construction in the v1 woven envelope (A-line skirt, darted bodice, set-in sleeve, classic collar,
  trousers) marked supported / rejected / deferred with a reason, an owning gate and the diagnostic a
  rejected construction must produce; the §1.3 non-goals appear as rejected rows; every term it
  introduces joins the glossary, which the census then enforces. Commit `STITCHCAD-G0-0004`.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement), and
  owns **D27** — the reference fixture's waistband is two different garments at once (§4 vs §6).
