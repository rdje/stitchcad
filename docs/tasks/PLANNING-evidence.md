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


## Original closed D372–D375 ledger before D380

## D372 — large book-search index warning

Fixed0818;[whole cause/prior checkpoints](../history/window29.md#stitchcad-devnotes-part993md),[whole closure/proofs](../history/window29.md#stitchcad-devnotes-part994md).

## D373/D374 — active journal documentation scope/storage drift
Fixed0696;[whole original ledger/proofs](../history/window28.md#stitchcad-devnotes-part935md).

## D375 — drafting chapters missing from the topic index
Fixed0760;[whole cause/reproduction/fix](../history/window29.md#stitchcad-devnotes-part942md).


## D380 — cache-inclusive occupancy mistaken for used RAM

Owner:G1-SLICE.5f.2.dependencies.semantic.binding.garment.relations.closures.placements.references.memory-accounting;P1 before Rust admission.
Top23G occupied was wrongly treated as >95% used RAM;Activity Monitor reported20.44/24GB with3.60GB reusable cache. Captures are not simultaneous;no exact UI/CLI parity claim.
Fixed by explicit cache-adjusted estimate/pressure/headroom diagnostic,frozen independent subtraction and watched refusal/threshold controls. Book/resume/incorrect blocker corrected;original captures retained as history.

## D381 — dependency-plan probe inherited ambient Rust stores

Owner:G1-SLICE.5b.4c.h2.b.p.inventory-reconcile;P1 before push. plan_sources.py passed ambient CARGO_HOME/RUSTUP_HOME
to Cargo when run without the launcher;only its scratch was guarded. Fixed0966:shared enter_producer before reads/children.
Fake-Cargo capture now receives target/cargo-home;linked output refuses rc=2;real probe rc=0. D156 runtime signoff stays open.

## D382 — changelog pointer rule outlived the live index

Owner:G1-SLICE.5f.2.prepush-400.ledger-pointer;P1 before push. 0779 moved the index into record954,but
the POINTER rule kept reading live table rows only and stayed red until the next make probes. Fixed0970:a live row
links record954 and the rule follows only that record;306 listed parts equal306 sealed rows by independent grep.

## D383 — make probes could hide a silently stopped suite

Owner:G1-SLICE.5f.2.prepush-400.probe-reporting;P1 before push. The recipe kept one bit,and a set -e suite that
stops prints no failure line:formula_structure skipped its later steps behind a stale copied chapter. Fixed0971:
failing paths are named;recovery documented. The fail-closed copied-book guard is intentionally unchanged.

## D384 — book publication drift hidden until pre-push

Owner:G1-SLICE.5f.2.prepush-400.book-publication;P1 before push. Two chapters lacked index rows since0871/0874;
my 0968 link and 0971 marker broke publication while the doctrine gate stayed green;G0-19 tested output before
building it. Fixed0972:publication suite50/0,G0-19 met. Recurrence guard stays with .api-declarations.

## D385 — refactor outran its fault tools

Owner:G1-SLICE.5f.2.prepush-400.mutation-anchors;P1 before push. 0836 moved namespace collision logic,stranding9 fault
anchors;a fixed example count of4 broke as the book reached14. Both sat behind a set -e stop. Fixed0974:17+18 compiled
reds and14 executed examples. Fault anchors are still only exercised by probes;pre-push remains their proving ground.

## D386 — book censuses ran only before a push

Owner:G1-SLICE.5f.2.prepush-400.api-declarations;P1 before push. C1 passed at0655 and reached794 undeclared name/chapter
pairs,invisible because make probes runs once per400 commits. Fixed0975:29 declaration tables and BOOK-CENSUSES at
every commit. It cannot judge a meaning's truth;rendered checks stay in the pre-push publication suite.

## D387 — no census watches distance to the gate

Decided by the director2026-11-06:input checks on hold,all in on features. The drift census counts product commits,so654
commits at G1 6/18 passed it while no drafting operation,solver,store,renderer or export existed.

## D388 — parallel plan beside the roadmap

Owner:G1-SLICE.thin-path;found by the director2026-11-06. 0977 opened six leaves restating owners the tree already had,with
weaker acceptance;0976's book table replaced the roadmap's seven clauses with my own list,skipping `.11` and adding `.5f.3` and `.10`.
Neither was checked against ROADMAP.md §11 or G1-SLICE.md's exit table. Fixed0978:owners restored,table matches the clauses.

## D390 — checks outlived what they watch

Owner:G1-SLICE.5f.2.prepush-400.stale-contracts. ci_environment last ran whole at0564,so six checks drifted unseen:a new script
without an inventory row,stale ledger,ordered-name and recipe populations,and a lost MEMORY line. One was a real fault of mine:0974
read a chapter before activation,which the producer contract forbids. Fixed0980 with every check kept;the count is pinned and checked.

## D392 — weak G0 checks

Found 2026-11-06 reading the exit review while building the gauge. G0-04,05,06,08,13 and 17 pass on test -f or grep -q of one phrase,so
a chapter could be emptied and its clause would stay met. Six of the 18 met clauses prove less than the count suggests. The review's own header discloses this;this entry owns the repair.
Fixed0985,leaf G1-SLICE.16.g0-checks:each row now checks what its clause asks,and G0's verdict stays 18 met,1 not met. These read structure and size,so they still cannot tell whether a specification's words are right.

## D391 — one look blamed a passing reader

Owner:G1-SLICE.5f.2.prepush-400.census-patience. The handoff census took one snapshot,so iTerm's status-bar read of the checkout,or a
process dying under lsof,could fail a two-hour probe run at its last step. Measured on this machine:one look blocked71 and refused4 of467.
Fixed0981:only the same process on every look across about a minute blocks. No name is exempted and a real held file still blocks.
