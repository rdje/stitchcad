# Sealed archive — StitchCAD changelog, slices 34–35

Immutable historical segment, sealed out of `CHANGELOG.md` under the rollover rule recorded in
`.doctrine/live_document_size/surfaces.tsv` (row `changelog`, lifecycle `rolling_ledger`): when the live
window passes its health target, the oldest entries are sealed here and a pointer is left behind.

- **Sealed identity:** 105 lines, 9793 bytes, `sha256:ff62d41856a65e7935c6259b811bd11e2f5e97aec9339c25453737212df76119`
- **Sealed by:** leaf `SPINE.15` on `2026-09-30` — the append that crossed the rollover milestone
  performed the rollover in the same commit, as the doctrine requires.
- **Coverage:** slices 34–35 — `STITCHCAD-G0-0013d`, `STITCHCAD-G0-0008`, newest first, exactly as they stood.
- **Predecessors:** `stitchcad-changelog-part1.md` … `part6.md`. part1’s own coverage line overstates its
  range by one slice; the correction is recorded in part2’s descriptor and in the live pointer (defect D30).
- **Byte contract:** the sealed content is everything below the `---` rule and the blank line after it, and
  it ends with exactly one newline — the contract `run_changelog_ledger_probes.sh` states after defect D43,
  whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.

---

## STITCHCAD-G0-0013d - one waistband, and the instrument that keeps it one (leaf `G0-CONTRACT.13d`)

The reference fixture described two different garments at once for nine commits: §4 published the cut width
of ONE band folded lengthwise while §6 listed a faced two-piece band and §8 sewed only the outer one
(defect D27). The director ruled on 2026-09-30 that the engineer decides it; this slice decides it, and
makes the agreement between the chapter's tables mechanical rather than a matter of reading.

- **the decision**: one straight band, cut once, folded lengthwise at its midpoint (5.0 cm from either long
  edge), plus one interfacing piece cut at the band's finished dimensions (77.0 x 4.0 cm, no allowance on
  any edge) and fused inside the seam lines. Five pieces, not six - `waistband_outer` and `waistband_inner`
  no longer exist anywhere in the book
- **sourced, not preferred**: five references read on this machine on 2026-09-30, each cited by URL and date
  in `docs/decisions/decision_reference-fixture-waistband-straight-folded.md` under a `read-external` label
  that upgrades no claim about a standard. The decisive fact is that the drafting literature gives the
  straight band ONE rectangle with a fold line and reserves "cut two pieces - one for the outer waistband,
  and one for the inner waistband" for the **contoured** band, while this fixture's waist sits at the natural
  waist where a straight band belongs. The sources disagree about band height (2-5 cm against a 3 cm maximum
  for a straight band), so `wb_width` = 4.0 cm stays `assumed` with the conflict recorded rather than quietly
  resolved in favour of the convenient source
- **every piece is accounted for**: §8 gains an attachment account - a span, or a declared non-sewn method
  from a closed list that holds `fused` alone. "Every piece has a span" is false for a fused interfacing,
  and believing it is what let an inner band nobody sewed pass for nine commits
- **the numbers are derived, not read**: §4 gains `waistband_fold_position` 5.0, `waistband_finished_length`
  77.0, `wb_interfacing_width` 4.0, `wb_interfacing_length` 77.0 and the band's two closure checks
  (`waistband_pattern_length - 2 x sa_cb - wb_extension = garment_waist` -> `74.0 = 74.0`, and
  `waistband_cut_width - sa_waist - sa_wb_bottom = 2 x wb_width` -> `8.0 = 8.0`), so §4.1 carries four
  oracles: two for the body, two for the band
- **a tracked producer at last**: `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh`
  evaluates every §4 formula over the tables above it, checks the four closures, the piece account, §12's
  published count against §6's list, and that §4's band width describes the same construction as §6's band
  pieces (rule `B1`). Over the chapter as committed: `16 derived rows / 2 closure checks / 6 pieces /
  7 mismatch(es)`, exit=1, naming D27 by shape. Over the chapter now: `20 / 4 / 5 / 0 mismatch(es)`, exit=0.
  Until this slice the arithmetic that found D33 and then D27 lived as `python3 -c` strings inside task
  leaves - re-runnable by nobody, which is the leg-3 breach this repository already committed once as D20
- **the probes are shown to discriminate, not merely to pass**: nine arms (one GREEN, eight RED) ->
  `probes: 9 pass / 0 fail`, and neutering one rule at a time in a scratch copy reddens exactly the arms
  that depend on it (`B1` removed -> `8 pass / 1 fail`; `P2` removed -> `7 pass / 2 fail`)
- **the token census corrected the chapter a fourth time at authoring time**: four undeclared tokens on the
  first draft - `Fold` (a column name of this chapter's own table), `fused` (a machine enum value a program
  reads, so it became a glossary term with one owner), and the two names of the rejected reading. The fix was
  the convention rather than an exemption: prose lost its backticks and the dead piece names survive in the
  layer-C record for anyone tracing the defect -> `276 terms / 8 parts / 145 tokens / 0 failure(s)`
- **not decided, deliberately**: whether a `SeamSpan` may name the same piece on both sides - a folded band's
  short ends are stitched across, which joins one piece to itself. Logged as D35 with `G1-SLICE.3` as owner;
  §8 records the ends as an edge finish instead of settling a type invariant by accident
- **the ledger probe refused an honest changelog, and the probe was the thing that was wrong** (D39): its
  work-unit id shape was `[0-9]+[a-c]?`, so this slice's own id `STITCHCAD-G0-0013d` parsed as
  `STITCHCAD-G0-0013` and collided with the sealed entry of that name -> `NO-DUP FAIL both live and sealed:
  STITCHCAD-G0-0013`, a duplicate that did not exist, plus an `ORDER FAIL` comparing the wrong commit. The
  same collapse hides a real mis-ordering between `-0013d` and `-0013e`, so one regex produced a false red
  AND a possible false green. Fixed to `[a-z]?` in all six places and pinned by a new GREEN arm `SUFFIX`,
  shown sensitive by reverting the class in a scratch copy -> `probes: 5 pass / 2 fail`; after the fix
  `probes: 7 pass / 0 fail`
- **also in this commit**: D36 logged and fixed (`LIVE_STATUS.md`'s spine row reported 13 universal gates and
  7 probe suites where `make gate` prints 12 universal plus the project slot's 2 and `make probes` finds 12);
  D37 logged and fixed (`DEV_NOTES.md` described itself below every lesson, at line 149 of 156); D38 logged
  (the census's open/closed counts are prose, so a naive derivation reports 33 closed and misses D34 —
  `PLANNING.5`'s goal is extended to derive them); D34's recurrence recorded - the commit that created `.13d`
  and `.4b` moved the frontier without writing `docs/TASK_TREE.md`, so the index named a leaf four commits
  done, and all three stale rows are corrected
- gates: `make gate` -> `=== all doctrines green ===`; `make probes` -> `12 suite(s) green`; `make book` ->
  exit=0; feature-matrix census -> `105 rows / 29 diagnostics / 0 failure(s)`; standards census ->
  `6 registered / 6 designations used / 0 failure(s)`; containment OK with the chapter and the record both
  over health and inside every ceiling, recorded in the leaf; no product code touched

## STITCHCAD-G0-0008 - ADR-0001: licence and solver are one decision (leaf `G0-CONTRACT.8`)

`docs/decisions/decision_adr-0001-license-and-solver.md` settles roadmap §5's ADR-0001 **before any solver
code exists**, which is when §5 says it must be settled: an external contributor arriving at G1 inherits a
decision, not a default.

- **the decision**: the core is dual `MIT OR Apache-2.0` (re-read from `Cargo.toml`, not recalled:
  `grep -n license Cargo.toml` -> `license = "MIT OR Apache-2.0"`), so the SolveSpace solver `slvs`
  (GPLv3) is **rejected**, and `sc-sketch`'s optional local constraints get a custom kernel or a numeric
  least-squares solver written for this repository. Licence and solver are one record because neither is
  decidable alone - the solver choice IS the licence choice
- **the exchange rate is stated, not asserted**: linking `slvs` copylefts the workspace, which decides who
  may embed the product - and the persona roadmap §1.2 names is a patternmaker producing a package for a
  *named factory*, with factories and partners embedding the tool. What `slvs` would buy is an *optional
  annotation layer* (ADR-0003 makes the recipe primary and sketch constraints local), while the production
  solver is custom anyway (§6.1's deterministic finite-domain CP engine, so native and WASM share one
  determinism argument). Trading the licence of the whole product for the second solver of an optional
  feature is the bad exchange this record refuses
- **BSL-1.1 is kept out of the permissive category**, as §5 demands: a use restriction is a commercial term
  needing the project owner's own review, and a downstream embedder cannot treat BSL code as MIT. Adopting
  it would be a new ADR superseding this one
- **consequences written down before they are needed**: no CLA to contribute; no GPL or GPL-incompatible
  dependency in a shipped crate, enforced by the re-runnable census `G1-SLICE.15` already owns; Z3 stays a
  CI-only differential oracle behind `csp-z3` (its MIT licence would permit shipping it - the reason it is
  not shipped is the C++ dependency and the browser profile, and this record does not reopen that); and an
  iterative least-squares kernel is NOT covered by the numerical contract's exactness argument, so if it
  ever lands it must quantize into the internal units with a declared tolerance class, a fixed iteration
  bound and a declared convergence test - never "until it looks converged"
- **one re-open condition, in three parts stated now** so the G1 spike cannot be argued into satisfying one
  of them later: the spike shows a capability the custom kernel cannot reach within scheduled effort on a
  realistic pattern set; a named persona or partner wants it; and the project owner accepts the licence
  consequence in writing. A partial re-open is allowed in one direction only - `sc-sketch` licensed apart
  from the core if it stays an optional crate nothing in the default build links, the shape §6.1 uses for Z3
- the rollover this append triggered is performed in the same commit: slices 25-27 sealed into
  `docs/history/stitchcad-changelog-part4.md` with a digest-verified descriptor, the pointer table rebuilt
  from every segment on disk so it cannot drift, all pre-existing entry bodies byte-identical, and
  `run_changelog_ledger_probes.sh` green on the result
- gates: `make gate` -> "=== all doctrines green ==="; `make probes` -> "11 suite(s) green"; `make book` ->
  exit=0; no product code touched and no book chapter added (this leaf's deliverable is the record)
- LIVE_STATUS: the G0 frontier moves to .9, ADR-0003 and the formula language
