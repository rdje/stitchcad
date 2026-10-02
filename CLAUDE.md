# Agent bootstrap (read this first — whatever AI / harness you are: Claude Code, Codex, Gemini, Cursor, Aider, a custom runner, …)

This repository is built on a **portable discipline spine**: durable memory, task-tree
tracking, a strict commit workflow, and mechanical doctrine enforcement. The spine is
enforced at the **git level** (hooks + CI), so it holds regardless of which agent or
human is working. Follow it exactly.



1. Read `README.md` — project objective, layout, standard commands.
2. Read `MEMORY_ARCHITECTURE.md` — the durable 4-layer memory model (**MANDATORY**;
   defines how nothing important is lost across sessions, machines, or harness switches).
3. Read `TOOLBOX.md` — the tools-first doctrine. For ANY unknown / failure / surprising
   result: use or build a diagnostic tool FIRST; never guess a root cause.
4. Read `CLAIM_VERIFICATION.md` — what "checked" means before a number is published: re-derive,
   falsify, make durable; state a missing leg instead of hiding it. Sibling of the next file:
   that one asks *is this rule enforced?*, this one asks *is this number earned?*
5. Read `DOCTRINE_ENFORCEMENT.md` — how every mechanizable doctrine is enforced, and the
   task-acceptance checklist a change MUST pass.
6. Resume from `MEMORY.md` (the bounded layer-A resume pointer) → the active task-tree's
   frontier under `docs/tasks/`.

## The non-negotiables

- **Nothing changes without a task-tree first.** Every code change is owned by a
  task-tree leaf under `docs/tasks/` (index: `docs/TASK_TREE.md`) BEFORE the change is
  made. Track every activity, task, slice, and lane so the project survives a lost session.
- **Record durable facts/decisions** as one-file-per-record notes under `docs/decisions/`
  (index there). Convert relative dates to absolute.
- **Verify a claim three ways before publishing it** (`CLAIM_VERIFICATION.md`): re-derive it by
  command, try to falsify it with an oracle you did not build, and make it durable (tracked
  producer, watched). Cite invocation + output + exit status; name a missing leg rather than
  hiding it.
- **Commit per `COMMIT.md`** after each completed leaf, with the work-unit id in the
  subject. A code change must pass the `TOOLBOX.md` / `DOCTRINE_ENFORCEMENT.md` acceptance
  checklist (root cause + addressed + no regression) in its task leaf.
- **Activate the hooks once per clone:** `git config core.hooksPath .githooks`. The
  pre-commit hook runs `scripts/check_doctrines.sh` (the general enforcer); CI runs the
  same. These are git-level and harness-agnostic.
- **Product work takes the frontier.** A spine/governance/hygiene slice is legitimate only when it
  blocks the product slice about to be taken, when a defect can destroy or corrupt work now, or when
  the director asks. Log every defect you find; do not let logging become scheduling
  (`docs/decisions/decision_product-work-takes-the-frontier.md`). The drift census is two commands:
  `git log --oneline | grep -cE 'leaf (SPINE|PLANNING|BOOTSTRAP)'` against
  `git log --oneline | grep -cE 'leaf (G[0-7]|V[12])'` — if the first grows and the second does not,
  re-sequence before taking another slice.
- **Keep the roadmap, the code, and the docs (README + mdBook) aligned** — locked
  together, no drift, for past, present, and future changes.
- **No background job at a handoff point.** Before you end a session (`/exit`, a pause, a
  handover), run `bash scripts/check_handoff.sh` with OS-visible process access and make it print `handoff: OK`:
  a job that outlives its session rewrites tracked files under the next one, with its log
  gone. Stop owned stragglers AND their children — a parent's death does not propagate.
  If no CUA call/result is pending, `--idle-cua` attests that state; only paired idle runtime metadata
  without project file handles becomes advisory. Denied/empty census refuses: rerun with OS visibility,
  never infer absence. The inherited neutral checker is retained as scaffold provenance, not this
  project's authoritative handoff entry point.
- **A commit message ends with its own last line** — no agent/tool attribution trailers
  (`COMMIT.md`); the `commit-msg` hook refuses them.

> One rule above all: **information that exists only in the live conversation is not yet
> saved — route it to a layer and commit it before the turn ends.**

## First time in a fresh clone

Run `scripts/bootstrap.sh` once. It sets the project name, installs the git hooks, and
seeds the first task-tree from `ROADMAP.md`. Then drop your real roadmap into `ROADMAP.md`
and grow the project from there.
