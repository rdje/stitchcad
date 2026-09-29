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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.13` (`pending`). Done: `.2` (units &
  tolerances spec), `.3` (garment ontology spec), `.18` (`sc-units` + `sc-core` + G0 CI).
- **Next action:** specify the **reference skirt** with real numbers —
  `docs/book/src/spec/reference-skirt.md`: an A-line skirt, one waist dart per side, CB zipper, grain
  parallel to CB, 1 cm side seam allowances and a 3 cm hem, single notches at the side seams, and a
  decided cut-on-fold vs paired-front choice with its reason. Give the measurement table, the formulas
  that draft it, the piece list, the sewing graph, notch and grainline placements, and the allowance
  policy it exercises — enough that a reader could draft it without asking a question. It is the corpus
  fixture for G2, so every number needs a source. Commit `STITCHCAD-G0-0013 (leaf G0-CONTRACT.13)`.
- **Then:** the remaining G0 chapters, in the order `docs/tasks/G0-CONTRACT.md`'s frontier table gives.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
