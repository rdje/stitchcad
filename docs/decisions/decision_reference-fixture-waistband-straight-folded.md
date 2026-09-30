# The reference fixture's waistband is ONE straight band, folded lengthwise, plus a fused interfacing

- **Type:** `decision`
- **Date:** `2026-09-30` (absolute)
- **Status:** `active`
- **Owner / source:** leaf `G0-CONTRACT.13d`, decided under the director's ruling of `2026-09-30`
  (`decision_director-ruling-2026-09-30-four-findings.md`, item D27). It is an engineering decision made
  under delegation and remains subject to the sewing/factory reviewer `G0-CONTRACT.14` names — a
  delegation to decide is not a domain certificate.

answers: "how many waistband pieces does the reference skirt have?" · "why is the fixture's band one rectangle and not a faced pair?" · "when is a two-piece waistband the right construction?" · "how wide should a straight waistband be?" · "how do I cite a sewing source that is not a standard?" · "what does a fixture do with a piece nothing sews?"

## The decision

The reference fixture's waistband is **one straight band, cut once, folded lengthwise at its midpoint**,
with **one separate interfacing piece** cut at the band's finished dimensions and **fused** to it. Five
operative rules follow, and the fixture chapter states each with its numbers:

1. `waistband` is ONE piece: a rectangle of `waistband_pattern_length` × `waistband_cut_width`
   (80.0 × 10.0 cm), whose fold line is an **internal construction line** at `waistband_fold_position`
   (5.0 cm) from each long edge — not a fabric fold edge, so its label never says "place on fold".
2. `waistband_cut_width = 2 × wb_width + sa_waist + sa_wb_bottom`: two layers of finished height, plus
   the allowance sewn into the waist seam, plus the turn-under at the free edge.
3. `waistband_interfacing` is a piece of its own, `wb_interfacing_length` × `wb_interfacing_width`
   (77.0 × 4.0 cm), with **no allowance on any edge**, because it is cut to the finished dimensions and
   fused inside the seam lines.
4. Every piece the fixture lists is **accounted for** in the sewing graph — by the spans it takes part
   in, or by a declared non-sewn attachment from a closed list that today contains exactly `fused`. A
   piece with neither is defect D27's shape and is now a refusal, not a prose gap.
5. The band carries **two closure checks** of its own (`waistband_length_closure`,
   `waistband_width_closure`), because its length and its height are finished dimensions and
   `decision_fixture-oracles-derive-the-finished-dimension` requires an oracle per finished dimension.

## Why: the two readings, and what decided between them

| | the reading kept | the reading rejected |
| --- | --- | --- |
| construction | one straight band, folded lengthwise | a faced two-piece band (outer + inner) |
| pieces | `waistband` + `waistband_interfacing` | `waistband_outer` + `waistband_inner` + interfacing |
| cut width | `2 × wb_width + allowances` = 10.0 cm | `wb_width + allowances` = 6.0 cm per piece |
| band form | straight | proper to a **contoured** band |
| piece count | 5 | 6 |

Both are real skirt constructions, so the choice could not be made from arithmetic — the arithmetic was
the *symptom* (§4 published the folded band's width while §6 listed the faced band's pieces). It was made
from what the sources say a **straight band at the natural waist** is, which is what §1 and §2 declare:

**Sources read in this repository on `2026-09-30`** (labelled `read-external`: the URL and the date read
are recorded, the claim is about the source's content, and none of these is a standard — nothing here
asserts conformance to anything, and a `read-external` source never upgrades a claim about a standard,
which stays governed by `docs/book/src/spec/standards.md` §1's closed vocabulary):

- **Minerva Patterns**, *Setting a Waistband on Trousers or a Skirt* —
  `https://minervapatterns.com/blog/setting-a-waistband-on-trousers-or-a-skirt`. "A straight waistband is
  a simple rectangle, interfaced and folded lengthwise, that wraps the waist edge. It works when the
  waistline sits at the true waist." "Interface the full band… Cut the interfacing to the full band
  including any wrap extension." The five-step attach sequence (interface and press the fold line and the
  inner-edge allowance → stitch the interfaced long edge to the garment waist → stitch across each end →
  trim, grade, turn → fold down and secure the inner edge). Also: "the waistband, not the garment body,
  sets the final waist measurement… cut to the actual waist plus wearing ease plus the underlap", which
  is what `waistband_pattern_length` and `waistband_length_closure` say in tokens. Weight: this supplier
  ships made-to-measure patterns in **DXF AAMA, PLT and PDF** — the interchange targets this product
  writes — so its construction vocabulary is the one a receiver of our files already uses.
- **anicka.design**, *Drafting Waistbands for Skirts and Pants* —
  `https://anicka.design/drafting-waistbands-for-skirts-and-pants/`. The **straight** band is drafted as
  one rectangle with a dashed *fold line* (its step 6), extension 3–5 cm, finished width "usually 2–5 cm".
  The instruction to "cut two pieces — one for the outer waistband, and one for the inner waistband"
  appears in its **curved/shaped** band section, not in the straight one. Weight: this is the decisive
  fact — the two-piece cut belongs to a contoured band, and this fixture's band is straight.
- **SewGuide**, *5 types of waistbands for your skirts and pants* —
  `https://sewguide.com/sewing-waist-bands/`. The interfacing rule: "cut the interfacing to the width,
  and length of the dimensions of the final waistband you want. In the second instance, the seam
  allowance is done away with and hence no added bulk when sewing the seams" — which is rule 3 above, and
  is why the interfacing piece carries no allowance. Its folded-band sequence also records that the free
  edge is pressed under and caught by stitching from the right side, and that the band is folded "almost
  half" — see *Not modelled* below.
- **The Shapes of Fabric**, *Experimenting with Waistband and Yoke Details* —
  `https://www.theshapesoffabric.com/2024/04/04/experimenting-with-waistband-and-yoke-details/`. A
  separate facing piece is what **shaped and decorative** bands use ("trace the main piece and its
  facing"; "the waistband has a smooth facing piece"), corroborating the same split. It is also the
  source of the one **disagreement** recorded below.
- **Wikipedia**, *Facing (sewing)* and *Interfacing* — `https://en.wikipedia.org/wiki/Facing_(sewing)`,
  `https://en.wikipedia.org/wiki/Interfacing`. A facing is a separate piece that finishes an edge from
  the inside and is **understitched**; interfacing is **fusible or sewn in**. That distinction is why the
  closed non-sewn list is `fused` alone: a sewn-in interlining is sewn, so it would need a span.
- Read and **not** used: Wikipedia's *Waistband*, which is a garment-history and culture article and says
  nothing about construction. Recorded so the next session does not re-read it.

**Not modelled, deliberately.** Two manufacturing habits sit outside the pattern geometry and stay there:
the fold is pressed a millimetre toward the free edge so a fell stitch cannot miss it (SewGuide), and the
inner edge may be secured by stitch-in-the-ditch or by edgestitching (Minerva). Neither changes a cut
dimension, so neither is fixture data; if either ever becomes a Factory Profile parameter, that is a
profile decision with its own record.

## What stays `assumed`, and the one disagreement between sources

`wb_width` = 4.0 cm is inside anicka.design's "usually 2–5 cm" and **above** The Shapes of Fabric's "the
maximum height for straight waistbands is 3 cm". The fixture keeps 4.0 cm — changing a published constant
to satisfy one tutorial would be exactly the silent revision this chapter's claim labelling exists to
prevent — and records the conflict instead. It, the interfacing's coverage of the visible half only, and
the midpoint fold line all stay `assumed` until the reviewer `G0-CONTRACT.14` names confirms them. A G2
golden may be frozen over the construction (it is decided and sourced); it may not be frozen over a
number that is still `assumed` without that review.

## Consequences

- The fixture's piece count is **5**, not 6; `docs/book/src/spec/reference-skirt.md` §6, §8 and §12 agree
  with each other and are compared mechanically rather than by reading.
- Tokens `waistband_outer` and `waistband_inner` no longer exist anywhere in the book. Tokens added:
  `waistband_fold_position`, `waistband_finished_length`, `wb_interfacing_width`, `wb_interfacing_length`,
  and the two closure rows — each declared by the table that uses it, per
  `decision_machine-tokens-declared-where-used`.
- `feature-matrix.md`'s `interfacing` row ("the fixture's waistband carries an interfacing piece", proven
  at G2) is still true and needed no edit — which is the check that this decision did not silently
  contradict a neighbouring chapter.
- The fixture now exercises three properties it did not: a piece whose every edge carries no allowance
  and is not a fold, a fold line that must never export as a cut line (the ⚠ cut/sew hazard), and a
  non-sewn attachment in the sewing graph's account.
- **The re-derivation is a tracked producer, not a scratch command:**
  `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` evaluates every §4 formula over the
  tables above it, checks the four closures, checks the piece account and the published count, and checks
  that §4's band width and §6's band piece list describe one construction (rule `B1`). Run over the
  chapter *before* this decision it reported **7 mismatches** and named D27; after it, **0**. Its probe
  suite is `run_fixture_probes.sh`. The arithmetic that found D33 and D27 had been typed into `python3 -c`
  strings in task leaves — re-runnable by nobody, which is the leg-3 breach this repository already
  committed once as D20.

## The question this decision refuses

The band's two short ends are folded right sides together and stitched across, which joins one piece to
itself. Whether a `SewingGraph` may carry a **self-span** — an edge range sewn to another range of the
same piece — is an ontology question that deciding here would settle by accident, so §8 records the ends
as an edge finish and the question is logged with an owner in `docs/tasks/PLANNING.md` (defect **D35**).

## Re-open condition

The reviewer `G0-CONTRACT.14` names may overturn any of it, and two findings would reopen it as a matter
of course: (1) the fixture's waist is moved off the natural waist, at which point the sources above make
a contoured band correct and the faced two-piece reading with it; (2) the domain reviewer rules that the
sample-room this product ships to builds straight bands as faced pairs, in which case the *construction*
changes and every number in this record's rule list is re-derived, not adjusted.

Related: [[decision_director-ruling-2026-09-30-four-findings]] ·
[[decision_fixture-oracles-derive-the-finished-dimension]] · [[decision_machine-tokens-declared-where-used]] ·
`docs/book/src/spec/reference-skirt.md` §4, §6, §8, §11 · `docs/tasks/PLANNING.md` (defects D27, D35).
