# A fixture's oracle must re-derive the finished dimension, not only the declared quantities

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.13c`; established by re-deriving the reference fixture's waist
  geometry and finding a 28.0 cm error its own invariant could not see (defect **D33** in
  `docs/tasks/PLANNING.md`)

answers: "how do I know a fixture's numbers are right?" · "why does the reference skirt carry two closure checks?" · "what makes a golden-file fixture trustworthy?" · "how could a normative chapter be arithmetically wrong and still pass its own oracle?" · "what do I check before freezing a fixture as a golden?"

## The fact / decision

1. **A fixture carries at least two kinds of oracle, and they are not interchangeable.** A *declared*
   oracle re-computes a relationship between quantities the fixture states (an allocation, a sum, a
   ratio). A *constructed* oracle re-derives a **finished dimension** from the geometry the recipe
   actually builds. Only the second one can see a misplaced point.
2. **Before a fixture is frozen as a golden, every finished dimension it declares is re-derived from its
   own recipe** — not from its declared quantities, and not by reading. For the reference skirt that means
   the finished waist, hip and hem circumferences, the panel widths and the seam lengths, each computed
   from the drafting steps rather than from the measurement table.
3. **A check that passes is evidence about what it measures.** The reference skirt's allocation balance
   closed exactly (`28.0 cm = 28.0 cm`) through the whole life of a drafting step that produced a garment
   28.0 cm too small at the waist. The balance was not wrong; it was answering a different question, and
   the chapter had described it as "the fixture's own invariant", which read as sufficient.
4. **An oracle belongs in the normative chapter, not only in a test.** `waist_closure` is a row of the
   fixture's derived-value table with its formula beside it, so a reader drafting the garment by hand
   performs the same check the implementation will.

## Why

Measured, on the fixture every G2 golden, mutation test, offset pathology and agent gate is built around:

| | as written | required by §3 and §4 |
| --- | --- | --- |
| waist side point, from CF | `quarter_waist − ss_suppress` = **15.5 cm** | `quarter_hip − ss_suppress` = **22.5 cm** |
| stitched quarter waist | `15.5 − 4.0` = 11.5 cm | `22.5 − 4.0` = 18.5 cm |
| finished garment waist | **46.0 cm** | **74.0 cm** (the declared measurement) |
| allocation balance | `28.0 = 28.0` — **passes** | `28.0 = 28.0` — passes |

The side seam takes its 3.0 cm of suppression off the **hip** width, not off the waist width; the earlier
text subtracted it from the waist, so the panel's waist edge was drawn 7.0 cm short and the dart then took
another 4.0 cm out of it. Every declared quantity in the chapter was still consistent — the measurements,
the ease, the suppression split, the dart intake — which is exactly why the balance could not see it. The
error was in *where a point is constructed*, and no relationship between declared quantities mentions that
point.

The general shape: **a parametric fixture can be internally consistent and externally wrong.** Consistency
checks are cheap and necessary; they are not sufficient, because the failure mode of a recipe is a step
that builds the wrong geometry from the right numbers.

## How to apply

- **Writing or reviewing a fixture:** list its finished dimensions and re-derive each from the drafting
  steps. Add the derivation to the fixture as a row with its formula, so the oracle is readable and
  reviewable rather than buried in a test.
- **Freezing a golden:** the question is not "do the checks pass?" but "which check would fail if a point
  moved 5 mm?". If none would, the fixture has no constructed oracle.
- **At G3** (bodice, set-in sleeve) and for every later fixture: the armscye and cap lengths, the eased
  differential and the collar roll are finished dimensions, so each needs its own closure check — the
  sleeve's is that the cap, less its declared ease, equals the armscye.
- **When a check is described as "the invariant":** say which quantity it closes over. A balance over
  declared quantities and a closure over constructed geometry are different guarantees, and prose that
  conflates them is how a 28.0 cm error survived a chapter that called itself checked.

Related: [[decision_machine-tokens-declared-where-used]] · [[decision_numerical-contract-fixed-point]] ·
`docs/book/src/spec/reference-skirt.md` §4.1 (the two closure checks) ·
`docs/tasks/PLANNING.md` (defect D33).
