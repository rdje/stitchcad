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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.1` (`pending`). Done: `.2` units &
  tolerances, `.3` garment ontology, `.13` reference skirt (all three normative in the book), `.18`
  `sc-units` + `sc-core` + G0 CI.
- **Next action:** write the glossary — `docs/book/src/spec/glossary.md`: one entry per construction
  term used anywhere in the spec set, each with a plain-language definition, the ontology object it
  names, the synonyms factories and other CADs use, and the machine token that must never be rendered
  raw. Then every term in the three written chapters is defined, and `G0-CONTRACT.4` (feature matrix)
  can follow. Commit `STITCHCAD-G0-0001 (leaf G0-CONTRACT.1)`.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
