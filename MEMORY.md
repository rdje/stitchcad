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
- **Active tree:** `G0-CONTRACT` → frontier leaf `G0-CONTRACT.18` (`pending`). `.2` (units &
  tolerances) is done: the numerical contract is normative in the book and recorded in layer C.
- **Next action — the first product code:** create the `sc-units` and `sc-core` crate skeletons in the
  workspace per roadmap §4.3, implement `sc-units` from `docs/book/src/spec/units-and-tolerances.md`
  (i64 µm length, i64 µ° angle, the declared domain limits, half-away-from-zero rounding, single-step
  conversions, the five tolerance classes as distinct types, dimensional errors as typed errors) with
  property tests, and add the G0 CI workflow (fmt / clippy / unit+property / a real
  `cargo build --target wasm32-unknown-unknown` smoketest for those two crates). Retire or repurpose the
  bedrock starter crate per `G1-SLICE.1`'s ownership note. Commit `STITCHCAD-G0-0018 (leaf
  G0-CONTRACT.18)`.
- **Then:** `G0-CONTRACT.3` (garment ontology) → `.13` (reference skirt with numbers) → `.1` (glossary)
  → the rest of G0.

- **Sequencing rule (`docs/decisions/decision_product-work-takes-the-frontier.md`):** product work
  takes the frontier; spine slices only when they block it, a defect is live, or the director asks.
  Census: `git log --oneline | grep -cE 'leaf (G[0-7]|V[12])'` must start growing.
- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence; count with `git rev-list --count origin/main..HEAD` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** `G0-CONTRACT.14` needs named humans from the director (project owner, procurement).
