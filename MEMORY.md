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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.3` (`pending`). Done: `.2` (units &
  tolerances spec) and `.18` (`sc-units` + `sc-core` + G0 CI; 30 tests, WASM build green).
- **Next action:** write the garment ontology specification —
  `docs/book/src/spec/ontology.md` from roadmap §3.1 and §4.1: field-level definitions and invariants
  for MeasurementTable, Ease, Design (construction recipe), Piece, SeamSpan/SewingGraph,
  Dart/Tuck/Pleat/Gather, SeamAllowance, Notch, Grainline, Hem/Facing/Lining/Interfacing, Closure,
  Pocket, and the walk/true operations — plus the persistent-identity contract (references to stable
  topological entities, never array indices; split/merge/reverse/delete preserves references or emits a
  visible repair task). Add it to `SUMMARY.md`, verify with `make book`, and commit
  `STITCHCAD-G0-0003 (leaf G0-CONTRACT.3)`.
- **Then:** `.13` reference skirt → `.1` glossary → `.4`–`.12`, `.14`–`.17` (order in the tree).

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
