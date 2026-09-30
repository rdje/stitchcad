# The scaffold updater classifies files: neutral is synced, project content is guarded

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `SPINE.9`; established by measured probe output
  (defect D17 in `docs/tasks/PLANNING.md`)

answers: "is it safe to run scripts/update_scaffold.sh?" · "will a spine update overwrite my task-tree index?" · "which files does the scaffold sync touch?" · "why did the updater skip docs/TASK_TREE.md?" · "how do I pull a new spine version without losing project content?"

## The fact / decision

`scripts/update_scaffold.sh` sorts every file it can sync into two declared classes:

Two classes, and the difference is whether the template itself asks the project to write into the file:

- **NEUTRAL** — carries no project content by construction, so it is overwritten freely with the previous
  copy backed up: the universal check scripts, the hooks, `MEMORY_ARCHITECTURE.md`,
  `docs/TASK_TREE_README.md`, `.doctrine/README.md`, the knowledge-map scripts, `DOCTRINE_VERSION`.
- **PROJECT-CONTENT** — the template instructs the project to write into it, so it is backed up, reported and
  **SKIPPED**, and overwritten only with `--force-project-sections`: `docs/TASK_TREE.md` (the Active Task
  Trees index), `TOOLBOX.md` (project toolbox), `README_POLICY.md` (local adoption note),
  `DOCTRINE_ENFORCEMENT.md` (its registry mirror invites project rows), `COMMIT.md` (a named per-project
  knob), `AGENTS.md` (bootstrap pointer), `docs/tasks/TEMPLATE.md`, `docs/decisions/TEMPLATE.md`.

Two further rules: the updater **refuses a dirty working tree** (a sync over uncommitted work hides
both the update and any loss in `git diff`), and `--dry-run` reports the classification while writing
nothing at all — no syncs, no backups, no file modes. Backups and the clone live under `target/`, on
the repository volume.

## Why

The previous single-list version classified all of the above as NEUTRAL, with the comment "safe to
overwrite because it never carries project content" — while `docs/TASK_TREE.md` held this project's
Active Task Trees index (layer-B navigation, the first thing a resuming session reads) and
`TOOLBOX.md` held the project toolbox table its own header asks the project to fill in. One run of
the documented "keep the spine current" command would have replaced both with template blanks. The
contradiction is internal to the spine, not a misreading: `docs/TASK_TREE.md`'s own note tells a
generated project to remove the seeded row and register its own trees.

Measured after the fix (`docs/tasks/artifacts/scaffold_sync/run_update_scaffold_probes.sh` →
`probes: 7 pass / 0 fail`): ARM-2 reproduces the founding situation — local index rows plus a changed
upstream copy — and asserts the rows survive, the report says `SKIPPED`, and a backup exists; ARM-4
proves the dry run writes nothing (whole-tree checksum unchanged); ARM-5 proves a dirty tree is
refused with nothing written; ARM-6 proves `--force` still backs up before overwriting.

## How to apply

- To pull a newer spine: `scripts/update_scaffold.sh <bedrock-url-or-path> --dry-run` first, read the
  classification report, then run it for real on a clean tree, `git diff`, `make gate`, commit.
- A `SKIPPED` line is an instruction to merge by hand, not a failure. Compare the backup under
  `target/scaffold_backup/<run>/` with the upstream copy and take the spine changes into the local
  file, keeping the project sections.
- Adding a new syncable file: classify it deliberately and record the reason in
  `project_content_reason()`. When in doubt, guard it — a skipped update is visible and recoverable,
  a clobbered index is neither.
- Project conventions never go into a PROJECT-CONTENT file *only*: they also belong in a layer-C
  record, which the updater does not touch at all. See
  [[decision_acceptance-evidence-per-leaf]].
