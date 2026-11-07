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

## D404 — a true that moved nothing

A reference holds a fraction of its edge and `true_seam` moves corners,so a true on a reference mid-edge or on a mirror would move nothing and report success. Owner:G3-GRADING.3. Found while binding seam ends to edges,and it never shipped:the form is new in 1014.
I first wrote that positions did this too. The test said otherwise:a position moves itself along the line and the walk reports it,so that stands,with a test,and the first reading was wrong. Fixed1014:a true that leaves the seam a defect is refused by name with nothing changed;a test holds it.

## D403 — no colorway on my labels

§6 says a piece's label holds its fabric and colorway. My session wrote the fabric and no colorway,and my judge did not check,because I wrote both from the same reading. An independent agent's first session wrote it and said the fixture names none. The labels now say so,the judge requires it,and the goldens are refrozen.

## D402 — a chapter that could not be drafted from

The reference skirt's chapter promises a reader can draft it without asking a question. The first independent reader had to decide nine things,and it cut the darts out of the boundary where my session marks them,which nothing in the book settled. For a factory that is a different pattern.
§13 now pins each boundary,mark and grainline,with a test holding 22 positions against the drawing. Two later sessions on that book,with no refused call,golden-match.

## D401 — platform math in the geometry

The units chapter says ordinary platform libm is not sufficient,and the recipe obeys it. I wrote the geometry kernel with hypot,atan2,sin,cos and acos anyway,so a golden file could have failed on another operating system and nowhere else. Found reading ahead for the determinism policy. Replaced by sc_units::pure,built only from IEEE-exact operations.
No golden moved,and a census now refuses any new call.

## D400 — a build watching a copy of its source

Cargo hashes a workspace package by its relative path,so a fault harness that copied the workspace and built with the shared target directory overwrote the real sc-units library and fingerprint on 2026-10-30,and cargo then watched the copy. Edits to sc-units rebuilt nothing,with no warning.
Found when a new module was reported missing from its own crate. The copy's sources are byte-identical to HEAD,so earlier results stand. Ten fingerprints cleared. scripts/check_build_fingerprints.sh now runs before the gauge's checks. Hosted CI builds clean and cannot have this.

## D399 — a closure blind to the geometry

The session's waist_closure said `(quarter_hip - ss_suppress) - dart_intake == quarter_waist`,which re-derives the arithmetic. The drawing uses `waist_side`,so building that point wrongly left the closure true:
the D33 fault the chapter warns about,in my own session. Found while writing the mutant for it. The closure now reads `waist_side`.

## D398 — one pattern,two files

The canonical form starts a closed line at its lowest point. Neither writer did,so a pattern as drafted and the same pattern read back wrote different DXF and PDF bytes. The goldens held only because one path was ever tested.
Found when the judge first compared a file with its canonical form,which reported every curved piece as wrong. Both writers now use the canonical first point,and each has a test that rotates a line's start.

## D397 — a corner that knew its direction

Where both edges at a corner named `Envelope`,the join folded over whichever came second,so a piece and its mirror got different corners,4142 micrometres apart at a 45 degree tip. The corpus had four such cases and passed them,because it judged cleanliness and clearance and never symmetry.
The mirror test in tests/metamorphic.rs found it on its first run. Fixed: two meeting edges both naming it have no fold between them,so they miter,as a mitred hem corner does.

## D396 — an export of a stale drawing

`export_pattern` wrote whatever the last `draft` drew. A session that drafted,changed a measurement and exported got the old geometry with the old revision on it,with no complaint. The skirt's own session never did that,so no test ran it;I saw it while reading `change` to build the shell.
Fixed: `change`,`undo`,`redo` and `reopen` clear the drawing,and an export without a new draft is refused by name. tests/shell.rs holds it.

## D395 — band corners against its rectangle

The reference skirt's §7 gives `trim` for the band's short ends and lower edge. Its §5 step 8 and both §4.1 band closures require one rectangle, and a trimmed corner shortens the edges those closures measure. Both had been read as binding since G0 because nothing drew the band.
Drawing it found this. Resolved in favour of the rectangle,with a note under §7's table and the choice listed in §13 for a domain reviewer. Corner shapes stay an engineer's until that seat is filled.

## D394 — the frontier pointer

python_cleanup_fixture_contract.py reads MEMORY.md's `- Next` line. From 0987 to 0996 I wrote it with a trailing period,which the pattern captures,so the pointer disagreed with the tree and the contract would have failed at the next push.
Fixed by writing a comma,the form the line had before. The same contract also required the frontier's `.1` to appear once in the task index,true only while the frontier was a G1 leaf;it now reads the frontier family's own row. Found when I ran it for G2's first leaf,rc=1 before,rc=0 after.

## D393 — recorded a pass from a tail line

0988's leaf said strict clippy rc=0. It had failed:one is_multiple_of lint in tests/project_kill.rs. My command piped clippy through tail -1,which showed a line about the
failed build,and I wrote the record without reading it. The failure surfaced in the same reply,when I reread that output. Hosted CI and the pre-push gate would have refused it.
Fixed0989:the lint,and the leaf's claim corrected in place. Rule from here:a recorded rc comes from the command's own exit status,captured to a retained log,never from what a pipe shows.

## D392 — weak G0 checks

Found 2026-11-06 reading the exit review while building the gauge. G0-04,05,06,08,13 and 17 pass on test -f or grep -q of one phrase,so
a chapter could be emptied and its clause would stay met. Six of the 18 met clauses prove less than the count suggests. The review's own header discloses this;this entry owns the repair.
Fixed0985,leaf G1-SLICE.16.g0-checks:each row now checks what its clause asks,and G0's verdict stays 18 met,1 not met. These read structure and size,so they still cannot tell whether a specification's words are right.

## D391 — one look blamed a passing reader

Owner:G1-SLICE.5f.2.prepush-400.census-patience. The handoff census took one snapshot,so iTerm's status-bar read of the checkout,or a
process dying under lsof,could fail a two-hour probe run at its last step. Measured on this machine:one look blocked71 and refused4 of467.
Fixed0981:only the same process on every look across about a minute blocks. No name is exempted and a real held file still blocks.
