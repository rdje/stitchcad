# Acceptance evidence is per leaf, added fresh in the commit that lands the work

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `SPINE.7`; established by measured probe output,
  not by preference (defect D15 in `docs/tasks/PLANNING.md`)

answers: "why was my code commit refused even though my leaf has a ticked acceptance checklist?" · "which leaf's evidence does the TASK-ACCEPTANCE gate judge in a multi-leaf tree file?" · "may a tree file keep unticked placeholder checkboxes?" · "how do I write acceptance evidence so the gate judges MY leaf?"

## The fact / decision

1. **A task-tree file carries acceptance checkboxes only for leaves whose work has landed.** No
   unticked placeholder boxes for future leaves, and no tree-level "fill in later" checklist.
2. **Each completed leaf adds its own `### <leaf-id>` subsection** under `## Acceptance Checklist`,
   in leaf order, **in the same commit as the work it describes**, with the three hard-gated boxes
   ticked and carrying real tool output inside each box's own bullet.
3. **The inherited gate judges the FIRST bullet matching each label in the file**, not the bullet
   belonging to the leaf being committed. That is a property of `scripts/check_task_acceptance.sh`
   as shipped, measured by
   `docs/tasks/artifacts/task_acceptance/run_multileaf_shadowing_probe.sh`.
4. Therefore the convention in (1)–(2) is **load-bearing**, not stylistic: it is what keeps a
   multi-leaf tree file's verdicts attributable, until the mechanical fix (`SPINE.8`,
   `FRESH-ACCEPTANCE-EVIDENCE` in the project doctrine slot) makes attribution independent of
   section order.
5. **An evidence bullet cites the invocation, its output, and its exit status** (`rc=0`,
   `exit=1`, `test result: ok`, `probes: N pass / M fail`). A listing of filenames or a paraphrase
   is not tool output the gate can recognize, and the refusal it earns is correct.
6. **A code commit stages the leaf that owns it.** Unrelated tree updates go in their own commit:
   the inherited check judges *every* staged leaf file, so co-staging a documentation tree with a
   code change puts that doc tree's boxes under the same signature requirement.

## Why

Measured, both directions, against the shipped check (`probes: 6 pass / 0 fail`):

- **False GREEN (`HOLE-1`).** One file, leaf `TREE.1` ticked with evidence above, leaf `TREE.2`
  unticked below; the staged code change is owned by `TREE.2`. Verdict:
  `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed checklist)`,
  `exit=0`. The gate accepted a change on evidence belonging to a different leaf — the leakage class
  its own header says box-scoping closed, surviving at file scope. The shipped probe suite exercises
  that class across *files* only (`CTRL-2` there: a co-staged unrelated leaf), so the property it
  demonstrates is narrower than the property the header claims.
- **False RED (`HOLE-2`/`HOLE-3`).** An honest leaf with ticked, evidence-backed boxes is refused
  (`the 'ROOT CAUSE' box is present but NOT ticked`, `exit=1`) purely because an unticked placeholder
  for a *future* leaf appears earlier in the same file; delete the placeholder and the identical leaf
  passes (`exit=0`). Attribution, proved by the pair.
- This repository's trees are deliberately multi-leaf (one file per roadmap lane, 87 leaves across
  7 trees at the time of writing), so the exposure is structural here, not theoretical.
- **Facet 3, measured in the wild on the commit that published this record.** With the probe (a
  `.sh` file) staged, the check judged every staged leaf file — including a documentation tree that
  owned no code — and refused it: `❌ TASK-ACCEPTANCE … docs/tasks/PLANNING.md — the 'ROOT CAUSE'
  box is ticked but carries no tool-output evidence`, `exit=1`. The box it read was
  `docs/tasks/PLANNING.md:155`; counting recognized signature families inside that bullet gives
  `0`, although the bullet cites two census commands and their real listings. Two consequences:
  rules (5) and (6) above, and a design constraint on the local fix — a check registered in the
  project slot can only **add** refusals, it cannot relax a universal one, so `SPINE.8` must make
  freshness mechanical while the authoring rules carry the rest.

## How to apply

- When a leaf lands work: add `### <TREE>.<n>` with the checklist, ticked, evidence inside each
  bullet — invocation, output, exit status — and commit it with the change. See
  `docs/tasks/SPINE.md` → `### SPINE.6` for the shape.
- When seeding a tree: list leaves in the `## Task Tree` section (goal / acceptance / verification /
  commit). Do **not** pre-create their checkboxes.
- If a commit is refused and your boxes are ticked: check whether an unticked box for another leaf
  appears earlier in the same file. That is D15, not your evidence.
- Do not "fix" this by editing `scripts/check_task_acceptance.sh`: it is shared spine code, re-synced
  by `scripts/update_scaffold.sh` (see the decision on scaffold sync and project content). Local
  mitigation belongs in `scripts/check_doctrines.project.sh`.
- Re-run the probe after any scaffold update; if the HOLE arms start failing, the defect moved —
  re-point the arms and update their header in the same commit.

Related: [[decision_scaffold-sync-protects-project-content]] · `docs/tasks/SPINE.md` (`.7`, `.8`) ·
`DOCTRINE_ENFORCEMENT.md` (TASK-ACCEPTANCE row).
