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
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.8`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.18`. Remaining:
  `.9`–`.12`, `.14`–`.17`.
- **Next action:** take **`.14`** — the governance model drafted in full (sewist-vs-programmer review paths,
  domain review of profile changes that alter exported bytes, golden-file approval ownership, procurement
  and its documented fallback); only *naming humans* stays blocked. Then **`SPINE.4.4`** (maxline health for
  table-shaped book parts) → **`.9`** (ADR-0003 + the formula language v1: a big chapter, whole slice) →
  `.10`–`.12`, `.15`–`.17`.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement owner,
  domain reviewer) and nothing else. **Awaiting his ruling:** the `ROADMAP.md` §11 G3 amendment four matrix
  cells depend on (`decision_d32-proving-gates-proposed-roadmap-amendment.md`), which `.15` puts to him;
  until then those cells say `(proposed)` and the census's A3 advisory prints them on every run.
