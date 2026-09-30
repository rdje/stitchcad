# Five envelope features get a proving gate — and the roadmap amendment that carries four of them is a PROPOSAL

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active` — the record is active; the `ROADMAP.md` amendment it carries is **proposed**, not
  applied, and does not become a commitment until the director rules on it
- **Owner / source:** leaf `G0-CONTRACT.4b`, decided under the director's ruling of `2026-09-30`
  (`decision_director-ruling-2026-09-30-four-findings.md`, item D32). Reservation 2 of that ruling is why
  this is a proposal: **amending `ROADMAP.md` is the director's**, and its changes go through its own
  disposition log. The engineer prepares the exact text; the matrix acts consistently with it as a proposal.

answers: "which gate proves a classic collar?" · "which gate proves trousers, buttons and pockets?" · "why does the feature matrix say `proposed` instead of a gate?" · "what roadmap amendment is waiting for the director?" · "why is a fly deferred to G7 while trousers go to G3?" · "what is the difference between permission and an exit criterion?"

## The decision

Each of the five rows defect **D32** listed as `unnamed (D32)` now names a gate. Four need a roadmap
amendment and say `(proposed)` until it is ruled on; one needs nothing.

| Feature | Disposition | Gate that proves it | Needs an amendment? |
| --- | --- | --- | --- |
| classic collar | `supported` | **G3** | yes — the G3 coverage criterion below |
| trousers | `supported` | **G3** | yes — the same criterion |
| button and buttonhole | `supported` | **G3** (derivation) + **G5** (notions) | yes for G3; G5's text already covers notions |
| pocket | `supported` | **G3** | yes — the same criterion |
| fly construction | `deferred` | **G7** | no — G7's exit already requires named limitations |

Why each lands where it does, in one line each: a collar's stand, fall and roll line are construction
geometry and G3 owns construction (its bodice already supplies the neckline); trousers are the second
domain jump G3's own note contemplates, and a crotch curve, an inseam and a waistband stress the drafting
and grading engine differently from a skirt; a buttonhole's length is *derived from its button* (ontology
§4.7), which is construction semantics, while the notions list that reports it is already a G5 exit
criterion (`ROADMAP.md:733`); a pocket's position, orientation, opening type and piece composition are
construction, and the trousers are the envelope garment that carries one; and a fly is refused in v1 with
`env_fly`, so rule 3 of the matrix's §1 puts it in the gate whose exit is a supported-envelope statement
*with named limitations* — exactly what the `lining` row already does.

## The proposed amendment, quoted exactly

`ROADMAP.md` §11, gate **G3**, lines `701`–`711` as they stand today:

```markdown
### G3 — Construction & grading
- **Exit:** bodice + set-in sleeve with **declared ease** (cap ease is
  intentional, not an invariant violation); darts, folds, walking/truing
  operations; stable references survive edit/split/mirror (or visible repair
  tasks); both instantiation paths; `.rul` with grade-point identifiers
  re-imported into an independent engine; extreme sizes reconstructed and
  measured; grading modes (base+rules / embedded / all-contours) validated
  separately; offset + grading golden suites green.
- Intermediate complexity note: skirt → bodice+sleeve is a domain jump
  (armscye, cap ease, balance notches); a shirt/trousers intermediate may
  be inserted without shame — gates slip on domain evidence, not on engineering.
```

Proposed replacement — the first bullet is unchanged, one bullet is added, the note is rewritten:

```markdown
### G3 — Construction & grading
- **Exit:** bodice + set-in sleeve with **declared ease** (cap ease is
  intentional, not an invariant violation); darts, folds, walking/truing
  operations; stable references survive edit/split/mirror (or visible repair
  tasks); both instantiation paths; `.rul` with grade-point identifiers
  re-imported into an independent engine; extreme sizes reconstructed and
  measured; grading modes (base+rules / embedded / all-contours) validated
  separately; offset + grading golden suites green.
- **Exit (envelope coverage):** every garment §3.2 names drafts, grades and
  exports at this gate or an earlier one — the A-line skirt at G2; the darted
  bodice and set-in sleeve above; a classic collar with a stand, a fall and a
  roll line; and trousers carrying at least one pocket and one closure whose
  buttonhole length is derived from its button. A garment the envelope names
  and no exit criterion proves is a gate failure, not a scope note.
- Domain-complexity note: skirt → bodice+sleeve is a domain jump (armscye, cap
  ease, balance notches) and trousers are a second one (crotch curve, inseam,
  waistband). An intermediate garment may be inserted where the domain evidence
  calls for one — gates slip on domain evidence, not on engineering — but an
  intermediate is a means, and never a substitute for the coverage criterion.
```

No other clause of `ROADMAP.md` is proposed for change. G7's exit already reads "supported-envelope
statement with named limitations" (`ROADMAP.md:746`), which is what the deferred fly row needs, and G5's
tech-pack clause already requires notions (`ROADMAP.md:733`).

## Why an amendment at all, and why this shape

- **Permission is not a criterion.** G3's note says an intermediate "may be inserted without shame". That
  is a licence, and a licence proves nothing at the gate: a trousers block could be skipped, or built and
  never graded, and G3's exit would still pass while roadmap §3.2 kept promising trousers. An envelope
  feature no exit criterion names is a feature G7 must eventually declare untested.
- **Close the class, not the instances.** The added criterion is written over §3.2's garment list, so a
  future envelope addition inherits a proof requirement instead of needing its own amendment — and the
  last sentence names the failure mode, so a gate review can fail on it rather than argue about it.
- **The cost is stated rather than hidden.** This makes G3 heavier: two garments and three features that
  its current text does not require. That is the honest price of the envelope §3.2 already declares, and
  paying it at G3 is cheaper than paying it at G7, where the alternative is publishing a supported-envelope
  statement with four of its own garments untested.
- **What was rejected.** Assigning collar and trousers to G5 (a designer dogfoods a real garment there)
  was considered and dropped: G5 proves the shells and the UX over constructions that already work, so a
  drafting defect found there arrives two gates after the engine that caused it was declared correct.
  Assigning the fly to G3 as a supported feature was also dropped — it would widen v1 scope, and the
  matrix's rule 3 already has an honest home for a capability nobody has scheduled.

## What happens next

1. `G0-CONTRACT.15` (the G0 exit review) puts this proposal to the director, with the four `(proposed)`
   cells listed, and records the ruling.
2. **If approved:** the director applies the amendment to `ROADMAP.md` under its disposition log; the four
   cells drop `(proposed)`; the census's A3 advisory reports `0` proposed cells; and this record gains a
   line naming the commit that applied it.
3. **If rejected:** the four cells revert to `unnamed (D32)`, the census's A1 advisory lists them again on
   every run, and G7's statement must name collar, trousers, buttons and pockets as untested — which is a
   legitimate outcome, but one the release claim has to carry in the open.
4. Until either happens, the matrix is honest about the state: a `(proposed)` cell is a claim about a draft
   of the roadmap, never about the roadmap as it stands, and the census prints all four on every run so the
   proposal cannot be read as settled by accident.

Related: [[decision_director-ruling-2026-09-30-four-findings]] ·
[[decision_product-work-takes-the-frontier]] · `docs/book/src/spec/feature-matrix.md` §1, §9, §12 ·
`docs/tasks/PLANNING.md` (defect D32) · `ROADMAP.md` §3.2, §11 (G3, G5, G7).
