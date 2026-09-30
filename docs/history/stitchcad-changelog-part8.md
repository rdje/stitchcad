# Sealed archive — StitchCAD changelog, slices 41–42

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the
live window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 106 lines, 9766 bytes, `sha256:2c7ee35af83d7c2e97d63a2e3471ba8c03790ea727000547d07330d1e431dce4`
- **Sealed by:** leaf `G0-CONTRACT.9` (the append that crossed the rollover milestone performed the
  rollover, as the doctrine requires).
- **Coverage:** `STITCHCAD-G0-0014` and `STITCHCAD-G0-0004b`, newest first, exactly as they stood in `CHANGELOG.md`.
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`.
- **Write policy:** none — sealed segments are immutable.

---

## STITCHCAD-G0-0014 - the governance model, and the three empty seats (leaf `G0-CONTRACT.14`)

Roadmap §11's G0 exit requires a governance model drafted with the project owner named and the
sewist-vs-programmer review paths defined; §14's mitigation for "community fork over governance" is "governance
doc at G0, while the room is empty". `git ls-tree --name-only HEAD docs/book/src/` -> `SUMMARY.md`,
`introduction.md`, `spec`: the doc did not exist. It does now, in full, and the only part still missing is the
part that is not this repository's to supply.

- **two review paths, and every change class lands on exactly one**: code review judges whether the
  implementation does what the spec says; domain review judges whether the artifact is right for a cutting room
  (roadmap §12: "domain review is not code review"). A classification table assigns eight change classes, so a
  change goes to whoever is competent rather than to whoever answers first
- **the two-step rule is conjunctive**: anything that alters exported bytes needs the domain expert AND the
  maintainer, and the tooling reports which half is missing rather than "partially approved" - a state a
  receiver can misread is worse than a refusal
- **a role is the decision it may make, not the person holding it**: seven roles, each with the authority it
  needs and whether an agent may hold it. Approval is human-only and never manufacturable by a graph mutation
  (§7.8); the G7 reviewer gets a checkable independence criterion - not an author of the code, the spec or the
  evidence under review
- **conflict resolution in four steps**: classify the question (most conflicts are two correct answers to
  different questions) -> make a contested default a Factory Profile parameter with the dissent recorded,
  because the model already carries both readings -> escalate by review round, not by date, since this project
  has no calendar -> treat a fork as a legitimate outcome, and make the rulings carry their evidence, which is
  the only thing a fork cannot copy
- **goldens are a release-contract event**: two signatures (mechanical + semantic), stale-ification per §9, and
  a hard precondition - no golden over an `assumed` constant, which means the unnamed domain expert gates the
  FIRST G2 golden. The dependency is written where a plan will hit it
- **procurement fallbacks state their cost**: evaluation seats, the physical plotter, the standards texts and
  the pilot partner each carry a fallback and what it costs in evidence quality (a partner run is layer-4
  evidence with recorded product, version and settings - slower, fewer targets), because an unstated cost is
  how a slip becomes a silent downgrade of the release claim
- **the three empty seats are in one table** (project owner, domain expert, procurement owner) plus the G7
  reviewer's independence rule, with what waits for each. Until a name exists the state is the ontology's
  `unknown`: not silently defaulted, and blocking what it governs. `.15` records the clause as `met - model
  drafted, named owner pending` or `not met`, never as met on the chapter's strength alone
- six of the chapter's rules are project decisions rather than citations, and are recorded as such in
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`; the chapter's own §10 says
  which clauses are which, so a reader never mistakes an invention for a requirement
- the slice also obeys the evidence-split convention `.4b` recorded, moving `.4b`'s checklist into
  `docs/tasks/G0-CONTRACT-evidence.md` (10 completed checklists) and leaving the tree at 744 lines / 64445 B
- **the rollover this append triggered is performed in the same commit**: the live window had crossed its
  health target (410 lines / 36632 B against 400 / 32768), so slices 30-31 are sealed into
  `docs/history/stitchcad-changelog-part5.md` (82 lines / 7505 B / `sha256:18548ff7…`), proved byte-identical
  to `git show HEAD:CHANGELOG.md` rather than to memory, and the window is back inside health at 329 / 29295.
  `run_changelog_ledger_probes.sh` -> `probes: 8 pass / 0 fail`, its DESCRIPTOR rule reproducing all seven
  sealed segments
- gates: `make gate` -> `=== all doctrines green ===`; `make book` -> exit=0 with `governance.html` rendered as
  the book's first non-specification part; `make probes` -> `12 suite(s) green`; glossary `276 terms`, matrix
  `105 rows`, standards `6 registered`, fixture `20 rows / 4 checks / 5 pieces` - all `0 failure(s)` /
  `0 mismatch(es)`; containment OK, 79 files measured; no `.rs` or `.sh` staged

## STITCHCAD-G0-0004b - every envelope feature has a gate that proves it (leaf `G0-CONTRACT.4b`)

Five rows of the supported-envelope matrix said `unnamed (D32)`: roadmap §3.2 promises a classic collar and
trousers, ontology §4.7 models buttons and pockets, and no gate's exit criteria proved any of them - while
the census stayed green, because the gap was an advisory. The director ruled on 2026-09-30 that the engineer
decides it, and reserved one thing: `ROADMAP.md` is his to amend.

- **the assignment**: collar, trousers, button/buttonhole and pocket -> **G3**; buttons also -> **G5**, whose
  exit already requires a tech pack with a notions list; fly construction -> **G7**, whose exit already
  requires a supported-envelope statement *with named limitations*, so the deferred row needs no amendment
  and is treated exactly like the lining row
- **the reasoning is one rule**: permission is not a criterion. G3's note that an intermediate "may be
  inserted without shame" schedules nothing, so the proposal adds ONE exit criterion over §3.2's whole
  garment list - closing the class rather than the four instances - and names the failure mode: a garment the
  envelope names and no exit criterion proves is a gate failure, not a scope note
- **a proposal is labelled as one**: the four cells that depend on the amendment say `(proposed)`, the exact
  text is quoted current-vs-proposed with its line numbers (`ROADMAP.md:701`-`711`) in
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`, and the record states what
  happens on approval and on rejection. `git diff --name-only HEAD -- ROADMAP.md | grep -c .` -> `0`: the
  roadmap is untouched. A provisional commitment that reads like a settled one is the same gap in better
  clothes, so the census gained an **A3** advisory printing all four cells on every run, with a probe arm
  that removes the markers and requires the count to fall
- **derived, not asserted**: `run_feature_matrix_census.sh` -> `105 rows / 29 diagnostics / 0 failure(s)`
  with `D32 rows: 0` (was `5`, re-measured against a synthetic root built from `HEAD`) and `proposed cells:
  4`; `run_feature_matrix_probes.sh` -> `probes: 12 pass / 0 fail`
- **D41, found by re-deriving the evidence for D32**: the entry cited `0` for `sed -n '/### G3 /,/### G4 /p'
  ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'`, and that command yields `1` - the range includes
  G3's complexity note, which says "a shirt/trousers intermediate". The conclusion was right (over the exit
  bullet alone, `sed -n '702,708p'`, the count is `0`) and the citation was wrong, in three live places and
  one sealed segment. All three live citations are corrected with the reason; the sealed `part4` line is
  immutable, so this entry is its superseding record. The trap generalises: a section range silently includes
  that section's notes, and `G0-CONTRACT.15` will re-derive every gate clause the same way
- **the dev-notes rollover this slice's own append triggered**: the four oldest lessons are sealed into
  `docs/history/stitchcad-devnotes-part1.md` (`66` lines / `5 589` B / `sha256:d3b94e9a…`), proved lossless
  against `git show HEAD:DEV_NOTES.md` rather than against memory, and the live window is back inside its
  health at `150` lines / `13 183` B. The ledger probe's `DESCRIPTOR` rule now runs over EVERY
  `docs/history/*.md` segment (REAL: `11` -> `13` verdicts) with a `DEVNOTES-DIGEST` arm pinning it ->
  `probes: 8 pass / 0 fail`. Its Coverage and pointer legs stay changelog-only, which is **D40**, owned by a
  new leaf `SPINE.19`, rather than left implicit in a green run
- **the token census fired a fifth time at authoring time**: `` `lining` `` and `` `proposed` `` wore token
  formatting in prose; both were de-tokenized, not exempted -> `276 terms / 8 parts / 145 tokens /
  0 failure(s)`
- **the containment ceiling refused this slice, and the registry's own remedy was the fix**: appending the
  `.4b` checklist put `docs/tasks/G0-CONTRACT.md` at `1182` lines / `104 182` B against a `tasks_collection`
  per-part byte ceiling of `98 304`, so `make gate` blocked with `104182 bytes exceed the byte ceiling`.
  A ceiling rises only by a recorded authority and never to land content, so the nine completed-leaf
  checklists moved to a new sibling `docs/tasks/G0-CONTRACT-evidence.md` (`494` lines / `45 505` B) and the
  tree file is back inside its health at `729` / `62 087`. The convention is recorded in the tree's decisions
  with the gate that forces it: `check_task_acceptance.sh` judges EVERY staged `docs/tasks/*.md`, so the leaf
  being landed keeps its checklist in the tree file - which also means that file's first matching box is now
  always the current leaf's, closing D15's facet 1 by structure instead of by care. `SPINE.md` is past the
  same threshold and owes the same split: **D42**, owned by `SPINE.4.4`
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`; `make book` ->
  exit=0; standards census -> `6 registered / 6 designations used / 0 failure(s)`; containment OK with the
  matrix's two over-health axes recorded in the leaf
