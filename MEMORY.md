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

- **Project:** StitchCAD — a sewing CAD with a headless Rust core: construction-recipe
  designs, versioned evidence-bearing Factory Profiles, deterministic artifact export
  (DXF/HPGL/PDF/tech pack), agent-first via MCP. `ROADMAP.md` v0.2, DRAFT until G0 exits.
- **Active tree:** `SPINE` → frontier leaf `SPINE.1` (`pending`). `PLANNING.3` is parked mid-tree
  (repo clean) and resumes after the `SPINE` identity/hygiene leaves.
- **Next action:** de-template the identity surfaces (defects D3, D4) — rewrite `README.md` as the
  StitchCAD landing page inside the reviewed caps, set `docs/book/book.toml` title/authors, replace
  `docs/book/src/introduction.md`, and grow `SUMMARY.md` with the `spec/` part that `G0-CONTRACT`
  fills; verify with `mdbook build docs/book` and by running the README quick start; then commit
  `STITCHCAD-SPINE-0001 (leaf SPINE.1)`.
- **Order after that:** `SPINE.2` (artifact cleanup + `docs/ARTIFACT_CLEANUP.md`) → `SPINE.3` (policy
  adoptions) → `SPINE.4` (live-doc containment) → `SPINE.5` (toolbox/knowledge map) → `PLANNING.3`
  (G5–G7, V1, V2 + coverage census) → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner,
  evaluation-seat/plotter procurement). It blocks only itself.
