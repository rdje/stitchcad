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
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.12`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.4c`,
  `.14`/`.14b`/`.14c`, `.18`, `.19`. Remaining: `.15`–`.17`.
- **Next action:** take **`G0-CONTRACT.16`** — the ONE message system (roadmap §7.6: Fluent **or** ICU, not
  "Fluent or ICU"; if both ends are needed, a designed bridge) plus the externalization architecture: the CI
  lint that fails on an inline user-facing string, the glossary/termbase per language with the safety terms
  first (notch types, the sew/cut aliases factories use in rejection emails), pseudolocalization, and
  locale-independent canonical files (a decimal comma in input ≠ a changed stored meaning). Then `.17` (the
  command layer with undo/redo granularity) and the `.15` exit review, which closes gate G0. ADR-0002 landed
  at `.11` as a decision structure and §9 at `.12`: the release contract's nine manifest fields, six
  acceptance states and tuned policy matrix are compared against the roadmap by an instrument, not restated.
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
