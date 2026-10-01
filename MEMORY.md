# MEMORY — resume pointer (layer A; overwrite-only)

Read `CLAUDE.md` and its doctrine pointers, then the active task-tree's frontier.
Latest commit: derive with `git log --oneline -1` (the recording commit invalidates a stored hash).

## Current state

- **Active tree:** `G1-SLICE`, frontier **`.3c.2`** in `docs/tasks/G1-SLICE.md`.
- **Next action:** discharge D55 with full-range topology resolution before implementing sewing spans;
  settle D35's same-piece seam rule, then build `SeamSpan`/`SewingGraph`. `.3c.1`'s structural pieces
  are committed; `.3c` remains open until its four children close. Take the unproven formula, canvas
  and CSP contracts early afterwards (execution order: `docs/TASK_TREE.md`).
- **In-flight uncommitted work:** none.
- **Gate/authority constraints:** G0 closure remains unapproved; `ROADMAP.md` remains DRAFT.
  Domain-expert appointment and the drafting-source licence/procurement decisions remain human acts;
  see `docs/book/src/governance.md` §8 and `docs/tasks/G3-GRADING.md` `.16`.
- **Push:** derive with `make push-due`; cadence and exceptions in `COMMIT.md`.
- **Cleanup:** read `docs/ARTIFACT_CLEANUP.md`; run when its latest entry is over 24 hours old.
