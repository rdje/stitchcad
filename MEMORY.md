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

- **Project:** StitchCAD — a sewing CAD with a headless Rust core: construction-recipe designs, versioned
  evidence-bearing Factory Profiles, deterministic export, agent-first via MCP. `ROADMAP.md` v0.2, DRAFT
  until G0 exits.
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.8`. Done: `.1`–`.7` (glossary 275 terms,
  units, ontology, envelope 105 rows, instantiation paths, size sets, standards registry),
  `.13`/`.13b`/`.13c` fixture, `.18` `sc-units` + `sc-core` + G0 CI.
- **Next action:** `.8`, ADR-0001 — one layer-C record coupling license and solver: permissive core,
  `slvs`/GPLv3 rejected, `sc-sketch` on a custom or least-squares kernel, the BSL-1.1 distinction, what it
  costs contributors, and the re-open condition (a strong case from the G1 spike). Commit
  `STITCHCAD-G0-0008`.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement), and
  owns **D27** — the fixture's waistband is two garments at once (§4 vs §6). `.15` owns **D32**: five
  envelope rows (collar, trousers, buttons, pockets, fly) no gate's exit criteria prove.
