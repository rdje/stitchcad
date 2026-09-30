# PLANNING — acceptance evidence for completed leaves

The evidence half of [`PLANNING.md`](PLANNING.md), split out under the containment registry's remedy for
`tasks_collection` ("a tree that passes 1000 lines splits its completed-leaf evidence into a sibling file
under `docs/tasks/` before adding more") and under the convention `G0-CONTRACT.4b` recorded, which
`PLANNING.6` applied here when this tree crossed 1 000 lines.

**Why the tree file keeps one checklist.** `scripts/check_task_acceptance.sh` judges EVERY staged
`docs/tasks/*.md` file and refuses one that carries no ticked ROOT CAUSE / ADDRESSED / NO REGRESSION box, so a
tree file emptied of checklists would turn an honest slice into a refusal. The leaf being landed keeps its
checklist in the tree file and the next slice moves it here — which also makes the tree file's FIRST matching
box the current leaf's, closing defect D15's facet 1 by structure rather than by care.

Order is landing order, oldest first, as in the tree's Commit Log. The defect census — this tree's largest
section — stays in the tree file, because it is live state rather than completed-leaf evidence.

### `PLANNING.2` — the engine-stage lanes (`G1-SLICE`, `G2-2D`, `G3-GRADING`, `G4-PROFILES`)

- [x] **ROOT CAUSE (WHY + WHERE)** — after `PLANNING.1` only the G0 lane and the spine lane
  existed, so the roadmap's engine gates had no owner: `ls docs/tasks/*.md` →
  `BOOTSTRAP.md G0-CONTRACT.md PLANNING.md SPINE.md TEMPLATE.md` (5 files, 4 trees), `rc=0`, while
  `grep -c '^### G[1-7] ' ROADMAP.md` → `7` gates and `grep -c '^### V[12] ' ROADMAP.md` → `2`
  parallel tracks are declared in §11, both `rc=0`. (Re-derived when this checklist moved to the evidence
  sibling, so the box still carries output a reader can run rather than output it once carried.)
- [x] **ADDRESSED (verified)** — leaf declarations, `grep -c '^- ID: ' docs/tasks/<tree>.md` →
  `G1-SLICE 17`, `G2-2D 15`, `G3-GRADING 15`, `G4-PROFILES 15` (each count = the tree node plus its
  leaves, i.e. 16 + 14 + 14 + 14 = **58 leaves**). Clause→leaf rows per gate table,
  `awk '/^## Acceptance Criteria/{s=1;next} /^## /{s=0} s&&/^\|/{print}' docs/tasks/<tree>.md | grep -cE '\|[^|]*\.[0-9]+[^|]*\|[[:space:]]*$'`
  → `G1 7`, `G2 8`, `G3 8`, `G4 9`, against roadmap §11's own exit-clause counts for those gates
  (7 / 8 / 8 / 8 — G4's first clause spans two rows, `.2`+`.3` and `.4`), every count `rc=0`, so no clause is
  unowned. (Re-derived when this checklist moved to the evidence sibling: the counts are the historical ones
  this leaf published, and `grep -c '^- ID: ' docs/tasks/G3-GRADING.md` → `16` today because roadmap v0.3 added
  `G3-GRADING.15`, `rc=0`.)
  The same census found three G0 clauses with no leaf (§4.3 CI shape, §4.4 undo/redo defined at G0,
  §7.6 one message system chosen at G0); they are now `G0-CONTRACT.16`/`.17`/`.18`, whose clause
  table carries 19 rows for 18 leaves (`.14` owns both the governance and the procurement clause).
- [x] **NO REGRESSION** — `scripts/check_doctrines.sh` → `=== doctrine enforcement (13 checks) ===`,
  `=== all doctrines green ===`, `rc=0`; `make check` → `test result: ok. 1 passed; 0 failed`;
  the index↔disk census reports no dead link and no unregistered tree.
- [x] **FIX** — added the four tree files; extended `G0-CONTRACT` by three leaves and corrected
  its "no code in G0" reading; added `SPINE.7`/`SPINE.8` to own defect D15; registered all four
  trees in `docs/TASK_TREE.md`.
- [x] **LOCKSTEP** — index, `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and the derived
  Knowledge Map updated in the same commit.

### `PLANNING.3` — the remaining five lanes, and a census that makes the claim re-derivable

- [x] **ROOT CAUSE (WHY + WHERE)** — five of the roadmap's ten lanes had no tree, so their exit
  criteria were owned only by a promise in a seeding leaf. Re-derivable against the commit before this
  one: `git ls-tree --name-only HEAD docs/tasks/ | grep -cE '/(G5|G6|G7|V1|V2)-'` → `0`, `rc=1`
  (no match), while `grep -cE '^### (G[0-7]|V[12]) ' ROADMAP.md` → `10`, `rc=0`; and the loop form
  `for g in G5 G6 G7 V1 V2; do ls docs/tasks/${g}* >/dev/null 2>&1 || echo "$g unowned"; done`
  printed `5` `unowned` lines.
- [x] **ADDRESSED (verified)** — `G5-SHELLS` (14 leaves), `G6-CONFORMANCE` (10), `G7-RELEASE` (7),
  `V1-ASSEMBLY` (7) and `V2-SIM` (6) now exist, each citing its §11 lane in its metadata and mapping
  every exit clause to a named leaf in a clause→leaf table. The capture is proved by a new census
  rather than asserted: `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh` →
  `census: 10 lanes / 13 trees / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`, checking both
  directions (lane→tree, and tree→index+declared lane) plus dead index links. Leaves across all trees:
  `142`. The RED state was observed, not assumed: before the index rows landed, the same census printed
  `5 orphan(s)` and exited `1`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 1 passed; 0 failed`; `make probes` → `7 suite(s) green`;
  `bash -n docs/tasks/artifacts/planning/run_tree_coverage_census.sh` → `exit=0`.
  This commit was **refused twice first, both refusals correct**: the inherited `TASK-ACCEPTANCE`
  judged all six staged leaf files (D15 facet 3) because five brand-new trees were co-staged with a
  `.sh` file — the exact case the layer-C record's rule 6 forbids — so the slice was split into a
  doc-only commit for the trees and this one for the census tool and its owning leaf; and this leaf's
  own `FRESH-ACCEPTANCE-EVIDENCE` check then refused the ROOT CAUSE box for citing commands with no
  result token (`exit=1`), the third time that rule caught its author.
- [x] **FIX** — added the five tree files; wrote the census tool (bash 3.2 compatible — no `mapfile`,
  because the spine must run on whatever bash a platform ships); registered all five in
  `docs/TASK_TREE.md` with the derived-census command and the corrected execution order; added
  `PLANNING.4` to own the D24 sequencing rule.
- [x] **LOCKSTEP** — index, `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md` and the derived Knowledge Map
  updated in this commit. The census is deliberately **advisory** on clause counts and says so in its
  header: more clause rows than roadmap clauses is expected (a tree may split one clause into several
  leaves, as `G5-SHELLS` does with the "full UX spec" list); fewer rows than clauses is the alarm. A
  classifier that guessed at prose meaning would be worse than the side-by-side.

### `PLANNING.4` — the sequencing rule, so the drift is a decision a future session reads

- [x] **ROOT CAUSE (WHY + WHERE)** — the frontier drifted off the product and nothing in the spine
  noticed: `git log --oneline | grep -cE 'leaf (SPINE|PLANNING|BOOTSTRAP)'` → `20` governance slices
  against `git log --oneline | grep -cE 'leaf (G[0-7]|V[12])'` → `0` product slices, `rc=0` for both;
  `ls docs/book/src/spec/` → `index.md` alone and `git ls-files 'crates/*'` → the bedrock starter crate,
  `rc=0`. Each individual slice was defensible — every one closed a real defect with evidence — which
  is exactly why the pattern needed a rule rather than more care.
- [x] **ADDRESSED (verified)** — `docs/decisions/decision_product-work-takes-the-frontier.md` now
  states the rule (product takes the frontier; spine work only when it blocks, when a defect is live,
  or when the director asks), the symptom to watch (a run of commits none of which touches the
  product), and the two-command census that measures it. It is indexed
  (`grep -c product-work-takes-the-frontier docs/decisions/INDEX.md` → `1`, `rc=0`) and wired into the
  bootstrap every agent reads (`grep -c product-work-takes-the-frontier CLAUDE.md` → `1`, `rc=0`), so
  the next session inherits the priority instead of rediscovering the spine.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make check` →
  `test result: ok. 1 passed; 0 failed`; `bash docs/tasks/artifacts/planning/run_tree_coverage_census.sh`
  → `census: 10 lanes / 13 trees / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `exit=0`.
- [x] **FIX** — added the decision record; wired it from `CLAUDE.md`'s non-negotiables and the layer-C
  index; closed D24 in the defect census; set this tree's frontier to closed and moved the repository's
  frontier to `G0-CONTRACT`.
- [x] **LOCKSTEP** — D24 marked closed in the census below; `LIVE_STATUS.md`, `MEMORY.md`,
  `CHANGELOG.md` and the derived Knowledge Map updated in this commit. This tree records no further
  work: the roadmap→tree mapping is complete and derived.

