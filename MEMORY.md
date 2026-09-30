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

- **Project:** StitchCAD — a sewing CAD, headless Rust core: construction-recipe designs, evidence-bearing
  Factory Profiles, deterministic export, agent-first via MCP. `ROADMAP.md` v0.2, DRAFT until G0 exits.
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.8`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.14`, `.18`.
  Remaining: `.9`–`.12`, `.15`–`.17`.
- **Next action:** take **`SPINE.4.4`** (re-derive the `book_collection` maxline health for table-shaped
  reference parts — the ruling of `2026-09-30` puts it before the next product chapter) → **`.9`** (ADR-0003 +
  the formula language v1: grammar, units in expressions, conditionals, name binding, evaluation order, error
  and dimension rules, worked over the reference skirt, plus the reference block set — a big chapter, whole
  slice) → `.10`–`.12`, `.15`–`.17`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** three seats need a named human from the director — project owner, procurement owner, and the
  sewing/factory domain expert, who gates the fixture's `assumed` constants and so the first G2 golden
  (`docs/book/src/governance.md` §8 lists them in one place). **Awaiting his ruling:** the `ROADMAP.md` §11 G3
  amendment four matrix cells depend on, which `.15` puts to him; those cells say `(proposed)` and A3 prints them.
