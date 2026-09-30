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
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.8`, `.13`/`.13b`/`.13c`, `.18`. Remaining: `.9`–`.12`,
  `.14`–`.17`, plus `.13d` and `.4b` below.
- **Next action:** the director's ruling of `2026-09-30` delegates four items to the engineer at signoff
  grade, external research permitted — read `decision_director-ruling-2026-09-30-four-findings.md` (it
  carries the scope and the two reservations), then take **`.13d`** (D27: one waistband construction, every
  number re-derived) → **`.4b`** (D32: a proving gate per row, roadmap amendment as a *proposal*) →
  **`.14`** (governance drafted; only naming humans stays blocked) → **`SPINE.4.4`** (maxline health for
  table-shaped book parts). Then `.9`, ADR-0003 + the formula language v1 — a big chapter, whole slice.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement), and
  owns **D27** — the fixture's waistband is two garments at once (§4 vs §6). `.15` owns **D32**: five
  envelope rows (collar, trousers, buttons, pockets, fly) no gate's exit criteria prove.
