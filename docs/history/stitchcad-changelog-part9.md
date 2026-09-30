# Sealed archive — StitchCAD changelog, the two oldest live entries

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the
live window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 90 lines, 8386 bytes, `sha256:8e4081d4137fe5b4b218262c6dad9d7227310fbaa875a1e9ccf823c663d90431`
- **Sealed by:** leaf `G0-CONTRACT.11` (the append that crossed the rollover milestone performed the
  rollover, as the doctrine requires).
- **Coverage:** `STITCHCAD-G0-0004c` and `STITCHCAD-SPINE-0004d`, newest first, exactly as they stood in
  `CHANGELOG.md`. **No slice range is declared**, deliberately: the "slices N–M" numbers in the earlier
  descriptors have no producer and disagree with a derivation from git —
  `git log --reverse --format='%s' | grep -oE 'STITCHCAD-[A-Za-z0-9]+-[0-9]+[a-z]?' | awk '!s[$0]++'`
  puts `G0-0013d` at 37 where part7 says 34, and `G0-0014` at 39 where part8 says 41. Sealed segments are
  immutable, so the correction is recorded here and in the live pointer rather than by editing them, and
  the underivable number stops being propagated (defect **D51**, owned by `SPINE.19`).
- **Retrieval:** this file, or `git log --follow CHANGELOG.md`.
- **Write policy:** none — sealed segments are immutable.

---

## STITCHCAD-G0-0004c - the envelope criterion is law, not a proposal (leaves `G0-CONTRACT.4c`, `SPINE.4.5`, `PLANNING.6`, `G0-CONTRACT.14b`)

The director delegated the three findings outright, so the amendment `.4b` had prepared as a *proposal* is ruled
approved and applied - through the roadmap's own machinery, and with two mechanisms the application exposed as
missing.

- **roadmap v0.3**: §11 G3 gains an *envelope coverage* exit criterion - every garment §3.2 names drafts,
  grades and exports at that gate or an earlier one; the A-line skirt at G2, the bodice and sleeve at G3, a
  classic collar with stand, fall and roll line, trousers carrying at least one pocket and one closure whose
  buttonhole length is derived from its button - and "a garment the envelope names and no exit criterion proves
  is a gate failure, not a scope note". The old note that an intermediate "may be inserted without shame" is
  rewritten so an intermediate can never substitute for a criterion. Marked where the roadmap's revision policy
  requires: the title, the status block, an Appendix A disposition entry naming its source and the defect it
  closes, and the end line. `947` lines / `52 818` B, +28 / +1 997
- **the criterion arrives with owners**, because a roadmap clause no leaf owns is D32 one level up:
  `G3-GRADING.5` became required (trousers + pocket + derived buttonhole, with a mutation test that editing the
  button changes the hole), `G3-GRADING.15` was created (the collar, its roll line never exported as a cut
  line), and `.14`'s exit review now fails if a §3.2 garment has no leaf's evidence. The four matrix cells
  dropped `(proposed)` -> census `105 rows / 29 diagnostics / 0 failure(s)` with `D32 rows: 0` and
  `proposed cells: 0`; the A3 advisory and the `(proposed)` mechanism stay for the next one, and its probe arm
  was rebuilt to pin A3 in BOTH directions (0 real, 1 injected) instead of asserting a count that no longer
  exists
- **SPINE.4.5, the mechanism the application needed**: the roadmap's transition-debt baseline was measured at
  exactly the file's size, so v0.3 could only land by hand-widening a number the doctrine forbids widening. The
  debt column now accepts `at=<revision>` and the checker REFUSES a baseline whose revision the file's first
  line no longer declares - a revision may re-base its baseline, but only in the commit that revises, which is
  where the authority has to be anyway. Declaring the right revision does not license growth past the baseline.
  This is the containment adoption note's deferred trigger 3 (a stored copy of a mechanically owned value needs
  an executed freshness oracle) fired and discharged locally: six lines in the existing awk evaluator, not the
  2 100-line neutral interpreter. `--self-test` `11` -> `15` arms; the probe suite gained `REAL-3`, which stales
  the real registry to `at=v0.2` and requires the refusal -> `probes: 5 pass / 0 fail`
- **PLANNING.6 / D44, found because a census was finally run**: the tree-coverage census defined a tree by
  FILENAME, so the evidence siblings the containment registry prescribes were reported as lane-less orphans -
  `15 trees / 1 unowned / 4 orphan(s)`, red for two committed slices, because `make probes` globs
  `run_*probe*.sh` and nothing else ran it. A tree is now recognised structurally by its `- Tree ID:` line in
  both populations AND in the advisory loop (three copies of one assumption; the advisory one hid `G0-CONTRACT`
  itself behind its sibling and no exit code could show it), a non-tree file must be linked from a tree or it is
  a stray, and `run_tree_coverage_probes.sh` puts the census under `make probes` with seven arms ->
  `census: 10 lanes / 13 trees / 2 sibling(s) / 0 unowned / 0 orphan(s) / 0 dead link(s)`, `probes: 7 pass / 0 fail`
- **G0-CONTRACT.14b, the second finding**: no human is named and none is invented. Governance §8 now states
  which seats are held ACTING by the director (project owner, procurement - authority he already holds, so the
  arrangement adds a record and no power) and which is openly VACANT (the sewing/factory domain expert, because
  competence cannot be acted). An acting holder may not confirm the fixture's `assumed` constants, sign a
  golden's semantic half, rule a safety term, or approve a byte-changing profile - so the first G2 golden stays
  gated on a real name, and §8.2 writes the ask per seat so naming one is a single act. The fixture and
  standards chapters now cite the vacant seat instead of a leaf that cannot name anyone
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `13 suite(s) green`; `make book` ->
  exit=0; glossary `276 terms`, standards `6 registered`, fixture `20 rows / 4 checks / 5 pieces`, ledger
  `9 pass / 0 fail`, all 0 failures; containment OK with the roadmap re-based under its record

## STITCHCAD-SPINE-0004d - the widest-line target is derived from the cell budget (leaf `SPINE.4.4`)

The containment check printed a warning on every run that no defect stood behind: `book_collection: widest
line 272 B = 136% of its 200 B target`, where the 272-byte line is a five-column termbase row and the 200-byte
target was derived from the shape of a prose chapter. The director's ruling of 2026-09-30 made re-deriving it
the fourth of its four items.

- **the target is now a cell budget, measured**: the binding shape is the glossary termbase
  (`Term｜What it means｜Canonical object｜Also called｜Machine token`, 276 data rows, 5 columns), whose
  per-column p95 cells sum to 259 B, plus the GFM separator overhead `3 x columns + 1` = 16 B -> **275 B**.
  Not a round number, and not today's widest line: the registry forbids deriving health from the largest file,
  and for a maximum axis that prohibition bites hardest, because the widest line is one row's accident while a
  per-column budget is what each column must carry
- **the producer is tracked**: `docs/tasks/artifacts/live_doc_size/run_cell_budget_census.sh` walks every
  table in a population, reports each shape's per-column widest and p95 cells and both derived budgets, and
  REFUSES (exit 1) when its recommendation does not cover the population's widest actual line ->
  `cell budget: 36 shapes / 625 data rows measured / recommended maxline health 275 B`, exit=0. Its
  `--self-test` pins the arithmetic on a synthetic table whose budget is known by construction ->
  `probes: 7 pass / 0 fail`
- **the ceiling rises by record, from 320 to 440 B**: the shape's own worst legitimate row (every column at
  its widest at once) is 379 B, which the old ceiling would have refused, and 440 = 1.6 x health is the ratio
  every other row in the registry uses. Authorised by
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`, which also records the two
  rejected alternatives with their arithmetic - raising health to 379 to silence the warning (then the
  denominator is the worst case, and a prose chapter could carry a 500-byte line), and splitting the
  collection into prose and table rows (measured: `PARTOF[$path] = sid` means the LAST matching row wins, and
  one registry field holds one glob, so a split needs `:(glob)` pathspec magic and three rows; deferred with
  the trigger that would reopen it)
- **the warning survives, and now means something**: 272 B is 99% of 275 B. No health target at or below the
  old 320 B ceiling could have silenced a 272-byte row - it would need more than 340 B - which is a structural
  property of a maximum axis, recorded so nobody re-derives it as a surprise. The remedy that would clear it
  is a table-authoring convention, which is `SPINE.15`'s beside defect D22, not a bigger number
- **D42 fixed on the way**: SPINE's 17 completed checklists moved byte-identically into
  `docs/tasks/SPINE-evidence.md`, taking the tree from 1096 lines / 88341 B to 552 / 42028 - the convention
  `G0-CONTRACT.4b` recorded, performed by the leaf D42 named
- gates: `check_live_doc_size.sh` -> `OK - 17 surfaces, 15 routes, 80 files measured`, exit=0, and its
  `--self-test` -> `11 arms, 0 failed`; `run_live_doc_size_probes.sh` -> `probes: 4 pass / 0 fail`;
  `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`. `.doctrine/` changed, so
  the immediate-push exception fires: this slice pushes at once and records the observed CI verdict in the
  leaf
