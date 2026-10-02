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
| [`PLANNING`](tasks/PLANNING.md) | roadmap → tree mapping (all lanes) | `active` | `.5` — derive the index↔tree frontier agreement (D34, which recurred on `2026-09-30`) | repo-local |
| [`SPINE`](tasks/SPINE.md) | repository identity, hygiene, adopted policy | `active` | `.5`/`.13`/`.19`/`.22` guarded; .23/.23v handoff verified; product .5e.3b; .5b.1c.2 awaits D124 | repo-local |
| [`G0-CONTRACT`](tasks/G0-CONTRACT.md) | §11 gate **G0** — product & semantic contract | `done` | no further leaf — `18 met / 1 not met` by `run_g0_exit_review.sh`, closure unapproved (§6.1) | repo-local |
| [`G1-SLICE`](tasks/G1-SLICE.md) | §11 gate **G1** — executable architecture slice | `active` | `.5e.3b` — D121 reference provenance; .5b.1c.2 awaits D124; axes D70 pending | repo-local |
| [`G2-2D`](tasks/G2-2D.md) | §11 gate **G2** — correct 2D slice (vertical proof) | `proposed` | `.1` — `sc-geometry` 2D kernel | repo-local |
| [`G3-GRADING`](tasks/G3-GRADING.md) | §11 gate **G3** — construction & grading | `proposed` | `.1` — dart/tuck/pleat/gather closure semantics | repo-local |
| [`G4-PROFILES`](tasks/G4-PROFILES.md) | §11 gate **G4** — profiles & uncertainty workflow | `proposed` | `.1` — `sc-profiles` schema v2 | repo-local |
| [`G5-SHELLS`](tasks/G5-SHELLS.md) | §11 gate **G5** — application shells & validated 2D UX | `proposed` | `.1` — Tauri native shell (Win/macOS/Linux) | repo-local |
| [`G6-CONFORMANCE`](tasks/G6-CONFORMANCE.md) | §11 gate **G6** — conformance lab & reliability | `proposed` | `.1` — DXF importer + honest loss report | repo-local |
| [`G7-RELEASE`](tasks/G7-RELEASE.md) | §11 gate **G7** — scoped production declaration | `proposed` | `.1` — independent evidence review | repo-local |
| [`V1-ASSEMBLY`](tasks/V1-ASSEMBLY.md) | §11 track **V1** — assembly visualization (parallel) | `proposed` | `.1` — `sc-mesh` triangulation (post-G3) | repo-local |
| [`V2-SIM`](tasks/V2-SIM.md) | §11 track **V2** — physically validated simulation (parallel, uncapped) | `proposed` | `.1` — `sc-sim` outside the default build | repo-local |
| [`BOOTSTRAP`](tasks/BOOTSTRAP.md) | one-time de-template from bedrock | `done` | — | repo-local |

All ten roadmap lanes are owned, **derived rather than asserted**:

```bash
bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh   # → 10 lanes / 13 trees / 10 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)
bash docs/tasks/artifacts/planning/run_tree_coverage_probes.sh   # → probes: 7 pass / 0 fail — watched, because a census no gate runs can go red unnoticed (D44)
```

The census checks both directions — every §11 lane has a tree whose metadata names it, and every tree on disk
is registered here with a declared lane — plus the advisory clause-versus-leaf table. A file under
`docs/tasks/` that is not a tree (no `- Tree ID:` line) must be linked from one: that is how the evidence
siblings the containment registry prescribes are told from strays. In the advisory table, more clause rows
than roadmap clauses is expected (a tree may split one clause into several leaves); fewer is the alarm.

Execution order right now: **`G1-SLICE.5e.3b`** (independent D121 reference irrational-result provenance; .5b.1c.2 awaits D124; axes .4c.2 awaits D70) and the lane that follows it. Gate G0 has no further leaf:
its review is derived (`bash docs/tasks/artifacts/g0_exit/run_g0_exit_review.sh` → `18 met / 1 not met /
19 clauses`), the one open clause travels with the director's ruling of `2026-09-30` that accepts it, and the
gate's closure stays unapproved under governance §6.1 because its reviewer authored most of what it reviews.
Take the three unproven contracts early in G1 — the formula evaluator against its reference oracle, the
canvas spike against its protocol, one CSP constraint — rather than the plumbing. `SPINE` keeps `.5`, `.13`
and `.19` open; .19.2 retains exact history bytes in bounded windows; otherwise product work leads. The frontier cells above are hand-kept and have
drifted four times (defect D34); `PLANNING.5` derives them. The ruling of `2026-09-30` delegated four items — D27, D32, `.14`'s drafting and the
containment derivation — and a second instruction delegated its three findings; all are landed, `ROADMAP.md` is
at **v0.3** carrying G3's envelope-coverage criterion, and what remains of the ruling is the director's alone:
naming the three humans governance §8 lists.
