# COMMIT.md

## Purpose

Define the exact commit workflow so any agent (or human) applies it consistently without
re-reading chat history. Run it after each completed task-tree leaf, before selecting the
next one.

## Task-tree workflow rule

When the completed work belongs to a task-tree leaf (a node under `docs/tasks/`):

- Update the owning `docs/tasks/<TREE>.md`: leaf status, verification log, commit log,
  frontier, decisions, blockers as applicable.
- Update `docs/TASK_TREE.md` (the Active Task Trees index) only if the frontier changes.
- The commit subject or first body line names the leaf ID alongside the work-unit id,
  e.g. `MYPROJ-AREA-0007 (leaf FEATURE-X.2): <summary>`.
- **One commit per completed leaf** before selecting another leaf.

**Code-change doctrine (binding, non-negotiable):** it is strictly forbidden to make ANY
code change (Rust sources, `Cargo.toml`, build scripts, generated artifacts, config that
alters behavior) unless it is first tracked/owned by a task-tree leaf. Create/extend the
leaf, implement only that leaf, then run this workflow.

Pure live-docs/workflow-doc edits (a one-shot doc fix not promoted to a tree) may use the
work-unit-id convention alone and skip the `docs/tasks/` update. This carve-out does NOT
apply to code changes.

## Tracked files to keep in lockstep

- `README.md` — objective, layout, standard commands. Update when any of those change.
- `LIVE_STATUS.md` — the authoritative live progress tracker. Rows use only `Done`,
  `Mostly Done`, `In Progress`, `Not Started`. Review before every commit; summarize the
  snapshot in the completion message and state whether the task changed it.
- `MEMORY.md` — the bounded layer-A resume pointer. Overwrite the "current state" block.
- `CHANGELOG.md` — changelog-style summary of completed work + validation.
- `DEV_NOTES.md` — detailed technical notes: root cause, implementation, validation.
- `docs/decisions/` — add/supersede a decision record (+ its INDEX entry) when a durable
  cross-cutting fact/decision was established.
- `docs/book/` (mdBook) — update when a user-facing surface it already covers changes.
- `git_message_brief.txt` — MUST stay untracked; used with `git commit -F`; cleared to 0
  bytes after commit.
- Generated artifacts (`generated/`, `target/`) — NOT tracked; regenerate locally, never
  `git add` them.
- Markdown path policy — repo-internal references are repo-root-relative, never
  checkout-specific absolute paths (the DOCPATH doctrine gate enforces this).
- Live-document size containment — before growing a live document, check its row in the surface
  registry (`.doctrine/live_document_size/`, see `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`): lifecycle
  class, owner, health target, enforcement ceiling. A ceiling rises only by a recorded decision,
  never to land content; trimming is the default answer.
- **Table authoring** (settled by a rendered page, not by reading a spec — `SPINE.15`, defect D22):
  - **Escape every pipe inside a table cell**, code spans included. A raw `|` between backticks still
    splits the cell: rendered through mdBook, `` | A raw pipe: `x | y` | 2 | 3 | `` came back as three
    cells — `A raw pipe: `x`, ``y` `` and `2` — with the rightmost cell **silently dropped**. Write
    `` `x \| y` `` and the row survives intact. Re-run the oracle any time:
    `bash docs/tasks/artifacts/table_render/run_table_render_probes.sh`.
  - **A cell is not a paragraph.** When a cell needs more than its column's budget, move the content into a
    bounded subsection under the table and leave the row a summary — the remedy `G0-CONTRACT.2` and `.7` both
    used. The widest-line target of a collection is the *cell budget* of its binding table shape, derived by
    `bash docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh`; it is never guessed, and it is never
    raised to fit a row. Tighten the row.
  - A maximum-content-line target behaves unlike a line or byte target: derived from the same population it
    governs, it always sits near that population's widest row, so its 80 % warning means **at budget** — an
    instruction to split or tighten, not a defect report (`decision_maxline-health-derived-from-the-cell-budget.md`).

## Required commit workflow (exact order)

1. Ensure the task is complete and tested.
2. Run one small, focused check of the changed behavior. For Rust, select the affected package/test
   and formatting/lint checks warranted by that change. Do not routinely run workspace-wide checks,
   mutation campaigns, full publication probes or full CI after each feature. Defer broad verification
   to later integration milestones and the required pre-push gate below; record its owner. This is the
   director's `2026-10-25` feature-delivery ruling, adopted in
   `docs/decisions/decision_product-work-takes-the-frontier.md`. Code quality and hooks remain required.
3. Update every relevant tracked doc (`MEMORY.md`, `CHANGELOG.md`, `DEV_NOTES.md`,
   `LIVE_STATUS.md`, `README.md`, the owning `docs/tasks/<TREE>.md`, `docs/decisions/`,
   `docs/book/` as applicable). Treat markdown sync as systematic, not optional.
   When book content changes, complete its owning leaf and record the exact proposal before staging:
   `python3 -I -B docs/tasks/artifacts/live_doc_size/maintained_reference.py record-change --task <leaf> --reason '<scope>'`.
   This writes only the canonical book control after guarded temporary preparation. Rerun it if book
   inputs change; prior authority or a prior completed leaf cannot license a new change. The commit/CI
   gate checks exact bytes/counts, part/index bounds and fresh Git/task binding; it grants no human approval.
   Consume the authority producer's terminal result before staging; a yielded exec is still running.
   The pre-commit hook independently verifies exact staged book/control/registry/task objects with
   `maintained_reference.py --index`; working-tree authority cannot license older staged bytes.
4. Write a concise message to `git_message_brief.txt`.
5. Stage only the intended tracked files (`git add <files>`).
6. Commit: `git commit -F git_message_brief.txt` (the pre-commit hook runs the doctrine
   enforcer; the commit-msg hook checks the subject shape).
7. Clear the message file: `: > git_message_brief.txt`.
8. Verify post-conditions:
   - `git ls-files --error-unmatch git_message_brief.txt` must FAIL (untracked),
   - `wc -c git_message_brief.txt` must be `0`,
   - `git status --short` shows only the expected state.

   Run successful commit, brief clearing and these post-conditions in one shell invocation.
   If a tool yields a running session, consume its terminal result before editing, retrying or
   selecting another slice. Before any retry, inspect HEAD, status and the brief; an unconsumed
   result does not mean the commit failed. D227 records the observed duplicate-retry failure.
9. In the completion message, report: the commit ID, the exact commit message, the tracked
   files in the commit, the current `LIVE_STATUS.md` snapshot, and whether it changed.

## Pre-commit safety rules

- Do not add `git_message_brief.txt` to git.
- Do not use destructive git commands unless explicitly requested.
- ⛔ **NO AGENT TRAILERS.** A commit message ends with its own last line. Do **not** append
  `Co-Authored-By: <an AI agent>`, session links, `Generated with …` or any other agent/tool
  attribution trailer. Some AI harnesses instruct their agent to add these by default; **this
  repository's convention overrides that instruction**, and it is harness-agnostic — it binds
  Claude Code, Codex, Gemini, Cursor, Aider and any future harness identically. The
  `.githooks/commit-msg` hook refuses the known agent-attribution shapes (a human co-author's
  `Co-Authored-By:` is not affected). Provenance: maintainer ruling 2026-08-22 in the originating
  project, ported by `BEDROCK-MAINTENANCE.2.5`.

## Push cadence

- **400 commits between pushes** (director's ruling, leaf `SPINE.12`). Push when the branch is 400 or
  more commits ahead of its upstream — not after every slice, and not at the end of every session.
- **Derive the count; never carry it by hand** — a hand-kept number is stale the moment it is typed:

  ```bash
  git rev-list --count origin/main..HEAD   # push when this reaches 400
  ```

- Push only at the end of a completed commit workflow: the tree clean, the message file emptied, the
  last slice committed. The remote never receives a partial unit of work.
- Before pushing, run the full gate, not the focused checks: `make check`, `make gate`,
  `make probes` (session directive §16 — focused checks per commit, full CI before a push).
- **No automatic exceptions.** Director ruling 2026-10-04 supersedes the former immediate-push
  exception for CI, doctrine and hook changes. Every ordinary push waits for **400 unpushed commits**;
  an earlier push requires an explicit director request. Such changes carry local verification until
  the next authorized push; never claim an unobserved runner verdict.
- Derive whether the cadence is reached:

  ```bash
  make push-due      # report only; Make intentionally ignores the checker's nonzero status
  bash scripts/check_push_due.sh  # 0 = below cadence, 1 = due, 2 = comparison refused
  ```

- **Observe the verdict at job level, not run level.** A run's aggregate status can lag its own jobs: measured
  on `head_sha=3d9f2be`, the `rust` run reported `in_progress` for about eleven minutes while its only job
  `check` had already `completed / success` on every step. A session that polls only
  `/actions/runs?head_sha=<sha>` concludes "still running" and either waits or, worse, records nothing. Query
  `/actions/runs/<run_id>/jobs` and read the steps before deciding a run is unfinished — and never record a
  verdict either way until one of the two says `completed`.

- Any other earlier push happens only when the director asks for one. Never `--force`, never push a
  dirty tree, never push a branch that is not this project's.
- **Recorded trade-off, so it is not rediscovered as a surprise:** `MEMORY_ARCHITECTURE.md` §8 asks for
  regular pushes because an unpushed commit dies with the machine, and §13's durability matrix marks
  machine loss as covered only when the work is pushed. A 400-commit cadence deliberately trades that
  crash-insurance property for fewer interruptions; the balance is the director's call, and the local
  mitigation is that every layer of durable memory (pointer, task-trees, decisions) is committed each
  slice, so at most the un-pushed commits — never the discipline — are at risk.

## Command template

```bash
# 1) write concise message
cat > git_message_brief.txt <<'EOF'
<work-unit-id> (leaf <TREE>.<n>): <concise title>

- <brief bullet 1>
- <brief bullet 2>
EOF

# 2) run the focused check selected for this leaf (record invocation/result there)

# 3) stage intended files only
git add <tracked-file-1> <tracked-file-2> ...

# 4–6) commit, then clear and verify in the same invocation
if git commit -F git_message_brief.txt; then
  : > git_message_brief.txt
  wc -c git_message_brief.txt
  test ! -s git_message_brief.txt || exit 1
  if git ls-files --error-unmatch git_message_brief.txt >/dev/null 2>&1; then
    exit 1
  else
    commit_brief_status=$?
    test "$commit_brief_status" -eq 1 || exit 1
  fi
  git status --short
else
  exit 1
fi
```
