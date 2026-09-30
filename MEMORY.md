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
  Factory Profiles, deterministic export, agent-first via MCP. `ROADMAP.md` v0.3, DRAFT until G0 exits.
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.8`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.4c`, `.14`, `.14b`,
  `.14c`, `.18`, `.19`. Remaining: `.9`–`.12`, `.15`–`.17`.
- **Next action:** take **`G0-CONTRACT.9`** — ADR-0003 (construction recipe primary) plus the formula
  language v1: grammar, units inside expressions, conditionals, name binding, evaluation order, error and
  dimension rules, worked over the reference skirt, and the drafting system that ships as the reference block
  set. A big chapter, so it is a whole slice. Then `.10`–`.12`, `.15`–`.17`. The ruling's four items and the
  three findings are all landed; `ROADMAP.md` is at **v0.3** and the table convention is in `COMMIT.md`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** one seat is **vacant** and cannot be acted — the sewing/factory domain expert, who gates the
  fixture's `assumed` constants and so the first G2 golden. The project owner's and procurement owner's seats
  are held acting by the director. All three are the director's to name (`docs/book/src/governance.md` §8, with
  §8.2 stating the ask per seat).
