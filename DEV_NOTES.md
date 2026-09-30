# DEV_NOTES.md

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-09-30)_ — a rule whose only compliant path is "don't change the file" gets bypassed

- Applying the roadmap amendment was blocked by the containment gate, and blocked *correctly*: the `roadmap`
  row's transition debt was measured at exactly the file's size (`lines=919;bytes=50821` = `wc -lc`), so any
  growth read as a widened baseline. The rule is right; what was missing was a legitimate path, and the only
  one available was editing the number — the silent widening the rule exists to prevent. **When a gate's only
  passing move is to falsify its own input, the gate is incomplete, not the change.** The fix was to give the
  baseline a revision identity (`at=v0.3`) that the checker executes: a revision may re-base its baseline, but
  only in the commit that revises, which is where the authority has to be anyway.
- **An exit criterion must arrive with owners.** The same commit that gave G3 the envelope-coverage criterion
  made `G3-GRADING.5` required, created `.15` and made `.14`'s review fail without a leaf's evidence — because
  a roadmap clause no leaf owns is defect D32 one level up, and the tree was seeded from the gate's old text so
  it inherited the gate's gap exactly.
- **A census nobody runs is a claim, and this one had gone red for two committed slices.** The tree-coverage
  census defined a tree by *filename*, so the evidence siblings the containment registry prescribes looked like
  lane-less orphans — and `make probes` globs `run_*probe*.sh`, so nothing re-derived it. Three copies of the
  same `ls ${lane}-*.md | head -1` assumption existed; the third was in an *advisory* table, where it silently
  replaced `G0-CONTRACT` with its sibling and no exit code could reveal it. Reading a tool's output, not just
  its status, is part of running it.
- **Office can be held acting; competence cannot.** Two of the three empty governance seats are the director's
  authority already, so acting costs nothing and adds a record. The third — the sewing/factory expert — is
  knowledge nobody here has, so it is recorded as **vacant** with four explicit prohibitions, and the first G2
  golden stays gated on a real name. The tempting move (let the engineer act as reviewer) satisfies the wording
  of the two-step rule and destroys its meaning.

## _(2026-09-30)_ — a digest is a contract about BYTES, so the bytes have to be written down

- Sealing the third changelog segment of the day produced a refusal that read as the worst thing an archive
  can report — "content hashes to `3148dd0f…`, its descriptor declares `8553accc…`" — and the cause was one
  newline. The verifier hashes `content=$(sed -n 'rule+2,$p' f)` followed by `printf '%s\n'`, and bash command
  substitution strips EVERY trailing newline, so a segment whose sealed content ends with a blank line can
  never reproduce a raw-byte digest. Measured: `tail -c 12` showed `…touched\n\n` on the new segment and
  `…touched\n` on the one sealed an hour earlier.
- **The general shape: two honest implementations of "the sealed content" disagreed, and nothing said which
  one was the contract.** A digest proves identity only when both sides mean the same bytes, so the byte rule
  belongs in the instrument's header, not in the author's head. Fixed by normalizing the segment, recomputing
  its descriptor with the verifier's own method, teaching the rule to refuse a trailing blank line BY NAME,
  and pinning that with an arm — because a mismatch that looks like drift in an immutable file invites the one
  edit the archive forbids.
- **Corollary for the next rollover:** seal with the verifier's method, not with the language you happen to be
  scripting in. The check is one command (`sed | shasum`), so running it before writing the descriptor is
  cheaper than diagnosing the difference afterwards. Promoted, with the day's other containment derivation, to
  `docs/decisions/decision_maxline-health-derived-from-the-cell-budget.md`.

## _(2026-09-30)_ — a blocked leaf splits into what can be drafted and what must be named

- `.14` sat blocked for a session because it needed three named humans. The director's ruling separated the
  two halves — draft everything, name nobody — and the whole chapter turned out to be draftable: a role is
  the **decision it may make**, not the person holding it, so each seat could be specified with its authority,
  its agent-eligibility and what waits for it. What was blocking was the *coupling* of the two halves, not the
  missing names. General shape: before calling a leaf blocked, ask which of its clauses actually needs the
  missing input; usually one does and the rest were waiting beside it.
- **Governance written while the room is empty is a contract; written after the first conflict it is a
  negotiation.** Roadmap §14's mitigation is literally "governance doc at G0, while the room is empty", and
  the reason shows up in the drafting: every rule that had to be invented (which changes take the domain
  path, what makes a reviewer independent, who signs a golden) is cheap to state now and expensive to state
  after somebody has a position. Promoted to
  `docs/decisions/decision_governance-two-review-paths-and-the-unnamed-roles.md`.
- **A fallback that does not state its cost is how a slip becomes a silent downgrade.** The eval-seat fallback
  (a partner runs the test) is roadmap-mandated, so the only honest addition was what it costs: layer-4
  evidence with recorded product, version and settings — slower, fewer targets. Same shape as a claim with a
  missing leg: name the leg, do not hide it.
- **The empty seat with a schedule behind it is the one to name loudly.** The domain expert gates the fixture's
  `assumed` constants, which gate the first G2 golden, so the chapter says so where a plan will hit it rather
  than where a reader will sympathise.

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

# Sealed archive — earlier lessons

| Segment | Coverage | Sealed identity |
| --- | --- | --- |
| [`devnotes-part1.md`](docs/history/stitchcad-devnotes-part1.md) | the `2026-09-29` and `2026-09-04` lessons, plus the bootstrap entry | 66 lines, 5589 bytes, `sha256:d3b94e9a…` |
| [`devnotes-part2.md`](docs/history/stitchcad-devnotes-part2.md) | the two oldest `2026-09-30` lessons (enumeration, and the vocabulary census) | 62 lines, 5915 bytes, `sha256:edcd0808…` |

The live window below holds the most recent lessons. When it passes its health target (200 lines /
16 384 bytes) again, the oldest entries are sealed the same way, and the `DESCRIPTOR` rule of
`run_changelog_ledger_probes.sh` proves the digest afterwards.

