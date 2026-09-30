# Sealed archive — StitchCAD dev notes, the two next-oldest 2026-09-30 lessons

Immutable historical segment, sealed out of `DEV_NOTES.md` under the rolling-ledger protocol in
`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, performed by leaf `G0-CONTRACT.9` in the commit whose append
crossed the window's byte health target (200 lines / 16 384 bytes).

- **Sealed identity:** 50 lines, 4723 bytes, `sha256:fcca661da27862a0e5cf2564d4f9b0358966b116598d9abb0619067d45a815d4`
- **Coverage:** the lessons `two tables can each be right and together describe two garments` and
  `a fixture can be internally consistent and externally wrong`, both dated 2026-09-30, oldest last,
  exactly as they stood.
- **Sealed by:** leaf `G0-CONTRACT.9` on `2026-09-30`, after `part2` (sealed by `SPINE.4.4`).
- **Retrieval:** the sealed content is everything below the `---` rule and the blank line after it, and
  it ends with exactly one newline — the byte contract `run_changelog_ledger_probes.sh` states after
  defect D43, whose `DESCRIPTOR` rule reproduces this digest on every `make probes`.

---

## _(2026-09-30)_ — two tables can each be right and together describe two garments

- D27's shape: §4's `waistband_cut_width` formula and §6's piece list were each internally consistent — the
  contradiction was *between* tables. Every instrument the chapter had read one table at a time (the token
  census reads names, the containment checker reads bytes), so nothing could see it; a human writing a
  sentence about the piece list did. **An invariant that spans tables needs an instrument that spans
  tables**: `run_fixture_derivation.sh` reads §2, §3, §4, §6, §7, §8 and §12 together and refuses a piece
  list, a span table, an account and a count that disagree — `7 mismatch(es)` over the chapter as committed,
  `0` after the decision.
- **A rule with no legal way to be satisfied is a rule nobody checks.** "Every piece has a span" is false for
  a fused interfacing, so the invariant that holds is a span *or* a declared non-sewn attachment from a
  closed list — `fused` alone today, because a sewn-in interlining is sewn and needs a span. Declaring the
  exception is what made the rule checkable; keeping the list closed is what stops it becoming a shrug.
- **The instrument that found two defects was scratch.** The arithmetic behind D33 and then D27 lived as
  `python3 -c` strings inside task leaves: readable, and re-runnable by nobody. That is leg 3 of
  `CLAIM_VERIFICATION.md`, and the breach this repository had already committed once as D20.
- **A choice between two real constructions is not arithmetic.** Both waistband readings are real skirts, so
  the decision needed sources, and what settled it was a *scope* fact rather than a preference: the drafting
  literature gives a straight band one folded rectangle and reserves the two-piece cut for a **contoured**
  band. Recorded as `read-external` with URL and date — a label that upgrades nothing, because a source read
  here is not a standard read here (`standards.md` §1 keeps that vocabulary closed).
- **When sources disagree, keep the declared constant and record the conflict.** Band height is "usually
  2–5 cm" in one and "maximum 3 cm for a straight band" in another; the fixture's 4.0 cm stays `assumed`
  with both cited. Adopting whichever source suits is how a specification launders a guess into a fact.
- Promoted to `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`.

## _(2026-09-30)_ — a fixture can be internally consistent and externally wrong

- The reference skirt's allocation balance closed exactly — `4 × (3.0 + 4.0) = 102.0 − 74.0`, i.e.
  `28.0 = 28.0` — for the whole life of a drafting step that produced a garment **28.0 cm too small at the
  waist**. §5 step 2 put the waist side point at `quarter_waist − ss_suppress` = 15.5 cm from CF; the side
  seam takes its suppression off the *hip* width, so the point belongs at `quarter_hip − ss_suppress` =
  22.5 cm, and the finished waist is `4 × (22.5 − 4.0)` = 74.0 cm instead of `4 × (15.5 − 4.0)` = 46.0 cm.
  Re-derived with `python3`, not with reading: the chapter's declared quantities were all consistent, so no
  consistency check could have caught it. Promoted to
  `docs/decisions/decision_fixture-oracles-derive-the-finished-dimension.md`.
- **The general shape:** a *declared* oracle (a balance, a sum, a ratio over quantities the fixture states)
  and a *constructed* oracle (re-deriving a finished dimension from the geometry the recipe builds) answer
  different questions, and only the second sees a misplaced point. A fixture needs both. The question that
  exposes the gap is not "do the checks pass?" but **"which check would fail if a point moved 5 mm?"** — for
  this fixture, before the fix, the answer was none.
- **What found it was not a review of the fixture.** It was writing the *instantiation-paths* chapter, which
  needs the fixture's graded numbers as a worked example: grading a waist point requires knowing which span
  it sits on, and the two candidate spans disagreed by 7.0 cm. Corollary: a defect hides best in a document
  nobody has a second use for. The first slice that has to *compute with* a specification is the slice that
  tests it — which is an argument for writing the dependent chapter early rather than for re-reading.
- **A correction to a sealed record is a superseding record, not an edit.** The sealed `STITCHCAD-G0-0013`
  changelog segment lists "dart intake and centre" among the derived values; it is immutable, so §11 of the
  chapter carries the correction and names the segment it supersedes. The ledger probe's `COVERAGE` rule
  already needed the same shape for part1's wrong coverage line (D30) — two instances of one rule now.
