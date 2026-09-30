# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-09-30)_ — permission is not a criterion, and a proposal must be as visible as the gap it replaces

- D32's five rows were not a documentation gap but a **roadmap** gap: §3.2 promises a collar and trousers,
  ontology §4.7 models buttons and pockets, and no gate's exit criteria proved any of them. G3's note that
  an intermediate "may be inserted without shame" is a licence, and a licence proves nothing at a gate
  review. The amendment this slice proposes closes the *class* — every garment §3.2 names drafts, grades
  and exports — rather than naming four features, so the next envelope addition inherits a proof
  requirement instead of needing its own amendment. Promoted to
  `docs/decisions/decision_d32-proving-gates-proposed-roadmap-amendment.md`.
- **A provisional commitment that reads like a settled one is the same defect in better clothes.** So the
  cells say `(proposed)`, the census gained an A3 advisory printing every proposed cell on every run, and a
  probe arm removes the markers and requires the printed count to fall — an advisory that reads nothing
  prints the same number either way, which is the vacuous-green shape the glossary census shipped once as
  its R1 bug.
- **The engineer prepares the amendment; the director applies it.** The ruling reserved `ROADMAP.md`, so the
  record quotes the current text with its line numbers beside the proposed text, states what happens on
  approval and on rejection, and the matrix acts consistently with a *proposal*. A reservation is not a
  blocker: the work lands, and the one thing that is not the engineer's to do is named rather than done.
- The token census fired a fifth time at authoring time (`` `lining` `` and `` `proposed` `` wearing token
  formatting in prose): both were de-tokenized, not exempted.

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

## _(2026-09-30)_ — a boundary is only real if something enumerates it, and a RED arm must remove the property

- **Writing the feature matrix found a roadmap gap that reading it four times had not.** Roadmap §3.2 puts
  a classic collar and trousers inside the v1 envelope; ontology §4.7 specifies button and pocket objects;
  and no gate's exit criteria in §11 mention any of them
  (`sed -n '/### G3 /,/### G4 /p' ROADMAP.md | grep -ci 'collar\|trousers\|button\|pocket'` → `0`; corrected
  by D41 — a section range also catches G3's complexity note, which says "a shirt/trousers intermediate", so
  the command yields `1` and the count that supports the claim is the exit bullet alone, `sed -n '702,708p'`,
  → `0`). The
  gap was invisible while the envelope was prose, because prose does not have to enumerate. It became
  visible the moment each feature needed a disposition AND a gate, and refusing to invent one produced
  `unnamed (D32)` — five rows the census now prints on every run. General rule: **a table with a required
  column is a census of the prose it replaces.** The column nobody can fill is the finding.
- **A RED arm that mutates one instance of a property tests nothing.** Two of the matrix probe's ten arms
  passed at `exit=0` on the first run: renaming `` `ngo_costing` `` renamed its §10 declaration and its row
  together, and de-citing one of three rows citing `ontology §4.4` left the clause covered. Both reported
  "the census discriminates" while the census was never asked to. The fix was to make the mutation the only
  copy of the property. Corollary worth keeping beside D15's shadowing probe: a suite's value is
  concentrated in its RED arms, so an arm deserves the same suspicion as a green gate — **what else in the
  tree still satisfies this rule?** Promoted to `TOOLBOX.md`'s probe conventions rather than to a layer-C
  record, because that is the file a probe author is directed to read (`CLAUDE.md` step 3) and a record
  would duplicate it; `promotion: declined` is recorded in the leaf.
- **The same slice reproduced the failure mode in the instrument itself.** Replacing a `{2,4}` interval
  with `###+` "for portability" silently stopped matching `##`-level headings, so the ontology-coverage rule
  required three clauses fewer and printed `0 failure(s)`. Nothing in the output distinguished "covered"
  from "not looking". Two guards caught it: a GREEN arm over the real tree (which reports the *count* of
  required clauses, so a shrinking population is visible) and a RED arm over a single citation. Corollary:
  **a census should print the size of the population it judged, not only the number of breaches** — a
  count is the only thing that reveals a rule that quietly stopped looking.

## _(2026-09-30)_ — a vocabulary census is a domain-defect detector, and a green rule may be a vacuous one

- **The instrument found the domain defect, not the reading.** Building the glossary's token census
  required deciding what "accounted for" means, and the only honest answer — a token must be declared by
  a table in the chapter that uses it — immediately failed on the reference fixture: `19` of its `42`
  tokens were declared nowhere (§4 named its derived values in prose while its own formulas used
  `quarter_hip`, `front_dart_centre`, `sa_cb`). Forcing each one to a declaration is also what exposed
  **D27**: `sa_wb_bottom` had no declaring row because the waistband it belongs to is described two
  incompatible ways in the same chapter (§4's `2 × wb_width + … = 10.0 cm` is one band folded lengthwise;
  §6's piece list is a faced two-piece band at `6.0 cm` each, and §8 sews only the outer one). A
  prose-only fixture hides that; a fixture that must name every value cannot. Promoted to
  `docs/decisions/decision_machine-tokens-declared-where-used.md`.
- **A rule that reports `0 failures` may have checked nothing.** The census's reference rule printed
  `dead references: 0` on the first run, and the honest reading is not "the glossary is clean" but "did
  this rule ever fail?". It had two bugs: it read the *meaning* column instead of the object column, so
  no cell was ever examined; and it sliced the clause with `substr(link, RSTART + 1, …)` where `§` is
  **two bytes** and `LC_ALL=C` makes awk count bytes, so every extracted clause carried a stray `\xa7`.
  Only the `DEAD-CLAUSE` RED arm — which mutates a copy and demands a refusal — showed it: the arm failed
  to fire. Corollary worth keeping: **a probe suite's value is concentrated in the arms that fail on
  purpose; a suite of GREEN arms is a report, not a test.** This is the third time this repository has
  measured that shape (D15's shadowing probe, D20's untracked producer, D25's misclassified prose).
- **Path arithmetic deserves the same suspicion.** `resolve()` collapsed `a/b/../c.md` by deleting `/../`,
  which leaves `a/b/c.md` — the parent segment has to go with it. Every one of 155 references was reported
  dead until the pattern became `/\/[^\/]*\/\.\.\//`. Relative-link resolution is where a census silently
  becomes a complaint, because both failure directions look like content problems.
- **A rollover is a transaction, not an append.** `CHANGELOG.md` crossed its health target on this slice,
  and doing it by hand reproduced the two defects the ledger probe now checks: the window was not in
  commit order (D29 — so "seal the oldest" seals the wrong end) and part1's descriptor claimed coverage
  through an entry that never left the live window (D30). The containment doctrine already said this
  ("run link, freshness, ordering, uniqueness, retrieval … checks"); what was missing was the executable.
  The sealed-content digest is now re-computed on every `make probes`, and part1's `365` lines still
  reproduce `sha256:f4aec75a…` — so immutability is a measured property, not a policy sentence.

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

