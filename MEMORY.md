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
- **Active tree:** `G0-CONTRACT`. Done: `.1`–`.10`, `.13`/`.13b`/`.13c`/`.13d`, `.4b`, `.4c`,
  `.14`/`.14b`/`.14c`, `.18`, `.19`. Remaining: `.11`, `.12`, `.15`–`.17`.
- **Next action:** take **`G0-CONTRACT.11`** — ADR-0002 as a decision *structure*: the chrome choice, the
  three canvas topologies, the egui/iced dev-shell ruling, the TypeScript domain-logic ban, and the exact G1
  spike protocol (measurements, pass/fail criteria, decision rule) written now so the outcome cannot be
  argued after the fact. Then `.12`, `.15`–`.17`. ADR-0003 landed at `.9` (the formula language in three
  censused parts, Aldrich named as the reference drafting system, `G3-GRADING.16` owning the blocks) and
  ADR-0004 at `.10` (six axes, a closed registry of four targets, the layer table derived against
  `ROADMAP.md`).
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
