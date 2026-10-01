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
- **Active tree:** `G1-SLICE`. **Gate G0 has no further leaf**: `.15`'s derived review reports
  `18 met / 1 not met / 19 clauses`, the one open clause (evaluation-seat procurement) accepted open by the
  director's ruling of `2026-09-30`, and the gate's **closure unapproved** because governance §6.1 withholds
  approval of a decision's evidence from its author. `ROADMAP.md` stays DRAFT, correctly: one criterion is
  not met.
- **Next action:** take **`G1-SLICE.3c`** — the geometry-bearing object types (ontology §4): `Piece`,
  `SeamSpan`/`SewingGraph`, `Notch`, `Grainline`, `SeamAllowance`, `Dart`/`Tuck`/`Pleat`/`Gather`, `Closure`,
  `Pocket` — structural invariants enforced at construction, the geometric ones carried as a visible
  `DeferredToG2` state (`decision_ontology-invariants-structural-g1-geometric-g2.md`), consuming `.3a`'s
  identity types and `.3b`'s resolution contract. Then take the three unproven contracts early rather than the
  plumbing: the formula evaluator against `run_formula_language_census.sh` as a differential oracle, the canvas
  spike against the protocol in `docs/tasks/artifacts/canvas_spike/` (its `results.tsv` is empty and
  `G1-SLICE.13` fills it), and one CSP constraint. Re-run `bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh`
  for the gate state.

- **Execution order and open defects:** `docs/TASK_TREE.md` (order) and `docs/tasks/PLANNING.md`
  (defect census) — both layer B; not restated here.
- **Push:** 400-commit cadence, **plus** an immediate push whenever an unpushed commit touches CI, a
  doctrine check, `.doctrine/` or `.githooks/` — derive it with `make push-due` (`COMMIT.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** one seat is **vacant** and cannot be acted — the sewing/factory domain expert, who gates the
  fixture's `assumed` constants, the formula chapter's `dart_intake_max`, and so the first G2 golden. The
  project owner's and procurement owner's seats are held acting by the director. All three are the director's
  to name (`docs/book/src/governance.md` §8, with §8.2 stating the ask per seat). Two questions sit with the
  acting project owner: whether blocks derived from a commercial drafting source need a licence clearance
  (`G3-GRADING.16` cannot start transcribing before an answer), and ADR-0003's source procurement.
