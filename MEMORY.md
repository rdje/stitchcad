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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.5`. Done: `.1` glossary (265 terms),
  `.2` units, `.3` ontology, `.4` envelope matrix (105 rows, 29 diagnostics), `.13`+`.13b` fixture,
  `.18` `sc-units` + `sc-core` + G0 CI.
- **Next action:** write `.5`, both instantiation paths — `docs/book/src/spec/instantiation-paths.md`:
  regeneration against `.rul` grade rules, where they diverge (the known information loss), the declared
  equivalence tolerance, extreme-size checking after reconstruction, and the three `.rul` attributes the
  glossary leaves unspecified (`stack point`, `fixed perimeter`, `smoothing`). Commit `STITCHCAD-G0-0005`.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement), and
  owns **D27** — the fixture's waistband is two garments at once (§4 vs §6). `.15` owns **D32**: five
  envelope rows (collar, trousers, buttons, pockets, fly) no gate's exit criteria prove.
