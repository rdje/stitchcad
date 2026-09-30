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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.1`. Done: `.2` units & tolerances, `.3`
  garment ontology, `.13`+`.13b` reference skirt, `.18` `sc-units` + `sc-core` + G0 CI.
- **Next action:** land `.1`, the glossary — `docs/book/src/spec/glossary.md` plus 8 domain parts (239
  terms, one owner per machine token, ⚠ on the safety-relevant ones) and the census that derives its
  coverage (`docs/tasks/artifacts/glossary/run_glossary_census.sh`). That append crosses the changelog's
  rollover milestone, so the same commit reorders the live window to git order (D29) and seals the oldest
  entries into `docs/history/` with part1's coverage line corrected (D30). Then `.4`, the feature matrix.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement), and
  now also owns **D27** — the reference fixture's waistband is two different garments at once.
