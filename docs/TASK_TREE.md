# Task-Tree Workflow

This document defines the repo-local task-tree workflow. A step-by-step setup guide is in
[TASK_TREE_README.md](TASK_TREE_README.md). Individual trees live under
[`tasks/`](tasks/); the leaf template is [`tasks/TEMPLATE.md`](tasks/TEMPLATE.md).

## Purpose

Use a task tree when a top-level task is too broad to finish safely as one signoff-quality
slice, or when it is expected to discover subtasks over time. The tree owns the recursive
breakdown, current frontier, acceptance criteria, blockers, decisions, validation, and
completion evidence for one top-level task — so the project survives a lost session and
continuity holds across sessions, machines, and harness switches.

The tree is not a second roadmap. `ROADMAP.md` states the high-level direction; a task tree
owns the disciplined execution of one lane of it.

## Code-change doctrine (binding, non-negotiable)

**It is strictly forbidden to make any code change unless it is first tracked/owned by a
task-tree leaf.** "Code change" = any edit to Rust sources, `Cargo.toml`/build scripts,
generated artifacts, or anything altering behavior. Before touching code, a leaf must exist
that owns the change (create/extend a tree, or add a leaf). The leaf — its goal, acceptance,
verification, and commit — is the unit of traceability. Enforced by
`scripts/check_task_tree_ownership.sh`.

## Leaf lifecycle (statuses)

`proposed` → `pending` → `active`/`in_progress` → `done`. A leaf is `done` only when its
acceptance criteria are met, verification is recorded, and it is committed via `COMMIT.md`.
Mark `blocked` (with the blocker named) rather than leaving a stalled leaf `active`.

## The pivot rule

**Do not pivot to a different task-tree while the repo is dirty.** The repo is
handoff-ready only when the tree is clean (no modified/untracked work except the task-tree
file itself). Finish the current leaf and get the repo clean before switching — even if
asked to pivot immediately. The guarantor of repo integrity holds this line.

## Commit traceability

Each slice uses a work-unit id in the commit subject (e.g. `MYPROJ-AREA-0007`). When the
slice belongs to a leaf, the subject or first body line also names the leaf ID
(e.g. `MYPROJ-AREA-0007 (leaf FEATURE-X.2): …`), so the slice id and the tree node coexist
on the same commit. One commit per completed leaf.

## Active Task Trees

A row appears here only when the tree file exists on disk — the index never links a file
that is absent, and a tree on disk is always registered. Both directions are censused by
the owning leaf (`PLANNING.1`, then `PLANNING.3`'s coverage map).

| Tree | Lane (roadmap source) | Status | Frontier (next leaf) | Owner |
| --- | --- | --- | --- | --- |
| [`PLANNING`](tasks/PLANNING.md) | roadmap → tree mapping (all lanes) | `active` | `.3` — seed `G5`–`G7`, `V1`, `V2` + publish the coverage map | repo-local |
| [`SPINE`](tasks/SPINE.md) | repository identity, hygiene, adopted policy | `active` | `.9` — guard the scaffold updater against clobbering project content (D17) | repo-local |
| [`G0-CONTRACT`](tasks/G0-CONTRACT.md) | §11 gate **G0** — product & semantic contract | `active` | `.1` — glossary of construction terms | repo-local |
| [`G1-SLICE`](tasks/G1-SLICE.md) | §11 gate **G1** — executable architecture slice | `proposed` | `.1` — workspace crate layout (retires the starter crate) | repo-local |
| [`G2-2D`](tasks/G2-2D.md) | §11 gate **G2** — correct 2D slice (vertical proof) | `proposed` | `.1` — `sc-geometry` 2D kernel | repo-local |
| [`G3-GRADING`](tasks/G3-GRADING.md) | §11 gate **G3** — construction & grading | `proposed` | `.1` — dart/tuck/pleat/gather closure semantics | repo-local |
| [`G4-PROFILES`](tasks/G4-PROFILES.md) | §11 gate **G4** — profiles & uncertainty workflow | `proposed` | `.1` — `sc-profiles` schema v2 | repo-local |
| [`BOOTSTRAP`](tasks/BOOTSTRAP.md) | one-time de-template from bedrock | `done` | — | repo-local |

The remaining lanes — gates **G5–G7** and the parallel **V1/V2** tracks — are seeded by
`PLANNING.3`, which also publishes the roadmap → tree coverage map. Until a tree exists, its lane's
requirements are owned by that seeding leaf, so no roadmap clause is ever unowned.

Execution order right now: `SPINE.9` → `SPINE.10` → `PLANNING.3` (G5–G7, V1, V2 + the coverage
census) → `SPINE.1`–`.5` → `G0-CONTRACT.1`–`.18` → `G1-SLICE` …
The two spine-integrity leaves come first because one protects the task-tree index from being
overwritten (D17) and the other keeps scratch on the repository volume (D16); both are cheap and
both would be expensive to discover late.
G0 is a specification gate; its only code leaf is `.18` (the two skeleton crates + CI the roadmap's
G0 CI clause requires).
