# G0-CONTRACT — acceptance evidence for completed leaves

The evidence half of [`G0-CONTRACT.md`](G0-CONTRACT.md), split out under the containment registry's own
remedy for `tasks_collection`: "a tree that passes 1000 lines splits its completed-leaf evidence into a
sibling file under `docs/tasks/` before adding more" (`.doctrine/live_document_size/surfaces.tsv`).

**The convention, and the gate that forces it.** `scripts/check_task_acceptance.sh` judges EVERY staged
`docs/tasks/*.md` file and refuses one that carries no ticked ROOT CAUSE / ADDRESSED / NO REGRESSION box,
so a tree file may not be emptied of checklists: the leaf being landed keeps its checklist in the tree
file, and the next slice moves it here. Two properties follow, and both are the point of
`docs/decisions/decision_acceptance-evidence-per-leaf.md` — the tree file's FIRST matching box is the
current leaf's, so no stale evidence can shadow it (defect D15's facet 1), and this file holds only
completed, ticked, evidence-backed checklists, never a placeholder (facet 2).

Order is landing order, oldest first, so this file reads the same way the tree's Commit Log does.

**A sealing operation changes which box the gate judges, so re-run the enforcer after moving checklists
between files and before committing.** The gate judges the FIRST matching box in a staged file, which means
a well-formed checklist at the top conceals a defective one behind it — three such boxes (`.14`, `.14b`,
`.19`, all ROOT CAUSE) survived eleven commits and refused a commit the moment the first ten checklists were
sealed out (defect **D52**, fixed with the invocations they should have carried).

## Sealed archive — earlier leaves

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`g0-contract-evidence-part1.md`](../history/window1.md#stitchcad-g0-contract-evidence-part1md) | the first 10 completed leaves, `G0-CONTRACT.2` … `G0-CONTRACT.4b` | 560 lines, 52573 bytes, `sha256:cf35caab…` |

The live window below holds the more recent checklists. When it passes the `tasks_collection` per-part
health again, the oldest are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

### `G0-CONTRACT.14` — the governance model exists before the community does

- [x] **REPRODUCE / ISSUE** — the G0 exit clause requires "governance model drafted (project owner named;
  sewist-vs-programmer review paths defined)" and roadmap §12 requires four things nothing here had written:
  a domain-review path that is not code review, golden-file approval ownership, the sewist-vs-programmer
  conflict rule, and named funding/procurement owners. Measured at `HEAD`:
  `git ls-tree --name-only HEAD docs/book/src/` → `SUMMARY.md`, `introduction.md`, `spec`, `rc=0` — no
  governance chapter, while `grep -ci governance ROADMAP.md` → `6` clauses expect one, and `G0-CONTRACT.15`
  cannot review an exit clause that has no deliverable.
- [x] **ROOT CAUSE (WHY + WHERE)** — §12 states the requirement as prose with no owner per rule: "a named
  domain-expert approval path (two-step: expert + maintainer)" says who approves but not which changes take
  that path, and §14's risk row "community fork over governance" is mitigated only by "governance doc at G0,
  while the room is empty" — scheduled, and never written. Three of the four rules also depend on a *person*
  this repository does not have, which is why the leaf stayed blocked until the ruling of `2026-09-30`
  separated the drafting from the naming; the blocking part was that coupling, not the missing names.
  Both roadmap clauses cited above, located: `grep -n 'named domain-expert approval path' ROADMAP.md` →
  `781`, `rc=0`, and `grep -n 'Community fork over governance' ROADMAP.md` →
  `839:| Community fork over governance | Governance doc at G0, while the room is empty |`, `rc=0`.
  *(Evidence supplied by `G0-CONTRACT.12`: this box asserted both quotations without an invocation, which
  the acceptance gate requires — defect D52.)*
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` (`wc -lc` → `198` lines / `15 593` B, widest
  line `199` B, inside the `book_collection` per-part health of `400` / `24 576` / `200`) carries ten sections:
  the classification table putting every change class on exactly one of the two review paths; the conjunctive
  two-step rule for anything that alters exported bytes; seven roles each with the authority it needs and
  whether an agent may hold it; the four-step conflict path (classify → make a contested default a profile
  parameter → escalate by review round, not by date → a fork is a legitimate outcome); golden approval with
  two signatures and the no-golden-over-an-`assumed`-constant precondition; the public/never-public boundary
  and its three consequences for the review paths; agents under governance; the procurement table with a
  fallback and its cost in evidence quality per item; **all four empty seats in one table** (§8), which is the
  acceptance clause "flagged to the director in one place, not discovered later"; the four questions
  deliberately left undecided; and a verification-status section separating the roadmap citations from the
  project decisions. `make book` → `INFO HTML book written to …`, `exit=0`, and
  `ls docs/book/book/governance.html` → present, wired in as the book's first non-specification part. The six
  project decisions are recorded separately in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md` (`84` lines / `6 600` B,
  indexed, carrying an `answers:` line), so a reader can tell the citation from the invention.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 77 files measured`,
  `exit=0`; the three censuses over the book are green and unchanged by the new chapter —
  `run_glossary_census.sh` → `276 terms / 8 parts / 145 tokens / 0 failure(s)` (so the chapter introduces no
  undeclared machine token), `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`,
  `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`; the fixture still re-derives
  at `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`; `make probes` →
  `make probes: 12 suite(s) green`, `exit=0`. No Rust and no instrument changed, so the suite count is
  unchanged; `git diff --cached --name-only | grep -cE '\.(rs|sh)$'` → `0`, `rc=1` — a documentation-only
  slice, so no instrument and no crate behaviour could regress.
- [x] **FIX** — wrote the chapter and the record; added the book's `# Governance` part to `SUMMARY.md`;
  moved `.4b`'s completed checklist into `G0-CONTRACT-evidence.md` in this same commit, which is the
  convention `.4b` recorded and the first slice to obey it — the leaf being landed keeps its checklist in the
  tree file, because `scripts/check_task_acceptance.sh` judges every staged `docs/tasks/*.md` and refuses one
  with no ticked boxes. The tree file is `744` lines / `64 445` B (inside its `800` / `65 536` health) and the
  evidence sibling holds `10` completed checklists at `576` lines / `53 726` B.
- [x] **The blocked part is stated as a cost with a schedule attached, not as an apology.** The unnamed domain
  expert is not a paperwork gap: that seat gates the reference fixture's `assumed` constants, which gate the
  **first G2 golden** (§4 of the chapter), so the dependency is written where a plan will hit it. Each
  procurement item likewise carries its fallback *and what the fallback costs in evidence quality*, because
  roadmap §14 already made the partner-run manual test normative and an unstated cost is how a slip becomes a
  silent downgrade of the release claim.
- [x] **The rollover this slice's changelog append triggered is performed and verified in the same commit.**
  The live window had crossed its health target (`410` lines / `36 632` B against `400` / `32 768`), so the two
  oldest entries are sealed into `docs/history/stitchcad-changelog-part5.md` (`82` lines / `7 505` B /
  `sha256:18548ff78f8345d1…`) with a pointer row in the live file, which is back inside health at `329` lines /
  `29 295` B. Losslessness is proved against the committed state rather than against memory: the sealed bytes
  are identical to `git show HEAD:CHANGELOG.md` from the same heading onward (`True`), and the standing
  verifier agrees — `run_changelog_ledger_probes.sh` → `probes: 8 pass / 0 fail`, its `DESCRIPTOR` rule
  reproducing all seven sealed segments' digests including the new one.
- [x] **LOCKSTEP** — the leaf's status, the frontier (which now names `SPINE.4.4` as the repository's next
  slice, per the ruling's order), the tree's decisions, blockers, verification and commit logs and its
  changelog; `docs/TASK_TREE.md`'s frontier cell; `docs/decisions/INDEX.md` and the regenerated Knowledge Map;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md` and `DEV_NOTES.md` (a new lesson) updated in this commit.
  Lesson promotion: **promoted** — the governance record gains an `answers:` line.

### `G0-CONTRACT.4c` — the proposal is ruled on and applied, and the criterion arrives with owners

- [x] **REPRODUCE / ISSUE** — two things were true and neither could stand. (1) The envelope was still
  unproved in the roadmap: `sed -n '/^### G3 /,/^- Domain-complexity/p' ROADMAP.md | grep -ci
  'collar\|trousers\|button\|pocket'` → `1` before this slice, and that one hit was the *note* that permitted
  an intermediate, not a criterion; the matrix said so mechanically —
  `bash docs/tasks/artifacts/feature_matrix/run_feature_matrix_census.sh` → `A3 advisory … proposed cells: 4`.
  (2) Applying the amendment was impossible without cheating: `wc -lc ROADMAP.md` → `919 50821`, exactly the
  `roadmap` row's transition-debt baseline (`lines=919;bytes=50821`), and `check_live_doc_size.sh` refuses a
  widened baseline — so the only way to grow the roadmap was to edit the number, which is the silent widening
  the rule exists to prevent.
- [x] **ROOT CAUSE (WHY + WHERE)** — the first cause was authority, not analysis: the director's ruling of
  `2026-09-30` reserved amending `ROADMAP.md`, and his second instruction delegated the finding outright, so
  the proposal became a decision this repository could execute. The second cause was a missing mechanism: a
  debt baseline is a *stored copy* of a file's size at a moment, and nothing tied it to the file's revision
  identity, so a legitimate revision and a silent widening looked identical to the checker. Measured, because
  the block is the evidence: `bash scripts/check_live_doc_size.sh` over the amended roadmap →
  `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (947 > baseline 919) — a baseline never grows`,
  `exit=1`, against `grep -o 'lines=[0-9]*;bytes=[0-9]*' .doctrine/live_document_size/surfaces.tsv` →
  `lines=919;bytes=50821`, `rc=0`. That is the containment adoption note's deferred trigger 3 — "a stored copy
  of a mechanically owned value needs an executed freshness oracle" — fired by the first roadmap amendment,
  and `SPINE.4.5` discharges it.
- [x] **ADDRESSED (verified)** — roadmap **v0.3** carries the criterion: `grep -n 'envelope coverage'
  ROADMAP.md` → line `712`, `- **Exit (envelope coverage):** every garment §3.2 names drafts, grades and
  exports at this gate or an earlier one …`, and the same census over the section now returns `3` matching
  lines instead of `1`. The revision is marked where the roadmap's own policy requires —
  `grep -n 'v0.3' ROADMAP.md` → the title (line 1), the status block (line 11, naming the source), the
  Appendix A disposition entry (line 926) and the end line (line 945) — and the file measures
  `wc -lc ROADMAP.md` → `947 52818` (+28 lines / +1 997 B), re-based in the registry as
  `lines=947;bytes=52818;at=v0.3`. The matrix is committed rather than provisional:
  `grep -c 'proposed §11 amendment' docs/book/src/spec/feature-matrix.md` → `0`, and
  `run_feature_matrix_census.sh` → `105 rows / 29 diagnostics / 0 failure(s)`, `exit=0`, with `D32 rows: 0`
  and `proposed cells: 0`; `run_feature_matrix_probes.sh` → `probes: 12 pass / 0 fail` after its
  `PROPOSAL-VISIBLE` arm was rebuilt to pin A3 in BOTH directions (0 on the real tree, 1 once a marker is
  injected) instead of asserting a count that no longer exists. The criterion arrives with owners:
  `G3-GRADING.5` is required (trousers + pocket + derived buttonhole), `G3-GRADING.15` is created (the classic
  collar), and `.14`'s exit review fails if a §3.2 garment has no leaf's evidence — so the capture claim still
  derives: `run_tree_coverage_census.sh` → `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned /
  0 orphan(s) / 0 dead link(s)`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `13 suite(s) green` (twelve before this slice; the thirteenth is the tree-coverage suite `PLANNING.6` adds),
  with `run_live_doc_size_probes.sh` → `probes: 5 pass / 0 fail` including the new `REAL-3` arm that stales the
  real registry to `at=v0.2` and requires the refusal, and `check_live_doc_size.sh --self-test` →
  `15 arms, 0 failed` (eleven before `SPINE.4.5`); `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`, `exit=0`, the roadmap row inside its
  re-based debt and its `240` B maxline health (widest `220` B); the book's other censuses are green and
  unchanged — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)`, standards
  `6 registered / 6 designations used / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces /
  0 mismatch(es)`; `make book` → `exit=0`. No Rust changed.
- [x] **FIX** — amended §11 G3 through the roadmap's revision policy (criterion added, the "intermediate
  complexity" note rewritten so an intermediate can never substitute for a criterion); logged it in Appendix A
  with its source and the defect it closes; re-based the containment baseline with `at=v0.3` and the authority
  cited in the row's notes; dropped `(proposed)` from the four cells and rewrote §1 rule 2, §9, §12 and §14 so
  no sentence still calls the gap open; marked `decision_d32-proving-gates-proposed-roadmap-amendment.md` as
  **applied** (keeping the rejection path, because a future revision may withdraw the criterion); marked the
  containment adoption record's trigger 3 as fired and discharged; gave `G3-GRADING` the two leaves and the
  acceptance-table row; fixed the coverage census's tree enumeration and put it under `make probes` (D44,
  `PLANNING.6`); and added the revision-aware baseline (`SPINE.4.5`).
- [x] **LOCKSTEP** — `MEMORY.md` (v0.3, the amendment no longer pending), `LIVE_STATUS.md`, `CHANGELOG.md`,
  `DEV_NOTES.md`, `docs/TASK_TREE.md` (the coverage claim now cites both the census and its probe suite),
  `TOOLBOX.md` (two new rows), `docs/decisions/INDEX.md` and the regenerated Knowledge Map; D44 logged and
  fixed in `PLANNING.md`; `.15`'s scope narrows because the proposal it was to carry is applied. Lesson
  promotion: **promoted** — `decision_revision-aware-containment-baseline.md` carries an `answers:` line.

### `G0-CONTRACT.14b` — an empty seat is held acting with limits, or it is vacant

- [x] **REPRODUCE / ISSUE** — governance §8 listed three seats as "blocked on the director" and every chapter
  that needed a domain reviewer pointed at a leaf that cannot name one:
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/` → the standards registry's owner cells and the
  fixture's `assumed`-constant bullets, i.e. three chapters deferring to an event that had no mechanism. The
  director then delegated the finding ("make the necessary calls and every needed action"), which makes the
  vacancy this repository's problem to state precisely rather than to wait on.
- [x] **ROOT CAUSE (WHY + WHERE)** — the chapter was drafted under a ruling that reserved naming humans, so it
  recorded the gap honestly and stopped there: "blocked" is a state, not a plan. What was missing is the
  distinction between two kinds of authority — **office**, which the director already holds and can therefore
  hold *acting*, and **competence**, which nobody in this repository has and no acting arrangement can supply.
  Treating the three seats alike would either stall the project owner's decisions unnecessarily or, worse,
  imply that an engineer can certify a sewing judgement. Measured at the commit that drafted the chapter:
  `git show 7398352:docs/book/src/governance.md | grep -ci 'acting'` → `0`, `rc=1`, and the same for
  `vacant` → `0`, `rc=1` — the distinction this leaf adds existed nowhere in the file. *(Evidence supplied
  by `G0-CONTRACT.12`, defect D52.)*
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` §8 is rewritten as three subsections: the seat
  table now names who holds each seat (`director, acting (§8.1)` twice, `**nobody — vacant, not acting**` for
  the domain expert); §8.1 states the acting authority and four hard prohibitions on it (no confirming the
  fixture's `assumed` constants, no signing a golden's semantic half, no ruling a safety-relevant term, no
  approving a byte-changing Factory Profile — which under §1's conjunctive rule leaves such a change
  unapproved with the missing half reported); §8.2 states the ask per seat so naming one is a single act.
  `wc -lc docs/book/src/governance.md` → `233` lines / `18 465` B, widest line `185` B, inside the
  `book_collection` per-part health of `400` / `24 576` / `200`; `make book` → `INFO HTML book written to …`,
  `exit=0`. The rule is recorded in `docs/decisions/decision_unnamed-seats-acting-authority.md` (indexed,
  `answers:` line) and the deferring chapters now cite the seat's real state:
  `grep -rc 'vacant' docs/book/src/spec/standards.md docs/book/src/spec/reference-skirt.md
  docs/book/src/spec/glossary.md` → `3`, `2`, `1`, and the stale deferral is gone —
  `grep -rn 'G0-CONTRACT.14` names' docs/book/src/spec/ | grep -c .` → `0`, `rc=1`. The standards census still
  resolves every owner cell: `run_standards_census.sh` → `6 registered / 6 designations used / 0 failure(s)`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`;
  `bash scripts/check_live_doc_size.sh` → `live-doc-size: OK — 17 surfaces, 15 routes, 84 files measured`,
  `exit=0`; the book's censuses are green — glossary `276 terms / 8 parts / 145 tokens / 0 failure(s)` (the new
  prose introduces no undeclared machine token), standards `6 registered / 0 failure(s)`, matrix
  `105 rows / 0 failure(s)`, fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  `make probes` → `13 suite(s) green`. No Rust and no instrument changed in this leaf.
- [x] **FIX** — rewrote §8 and its status block, §2's "currently" column for the three seats, the fixture
  chapter's two reviewer references, the glossary's two, the standards registry's two owner cells and its
  verification-plan step 1 — each cross-chapter link written as `../governance.md`, because a chapter under
  `spec/` reaches the book's top level one directory up and a dead link here would have been the D28 class
  (caught by building the book, not by reading the path); added the decision record and its index row.
- [x] **The first G2 golden stays gated, and that is the point of the leaf.** The tempting move was to record
  the director as acting domain expert so the schedule clears; the honest one is a vacant seat with a named
  consequence, because a golden frozen over an unreviewed `assumed` constant freezes a guess with the
  confidence of a fact, and the fixture chapter says so where a plan will hit it.
- [x] **LOCKSTEP** — the leaf, this checklist and the tree's blockers; `MEMORY.md`'s blocker bullet;
  `LIVE_STATUS.md`; `CHANGELOG.md`; `docs/decisions/INDEX.md` and the regenerated Knowledge Map. Lesson
  promotion: **promoted** — the record carries an `answers:` line.

### `G0-CONTRACT.14c` — a delegated decision is bounded, not merely disclosed

- [x] **REPRODUCE / ISSUE** — finding 1: the engineer proposed the `ROADMAP.md` §11 G3 amendment in `.4b` and
  applied it in `.4c`, and the only thing recording that was a sentence in a session report.
  `git show HEAD:docs/book/src/governance.md | grep -ci 'self-appl\|same party'` → `0`, `rc=1`, against `2`
  after this slice: the governance model covered human seats and agents, and said nothing about the case this
  project actually runs in.
- [x] **ROOT CAUSE (WHY + WHERE)** — governance §8 was written to list *vacancies*, and §6 to bound *agents*;
  neither addresses a party that legitimately holds the pen for both halves of a decision. A disclosure is not a
  control: it decays with the conversation it was made in, while a rule is read by whoever acts next. The
  founding instance is measured in the tree: `git log --oneline -3 -- ROADMAP.md` → `513374c` (the v0.3
  amendment), `5257257` (bootstrap), `f1dcbe4` (initial), `rc=0` — the roadmap's first revision in its life was
  prepared as a proposal in `4b0bb95` and applied in `513374c`, adjacent commits by the same author.
- [x] **ADDRESSED (verified)** — `docs/book/src/governance.md` §6.1 carries five rules: record the author and
  the applier and say so when they are the same; the author of a decision may not approve the evidence that
  decision requires (approval belongs to a reviewer meeting §2's independence criterion, and where none exists
  the claim stays **unapproved**); consequences become instruments others can run; the record states what would
  reverse it and who may; a delegation to decide is not one to upgrade evidence.
  `wc -lc docs/book/src/governance.md` → `265` lines / `21 303` B, inside the `book_collection` per-part health
  of `400` / `24 576`; `make book` → `INFO HTML book written to …`, `exit=0`. The rule bites in three places,
  each verifiable: the roadmap's Appendix A v0.3 entry now states that author and applier were the same party
  (`grep -c 'same one' ROADMAP.md` → `1`, `rc=0`), `G3-GRADING.14`'s acceptance withholds the review from the
  criterion's author, and the reasoning is in
  `docs/decisions/decision_self-application-under-delegation.md` (indexed, `answers:` line).
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; the roadmap grew by four lines
  and the revision-aware baseline refused it, exactly as `SPINE.4.5` designed:
  `LIVE-DOC-SIZE: roadmap: transition debt WIDENED on lines (951 > baseline 947)`, `exit=1` — handled by the
  rule rather than around it, re-basing to v0.3's final state (`lines=951;bytes=53153;at=v0.3`) with the
  authority cited in the row's notes, and recording in the decision record the limit this exposed (`at=` binds a
  baseline to a revision marker, not to a commit). `bash scripts/check_live_doc_size.sh` →
  `live-doc-size: OK — 17 surfaces, 15 routes, 89 files measured`, `exit=0`; `make probes` →
  `15 suite(s) green`; `make book` → `exit=0`; the book's censuses green.
- [x] **FIX** — added §6.1 to the governance chapter, the disclosure to the roadmap's disposition entry, the
  independence requirement to `G3-GRADING.14`'s acceptance, the decision record and its index row; re-based the
  containment baseline under the rule and recorded that rule's limit.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier and the tree's logs; `MEMORY.md`, `LIVE_STATUS.md`,
  `CHANGELOG.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` and the regenerated Knowledge Map in this commit. Lesson
  promotion: **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.19` — what the project does not know is one command away

- [x] **REPRODUCE / ISSUE** — the domain seat is vacant and the book's unverified claims were enumerable only by
  reading it: `git grep -c '\`assumed\`' HEAD -- docs/book/src` → `18` occurrences across `6` files, with
  nothing tying them to the seat that owes them and nothing noticing if one disappeared. The roadmap's core
  property is that uncertainty is data; a datum nobody can list is prose.
- [x] **ROOT CAUSE (WHY + WHERE)** — every chapter states its own verification status, and no instrument reads
  them together, so "what waits on a human" lived in the reader's memory. At the commit before this leaf
  landed: `git ls-tree -d --name-only a743d53 docs/tasks/artifacts/ | wc -l` → `13` (this box first said
  twelve; corrected by `G0-CONTRACT.12` under D52, which also supplied the invocation) and
  `… | grep -c uncertainty` → `0`, `rc=1` — thirteen instruments, none about uncertainty. The population
  spans nine files and six marker spellings, which is exactly the shape that drifts.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/uncertainty/run_uncertainty_census.sh` →
  `uncertainty census: 88 markers / 10 files / 0 unowned / 0 failure(s)`, `exit=0`, enumerating every marker in
  the book's own vocabulary (`assumed`, `unknown`, `unverified-with-owner`, `read-in-repo`,
  `cited-from-roadmap`, `read-external`, `known`, `derived`, `(proposed)`, `vacant`) per file and per resolving
  authority, and refusing — rule `U1` — a blocking marker whose verification-status section names no resolver.
  Its discrimination is proved, not assumed:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/uncertainty/run_uncertainty_probes.sh` →
  `probes: 6 pass / 0 fail`, with an owned-claim control arm, an unowned synthetic chapter refused by name, a
  chapter added *after* the census was written refused (so the rule is about the population), a definition left
  alone, and an absent book refusing with `exit=2`. Governance §8.1 and the fixture's §11 now point at it, so
  the day a name arrives the work it unblocks is one command away.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `make probes: 15 suite(s) green`, `exit=0`; `make book` → `exit=0`;
  `bash docs/tasks/artifacts/glossary/run_glossary_census.sh` → `277 terms / 8 parts / 146 tokens /
  0 failure(s)` after the new `vacant seat` entry and a re-derived index, and its own suite still
  `probes: 10 pass / 0 fail`; the other censuses unchanged (matrix `105 rows / 0 failure(s)`, standards
  `6 registered / 0 failure(s)`, fixture `20 rows / 4 checks / 5 pieces / 0 mismatch(es)`, coverage
  `13 trees / 3 sibling(s) / 0 orphan(s)`); `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces,
  15 routes, 89 files measured`, `exit=0`.
- [x] **FIX** — built the census and its probe suite; added the `vacant seat` glossary entry (the token census
  demanded an owner for `vacant`, its sixth catch at authoring time) and re-derived the A–Z index; pointed
  governance §8.1 and the fixture §11 at the census; added both `TOOLBOX.md` rows. **Building it found two
  defects in existing instruments, both fixed here:** the glossary census's `resolve()` deleted one `/x/../` per
  `gsub` pass and so reported `../../governance.md` — a file that exists — as a dead reference (it now
  normalises segment by segment), and this census's first authority list counted a bare gate id as an owner, so
  "frozen as a golden at G2" satisfied it and the `UNOWNED` arm passed for the wrong reason until the list was
  narrowed to parties that can *resolve* a claim.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier and the tree's logs; `TOOLBOX.md`,
  `LIVE_STATUS.md` (fifteen probe suites, re-derived), `MEMORY.md`, `CHANGELOG.md` and the regenerated
  Knowledge Map in this commit. Lesson promotion: declined (the two instrument defects are recorded in this
  checklist and in the tools' own headers, which is where a probe author will look; no new dated lesson was
  added to `DEV_NOTES.md` this slice).

### `G0-CONTRACT.9` — the formula language exists, and its numbers are computed

- [x] **REPRODUCE / ISSUE** — ADR-0003 had two halves and the repository held neither.
  `git ls-files docs/book/src/spec/ | grep -c formula` → `0`, `rc=1`; `git ls-files docs/decisions/ |
  grep -c adr-0003` → `0`, `rc=1`. Two chapters already pointed at the missing one —
  `git show HEAD:docs/book/src/spec/ontology.md | grep -c 'formula-language'` → `1` ("specified in the
  formula-language chapter") — and the vocabulary had parked the system decision in this leaf:
  `git show HEAD:docs/book/src/spec/glossary/recipe-and-pieces.md | grep -c 'G0-CONTRACT\.9'` → `2`
  (`block (pattern)`, `reference drafting`). Roadmap §5 requires the language "specified HERE, not
  later" and §11's G0 exit clause names it, so the gap was a leaf not yet taken.
- [x] **ROOT CAUSE (WHY + WHERE)** — the requirement is in two places and neither had an artifact behind
  it. `grep -n 'specified HERE' ROADMAP.md` → `349:language is specified HERE, not later: operators,
  units inside expressions,`, `rc=0` — that is §5's ADR-0003, whose second half also requires "a named
  drafting system ships as reference blocks (v1: free drafting + one documented system, decided at G0)"
  (`sed -n '345,353p' ROADMAP.md`). `grep -n 'slides into mechanical' ROADMAP.md` → line `831`, whose
  mitigation cell reads "ADR-0003; formula language specified at G0", `rc=0` — the risk register names the
  failure this gap produces. Ownership was never in doubt:
  leaf `.9` held it and the frontier named it next, so the cause is a leaf not taken, and the constraint
  on *how* to take it is that row: a recipe language specified loosely becomes a solver, and a solver is
  what roadmap §6.2 and ADR-0001 put elsewhere. The chapter's first exclusion is therefore implicit
  solving, and its evaluation model is single-pass with no fixpoint.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/formula_language/run_formula_language_census.sh`
  → `formula-language census: 17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, `exit=0`, with
  `names shared with the fixture: 12 · disagreements: 0`, all four fixture oracles holding as `assert`
  statements (`280000 = 280000`, `185000 = 185000`, `740000 = 740000`, `80000 = 80000` in internal µm),
  and each of the 13 refusals raising the token its row names. Discrimination is proved, not assumed:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/formula_language/run_formula_language_probes.sh`
  → `probes: 15 pass / 0 fail`, including a CONTROL arm that keeps an unrelated prose edit green. Sizes
  are inside the collection's per-part health (`400` lines / `24 576` B / `275` B): `wc -lc` →
  `308 20593`, `247 12690`, `92 7116`, widest line `189` B. `make book` → `INFO HTML book written to …`,
  `exit=0`, with `formula-language.html` and both parts rendered.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `16 suite(s) green`; the neighbouring censuses unchanged: fixture `20 derived rows / 4 closure checks /
  5 pieces / 0 mismatch(es)`, matrix `105 rows / 29 diagnostics / 0 failure(s)`, standards `6 registered /
  6 designations used / 0 failure(s)`, coverage `10 lanes / 13 trees / 3 sibling(s) / 0 unowned`,
  uncertainty `107 markers / 13 files / 0 unowned / 0 failure(s)` (was `88 / 10` — the three new files'
  markers are all owned); `bash scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 90 files
  measured`, `exit=0`, at `31 warning(s)` against the `32` before the slice, because four `LIVE_STATUS.md`
  cells were tightened under D36's third instance and its widest line went `313` B → `213` B inside a
  `220` B health target. Two instrument bugs were found and
  fixed while building the census, both recorded in its header: a code-span scanner that paired backticks
  across newlines read the whole file as one span (a fence's ``` is an odd count) and reported `125`
  undeclared operators that were EBNF nonterminals, and a link resolver that dropped the leading `/` of an
  absolute base and so reported every link in the book dead.
- [x] **FIX** — wrote the three chapter parts; the ADR-0003 record with fourteen language decisions, five
  read candidates and three re-open conditions; the census and its 15-arm probe suite; `G3-GRADING.16` to
  own the blocks; twelve glossary terms plus two updated entries with the A–Z index re-derived; SUMMARY,
  the spec index, the ontology's cross-reference and the fixture's §3 note. **D48 logged and fixed** (a
  duplicate `## Acceptance Checklist` heading in `G3-GRADING.md` — D15's class by heading instead of by box
  — and a children range that said `.14` while `.15` was in the file), and **D36's third instance**
  recorded and removed (`LIVE_STATUS.md` listed D47 open after `SPINE.20` closed it; its probe count is now
  the command's, not a hand-kept number).
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and open questions, its
  three logs; `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`,
  `TOOLBOX.md` (two instrument rows, and the `make probes` row no longer carries a hand-kept suite count),
  `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in this
  commit. Both ledgers rolled over in the commit whose append crossed them (`devnotes-part3` 50 lines /
  4723 B, `changelog-part8` 106 lines / 9766 B), with `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`
  reproducing both digests. Lesson promotion: **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.10` — the dialects are a closed registry, and the layer table is read against the roadmap

- [x] **REPRODUCE / ISSUE** — ADR-0004 was the last of the four ADRs with neither a record nor a chapter.
  `git ls-files 'docs/book/src/spec/interchange*' | wc -l` → `0`, and
  `git ls-files docs/decisions/ | grep -c adr-0004` → `0`, `rc=1`. Three chapters already leaned on the
  missing one: `git show HEAD:docs/book/src/spec/glossary/interchange-and-envelope.md | grep -c
  'G0-CONTRACT\.10'` → `16` entries whose canonical object was a leaf rather than a clause,
  `git show HEAD:docs/book/src/spec/ontology.md | grep -c 'interchange-dialects chapter'` → `1`, and
  `git show HEAD:docs/book/src/spec/instantiation-paths.md | grep -c 'G0-CONTRACT\.10'` → `2`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n 'ADR-0004 — Interchange dialects' ROADMAP.md` → `356`, and
  the clause that makes this a G0 deliverable rather than a writer detail is at `358`: `grep -n 'D6673-10
  is withdrawn' ROADMAP.md` → `358:- **ASTM D6673-10 is withdrawn (Jan 2019, no replacement)** — record
  this.`, `rc=0`. A withdrawn specification whose convention cutting rooms still enforce is the reason a
  dialect is a *named target with a validation behind it* instead of a format with flags: there is no
  document to conform to, so the oracle can only be a receiver reading the file back. Ownership was
  unambiguous — leaf `.10` held it and the frontier named it next.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/interchange/run_interchange_census.sh` →
  `interchange census: 17 layers / 4 targets / 12 entities / 0 failure(s)`, `exit=0`, where the `17`
  layers and `7` AAMA names are parsed out of `ROADMAP.md`'s own bullet rather than listed beside it, so
  amending the roadmap's convention reddens this run until the chapter dispositions the change. Its
  discrimination is proved: `TMPDIR=$PWD/target/scratch bash
  docs/tasks/artifacts/interchange/run_interchange_probes.sh` → `probes: 13 pass / 0 fail`, including
  ROADMAP-GROWS (a layer added to a *copy* of the roadmap is refused by number), AXIS-COLUMN (registry
  column and axis disagreeing in either direction) and CONTROL. The chapter is `303` lines / `21 650` B,
  widest line `227` B, inside the `book_collection` per-part health of `400` / `24 576` / `275`;
  `make book` → `INFO HTML book written to …`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `17 suite(s) green`; the neighbouring censuses unchanged: glossary `294 terms / 8 parts / 155 tokens /
  0 failure(s)` after five new terms and `16` entries repointed from this leaf to a chapter clause, with
  the A–Z index re-derived; fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`;
  formula language `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`; standards `6 registered /
  0 failure(s)`; uncertainty and coverage green; `bash scripts/check_live_doc_size.sh` → `OK — 17
  surfaces, 15 routes, 91 files measured`, `exit=0`. Two arm defects were found while building the probe
  suite and fixed there rather than in the census: one arm asserted a *second* symptom the property does
  not require (an invented AAMA name displacing DRAW, which layer 7 still uses), and one cited `units §9`
  as a dead clause when §9 exists — an arm whose mutation is not a breach is a green arm that tested
  nothing.
- [x] **FIX** — wrote the chapter (six axes with the party that resolves each, a closed registry of four
  targets, the seventeen-layer table in both naming modes with this project's object mapping, cut-as-1
  against sew-as-1, BLOCK/SST/PST, one polyline-only entity set for both releases with arcs exact as
  bulges and Béziers tessellated at T2's bound, three grading carriages, HPGL and PDF, the
  receiver-config record, seven diagnostics); the ADR-0004 record with ten decisions and the sources read
  and attempted; the census and its 13-arm probe suite; five glossary terms plus `16` repointed entries
  and a re-derived index; SUMMARY, the spec index and the ontology's cross-reference. **D50 logged and
  its live pressure discharged in the same commit:** `KNOWLEDGE_MAP.md` had reached `99 %` of its byte
  ceiling (`99 8128` against `8192`), so the hand-curated input was trimmed to `37` lines / `3 255` B and
  the map fell to `84` / `6 733`; the convention, the size formula and the trigger for re-deriving the
  row are recorded in `decision_knowledge-map-entries-are-orientation-sized.md` and the durable half is
  `SPINE.5`'s.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs;
  `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two
  instrument rows), `docs/decisions/INDEX.md` (two records), `knowledge-map/subsystems.md` and the
  regenerated Knowledge Map in this commit. Lesson promotion: **promoted** — the ADR-0004 record carries an
  `answers:` line.

### `G0-CONTRACT.11` — the spike's rule is written before its measurement exists

- [x] **REPRODUCE / ISSUE** — ADR-0002 was the one ADR whose evidence did not exist yet, and nothing in the
  repository constrained what the spike would be allowed to conclude. `git ls-files docs/decisions/ |
  grep -c adr-0002` → `0`, `rc=1`; `git grep -ci 'spike' HEAD -- docs/decisions/` → only
  `decision_adr-0001-license-and-solver.md` (which mentions the G1 spike as a re-open condition), so no
  record carried a measurement protocol; and the consumer was already waiting on one:
  `git show HEAD:docs/tasks/G1-SLICE.md | grep -c 'G0-CONTRACT\.11'` → `1`, the leaf `G1-SLICE.13` whose
  acceptance said only "the measurements the protocol named are recorded".
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n 'ADR-0002 — UI stack' ROADMAP.md` → `328`, whose own text
  defers the decision ("A G1 executable spike … settles it with evidence") and warns against the failure
  mode this leaf exists to close: *"Custom wgpu, not DOM canvas" is a hypothesis to test, not an axiom*.
  `grep -n 'canvas-hosting' ROADMAP.md` → `685` (G1's exit clause) and `867` (§15.10's locked decision),
  `rc=0`. So the gap was not a missing opinion but a missing **rule**: a spike run without a written
  decision rule is argued afterwards, and the argument is won by whoever likes the result.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/canvas_spike/run_spike_verdict.sh` →
  `spike verdict: 0 rows / 0 profiles / PENDING — ADR-0002's canvas half stays `proposed` until
  G1-SLICE.13 records measurements / 0 refusal(s)`, `exit=0`: the instrument reads the seven gates, the
  applicability table and the rule parameters from three TSVs and reports the honest state. Its
  discrimination is proved on twelve synthetic result sets:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/canvas_spike/run_spike_verdict_probes.sh` →
  `probes: 13 pass / 0 fail`, including CORRECTNESS (the fastest topology loses to `snap_exact = no` and
  the gate is named), R4-FREE (a faster native-only winner gives way to one renderer for both profiles),
  NO-SURVIVOR (R5 escalation naming the `dev-shell` fallback), GATES-READ (tightening `fidelity_max_px`
  from `0.5` to `0.2` in a copy of the TSV changes the verdict, so the thresholds are read and not
  hardcoded), INCOMPLETE, UNDECLARED, MISSING-ROW and CONTROL. The record itself is `116` lines /
  `8 849` B: inside the `decisions_collection` per-part line health of `120` and at 108 % of its `8 192`-byte
  health, which is a warning and not a breach — the two other ADR records are the same shape.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `18 suite(s) green`; `make book` → `exit=0`; the book's censuses unchanged: glossary `294 terms /
  8 parts / 155 tokens / 0 failure(s)`, interchange `17 layers / 4 targets / 12 entities / 0 failure(s)`,
  formula language `17 bindings / 4 assertions / 13 refusals / 0 mismatch(es)`, fixture `20 derived rows /
  4 closure checks / 5 pieces / 0 mismatch(es)`, standards `6 registered / 0 failure(s)`, matrix
  `105 rows / 0 failure(s)`, coverage `10 lanes / 13 trees / 0 unowned`; `bash
  scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 99 files measured`, `exit=0`. No Rust
  file changed, so `make check` is not this slice's gate; the two new scripts are bash and were run.
- [x] **FIX** — wrote `docs/decisions/decision_adr-0002-ui-stack-and-canvas-spike-protocol.md` (what is
  decided, the three topologies and the hypothesis each tests, the declared corpus with its re-run
  trigger, the seven gates with what each protects, the six-rule decision procedure, what the spike does
  not decide, and the three reversal conditions); the four-file data plane under
  `docs/tasks/artifacts/canvas_spike/` with `results.tsv` empty on purpose; the verdict instrument and its
  13-arm probe suite; and `G1-SLICE.13`'s acceptance rewritten to consume the instrument by path, name the
  corpus, and require a recorded decision for any human overruling the printed verdict (R6).
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two instrument rows,
  and the formula-language row tightened under its column's budget), `docs/decisions/INDEX.md` and the
  regenerated Knowledge Map in this commit. Lesson promotion: **promoted** — the new record carries an
  `answers:` line.

### `G0-CONTRACT.12` — the release contract is the roadmap's §9, compared rather than restated

- [x] **REPRODUCE / ISSUE** — roadmap §9 was the last G0 clause with a promised chapter and no chapter.
  `git ls-files 'docs/book/src/spec/release*' | wc -l` → `0`; the spec index carried the promise as an
  unlinked row, `git show HEAD:docs/book/src/spec/index.md | grep -c '^| Release and approval |'` → `1`,
  `rc=0`; and the vocabulary was parked against this leaf,
  `git show HEAD:docs/book/src/spec/glossary/profiles-and-release.md | grep -c 'G0-CONTRACT\.12'` → `13`
  (twelve entries plus the part's own header note saying "until that chapter lands, the roadmap clause is
  cited").
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n '^## 9. Signoff' ROADMAP.md` → `626`, `rc=0`, and the exit
  clause that makes it a G0 deliverable: `grep -n 'approval states & release contract' ROADMAP.md` →
  `670:  decided; approval states & release contract (§9) specified; ADR-0001`, `rc=0`. §9 is a list — nine
  manifest fields, six acceptance states, three artifact classes — and a list realised in prose drifts: a
  field is renamed, a rung is merged, a class is forgotten, and every gate stays green because nothing
  compares the two documents. So the cause is not only an unwritten chapter but the absence of an
  instrument that would notice the difference.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/release_contract/run_release_contract_census.sh`
  → `release-contract census: 9 manifest fields / 6 states / 8 matrix rows / 0 failure(s)`, `exit=0`, where
  the `9` and the `6` are parsed out of roadmap §9 (parenthesis-aware, because one field carries a nested
  comma) and the `8 × 3` matrix is compared against §8.2's header and example rows and against the five
  states ontology §5 declares. Its discrimination is proved:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/release_contract/run_release_contract_probes.sh` →
  `probes: 14 pass / 0 fail`, including LADDER-ORDER (two rungs swapped, both orders printed),
  FIELD-INVENTED (a field wearing the roadmap's authority without its wording), DISPOSITION-UNUSED (a
  vocabulary word no cell carries) and CONTROL. The chapter is `257` lines / `19 010` B, widest line
  `256` B, inside the `book_collection` per-part health of `400` / `24 576` / `275`; `make book` →
  `INFO HTML book written to …`, `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `19 suite(s) green`; the eight neighbouring censuses unchanged: glossary `298 terms / 8 parts /
  156 tokens / 0 failure(s)` after four new terms, twelve repointed entries and a re-derived index,
  interchange `17 layers / 4 targets / 12 entities / 0 failure(s)`, formula language `17 bindings /
  4 assertions / 13 refusals / 0 mismatch(es)`, fixture `20 derived rows / 4 closure checks / 5 pieces /
  0 mismatch(es)`, matrix `105 rows / 0 failure(s)`, standards `6 registered / 0 failure(s)`, uncertainty
  `127 markers / 15 files / 0 unowned`, coverage `10 lanes / 13 trees / 0 unowned`; `bash
  scripts/check_live_doc_size.sh` → `OK — 17 surfaces, 15 routes, 102 files measured`, `exit=0`. Two probe
  arms failed first against a correct census and were fixed in the arms, not the tool: one quoted a
  sentence as it read in a draft rather than as it wraps in the file, and one replaced every occurrence of
  a disposition token — deleting its declaration along with its uses, which is the "remove the property,
  not one instance" trap inverted into "remove the rule too".
- [x] **FIX** — wrote the chapter (nine manifest fields each with its source, identity as the manifest's
  digest with no in-place amendment, seven completeness checks against the declared construction, the six
  states in the roadmap's order with the evidence and granter each needs, five scope axes with the
  intersection rule, human-only approval, §8.2's matrix tuned in a recorded table with a closed
  four-word disposition vocabulary and the dependency-closure rule, eight diagnostics); the decision record
  with six rejected alternatives; the census and its 14-arm probe suite; four glossary terms plus twelve
  repointed entries and a corrected `spi` routing (it pointed at this leaf and belongs to `G5-SHELLS.13`).
  **D49's trigger fired and was discharged here rather than deferred:** the evidence sibling had reached
  `1000` lines / `94 086` B against a `98 304`-byte ceiling, so ten completed checklists
  (`G0-CONTRACT.2` … `.4b`) were sealed into `docs/history/stitchcad-g0-contract-evidence-part1.md`
  (`560` lines / `52 573` B, digest reproduced by `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`)
  and the live sibling fell to `449` / `42 065`. The seal then exposed **D52**: three completed leaves'
  ROOT CAUSE boxes (`.14`, `.14b`, `.19`) asserted their evidence with no invocation, which the acceptance
  gate refused once the seal changed which box comes first — all three now carry the command and its real
  output, `.19`'s directory count is corrected from twelve to the `13` the command prints, and the sibling's
  header states the rule (re-run the enforcer after moving checklists, before committing).
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two instrument
  rows), `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in
  this commit. Lesson promotion: **promoted** — the new record carries an `answers:` line.

### `G0-CONTRACT.16` — one message system, chosen on evidence, with an inventory nothing keeps by hand

- [x] **REPRODUCE / ISSUE** — §7.6 required the choice at G0 and the repository had neither the choice nor
  the architecture. `git ls-files 'docs/book/src/spec/i18n*' | wc -l` → `0`; `git ls-files docs/decisions/ |
  grep -c i18n` → `0`, `rc=1`; the spec index carried the promise as an unlinked row,
  `git show HEAD:docs/book/src/spec/index.md | grep -c '^| Internationalization |'` → `1`, `rc=0`; and
  seven references across five chapters pointed at this leaf instead of a clause:
  `git grep -c 'G0-CONTRACT\.16' HEAD -- docs/book/src` → `feature-matrix.md:1`, `formula-language.md:1`,
  `glossary.md:3`, `glossary/profiles-and-release.md:1`, `interchange-dialects.md:1`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `grep -n 'One message system, chosen at G0' ROADMAP.md` → `494`,
  `rc=0`, whose own parenthesis states the trap: "Fluent OR ICU — not 'Fluent or ICU'; they are distinct
  systems; if both ends are needed, a designed bridge". A leaf may record that sentence, or it may decide
  it; deciding needs evidence about two ecosystems that this repository had not read, which is why the
  clause sat open behind four chapters that each cited it. The second cause is the one an instrument had to
  close: the message inventory spans four chapters' diagnostic tables **and a crate's error enum**, so a
  count written by hand is stale the day either grows.
- [x] **ADDRESSED (verified)** — `bash docs/tasks/artifacts/i18n/run_i18n_census.sh` →
  `i18n census: 8 families / 64 message ids / 0 failure(s)`, `exit=0`, where every count is re-derived from
  the envelope's §10, the formula language's §5.2, the dialects' §11, the release contract's §9 and
  `crates/sc-units/src/error.rs`. Deriving it found two errors in the chapter's own first draft, both
  corrected before it landed: the envelope's 29th token `geom_offset_budget` had been folded into the
  `env_*`/`ngo_*` family (the census reads the population by shape, so a sixth prefix could not hide), and
  `UnitError` carries **five** variants — `EmptyDerivation` has no braces and a first grep for
  `Variant {` counted four. Discrimination is proved:
  `TMPDIR=$PWD/target/scratch bash docs/tasks/artifacts/i18n/run_i18n_probes.sh` → `probes: 11 pass /
  0 fail`, including CODE-GROWS, which adds a variant to a *copy* of the crate's source and requires the
  inventory's count to be refused. Sizes: the chapter is `250` lines / `17 650` B, widest `202` B, and the
  record `79` / `5 822`, both inside their per-part health; `make book` → `INFO HTML book written to …`,
  `exit=0`.
- [x] **NO REGRESSION** — `make gate` → `=== all doctrines green ===`, `exit=0`; `make probes` →
  `20 suite(s) green`; the neighbouring censuses unchanged: glossary `305 terms / 9 parts / 156 tokens /
  0 failure(s)` after a ninth part, seven terms and one repointed entry, with the A–Z index re-derived;
  interchange `17 layers / 4 targets / 12 entities / 0 failure(s)`; release `9 manifest fields / 6 states /
  8 matrix rows / 0 failure(s)`; formula language `17 bindings / 4 assertions / 13 refusals /
  0 mismatch(es)`; fixture `20 derived rows / 4 closure checks / 5 pieces / 0 mismatch(es)`; matrix
  `105 rows / 0 failure(s)`; standards `6 registered / 0 failure(s)`; uncertainty `133 markers / 16 files /
  0 unowned`; coverage `10 lanes / 13 trees / 0 unowned`. `bash scripts/check_live_doc_size.sh` → `OK — 17
  surfaces, 15 routes, 105 files measured`, `exit=0`. Two probe arms failed first against a correct census
  and were fixed in the arms: one expected an "uncovered id" message where the family row still covered the
  new variant (the property is the count agreeing with the code), and one mutated half a table cell so the
  reason it meant to delete survived.
- [x] **FIX** — wrote the chapter (what is externalized and what is not, the choice with its evidence
  table, message identity, the termbase per language, the lint with five reasoned exemptions,
  locale-independent canonical files, pseudolocalization, the RTL geometry rule, three review tiers with
  absolute thresholds for the two that reach fabric, the derived inventory, five diagnostics); the decision
  record with six rejected alternatives; the census and its 11-arm suite; a ninth glossary part
  (`localization.md`, seven terms) with the parts table, SUMMARY and index updated; six cross-references
  repointed from this leaf to a clause. **D49's trigger fired on the tree file itself and was discharged
  here:** it had reached `89 632` B against a `98 304` ceiling, so the oldest fourteen changelog entries
  were sealed into `docs/history/stitchcad-g0-contract-changelog-part1.md` (`84` lines / `7 941` B, digest
  reproduced by `run_changelog_ledger_probes.sh` → `9 pass / 0 fail`) and the tree fell to `81 811` B; two
  verification rows past the `443` B cell budget were tightened rather than the target raised.
- [x] **LOCKSTEP** — the leaf, this checklist, the frontier, the tree's decisions and logs; `MEMORY.md`,
  `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`, `docs/TASK_TREE.md`, `TOOLBOX.md` (two instrument
  rows), `docs/decisions/INDEX.md`, `knowledge-map/subsystems.md` and the regenerated Knowledge Map in
  this commit. Lesson promotion: **promoted** — the new record carries an `answers:` line.
