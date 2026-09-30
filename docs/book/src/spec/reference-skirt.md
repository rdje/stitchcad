# The reference skirt

> **Status:** normative fixture specification, gate **G0** (roadmap §11, G0 fixture clause). It is the
> corpus garment for gate G2 — drafted through the CLI with deterministic replay, exported to one named
> DXF mode and to PDF, printed and measured with a ruler — and it is the garment the mutation tests and
> the agent gate are run against.

**Why one garment is specified to the millimetre.** Every conformance suite, golden file, offset test
and agent evaluation in this project needs a subject. If the subject is described in prose, two
implementations produce two garments and every comparison between them is meaningless. So this chapter
gives numbers, and every number has a **source**: a body measurement, a declared constant of the
reference drafting, or a value derived by a stated formula from those. A reader should be able to draft
the garment from this chapter without asking a question.

## 1. The garment

An **A-line skirt**, base size, with:

- one waist dart in each pattern quadrant (four in the assembled garment);
- a centre-back (CB) seam housing a **centred zipper**;
- a straight **waistband**, cut once and folded lengthwise, plus a separate interfacing piece,
  closing with a hook and bar;
- grain parallel to CB/CF on every body piece;
- **front cut on the fold**, back as a mirrored pair;
- seam allowances of 1 cm at the side seams and 3 cm at the hem, with the remaining edges declared
  below — so the fixture exercises variable allowances and the included/excluded policy.

**Recorded interpretation.** The roadmap describes the fixture as "one waist dart/side". This chapter
realises it as **one dart per pattern quadrant**: the front piece carries two darts, symmetric about CF
(the piece is cut on the fold, so a single off-centre dart would be asymmetric), and each back piece
carries one. The reason is arithmetic, not preference: the suppression each quadrant must absorb is
7.0 cm (§3), of which the side seam takes 3.0 cm, leaving 4.0 cm for a dart. Two darts per body side
would need 8.0 cm of intake in one dart, well past the practical single-dart maximum.

## 2. Measurements

Body measurements at the base size, with landmarks and procedures as the ontology requires. Values are
**declared fixture constants**, chosen as a mid-range woven-womenswear base; they are not copied from
any size standard (see §11).

| Token | Name | Kind | Value | Landmark / procedure |
| --- | --- | --- | --- | --- |
| `waist_girth` | waist circumference | body | 74.0 cm | natural waist, narrowest circumference |
| `hip_girth` | hip circumference | body | 98.0 cm | fullest seat, 20 cm below waist |
| `waist_to_hip` | waist-to-hip depth | body | 20.0 cm | waist to hip level, centre back |
| `waist_to_hem` | skirt length | garment | 62.0 cm | waist to hem, centre back |

Ease, per point of measure (§2.2 of the ontology):

| Ease token | POM | Body | Ease | Fit intent | Garment |
| --- | --- | --- | --- | --- | --- |
| `ease_waist` | waist | 74.0 cm | 0.0 cm | `close` | 74.0 cm |
| `ease_hip` | hip | 98.0 cm | 4.0 cm | `semi` | 102.0 cm |

The waist carries no ease because the closure is a zipper plus a hook-and-bar waistband; the hip carries
4 cm, the conventional minimum for a woven A-line that must clear the seat when sitting.

## 3. Declared drafting constants

These are the constants of the **StitchCAD reference drafting**, named here so that the fixture is
self-consistent and reviewable. They are *not* a reproduction of any commercial drafting system; the
choice of which named system ships as reference blocks is ADR-0003's.

| Token | Value | Meaning |
| --- | --- | --- |
| `quarter` | 1/4 | the fixture is drafted per quadrant, symmetric about CF and CB |
| `ss_suppress` | 3.0 cm | waist suppression taken by the side seam, per quadrant |
| `dart_intake` | 4.0 cm | waist dart intake, per quadrant (derived, see below) |
| `dart_len_front` | 12.0 cm | front dart length below the waist line |
| `dart_len_back` | 14.0 cm | back dart length below the waist line |
| `a_line_flare` | 3.0 cm | side-seam flare outward, per quadrant, from hip to hem |
| `zip_len` | 18.0 cm | centred zipper length at CB |
| `wb_width` | 4.0 cm | finished waistband width |
| `wb_extension` | 3.0 cm | waistband overlap for the hook and bar |

## 4. Derived values (each with its formula)

Every derived value carries the **token** the recipe refers to it by, so a formula in this chapter or in
code never names something this table has not declared. Formulas are written over tokens only: no prose
word appears inside one, which is what makes them checkable by machine.

| Token | Value | Formula | Result |
| --- | --- | --- | --- |
| `garment_waist` | garment waist | `waist_girth + ease_waist` | 74.0 cm |
| `garment_hip` | garment hip | `hip_girth + ease_hip` | 102.0 cm |
| `quarter_waist` | quarter waist | `garment_waist / 4` | 18.5 cm |
| `quarter_hip` | quarter hip | `garment_hip / 4` | 25.5 cm |
| `suppression` | waist suppression per quadrant | `quarter_hip − quarter_waist` | 7.0 cm |
| `dart_intake` | waist dart intake per quadrant | `suppression − ss_suppress` | 4.0 cm |
| `allocation_balance` | the allocation balance (§4.1) | `4 × (ss_suppress + dart_intake)` = `garment_hip − garment_waist` | 28.0 cm = 28.0 cm |
| `waist_closure` | the stitched quarter waist (§4.1) | `(quarter_hip − ss_suppress) − dart_intake` = `quarter_waist` | 18.5 cm = 18.5 cm |
| `side_seam_total` | side-seam intake, 2 seams × 2 quadrants | `4 × ss_suppress` | 12.0 cm |
| `dart_total` | dart intake, 4 darts | `4 × dart_intake` | 16.0 cm |
| `front_dart_centre` | front dart centre, from CF | `(quarter_hip − ss_suppress) / 2` | 11.25 cm |
| `back_dart_centre` | back dart centre, from CB | `(quarter_hip − ss_suppress) / 2` | 11.25 cm |
| `quarter_hem_width` | quarter hem width | `quarter_hip + a_line_flare` | 28.5 cm |
| `garment_hem` | garment hem circumference | `4 × quarter_hem_width` | 114.0 cm |
| `hip_to_hem_drop` | hip-to-hem drop | `waist_to_hem − waist_to_hip` | 42.0 cm |
| `side_seam_length` | side-seam length, hip to hem | `√(hip_to_hem_drop² + a_line_flare²)` | 42.107 cm |
| `waistband_pattern_length` | waistband pattern length | `garment_waist + 2 × sa_cb + wb_extension` | 80.0 cm |
| `waistband_cut_width` | waistband cut width, one band folded | `2 × wb_width + sa_waist + sa_wb_bottom` | 10.0 cm |
| `waistband_fold_position` | fold line, from either long edge | `waistband_cut_width / 2` | 5.0 cm |
| `waistband_finished_length` | band length once sewn, with extension | `waistband_pattern_length − 2 × sa_cb` | 77.0 cm |
| `wb_interfacing_width` | interfacing width, no allowance on it | `wb_width` | 4.0 cm |
| `wb_interfacing_length` | interfacing length, no allowance on it | `waistband_finished_length` | 77.0 cm |
| `waistband_length_closure` | the band's net length (§4.1) | `waistband_pattern_length − 2 × sa_cb − wb_extension` = `garment_waist` | 74.0 cm = 74.0 cm |
| `waistband_width_closure` | the band's net height (§4.1) | `waistband_cut_width − sa_waist − sa_wb_bottom` = `2 × wb_width` | 8.0 cm = 8.0 cm |

### 4.1 The four closure checks

The fixture carries four oracles in two pairs, because they answer different questions and one of them
is not enough.

- **`allocation_balance`** — is the waist suppression fully distributed?
  `4 × (ss_suppress + dart_intake) = garment_hip − garment_waist`, here `28.0 cm = 28.0 cm`.
- **`waist_closure`** — does the distribution land where the geometry is? The panel's waist edge is drawn
  from CF to a side point `ss_suppress` *inside the hip point*, and the dart then removes its intake, so
  `(quarter_hip − ss_suppress) − dart_intake` must equal the `quarter_waist` the panel owes: here
  `(25.5 − 3.0) − 4.0 = 18.5 cm`, and `4 × 18.5 = 74.0 cm` is the declared garment waist.

The second check exists because this chapter is the evidence that the first is insufficient: an earlier
revision placed the waist side point at `quarter_waist − ss_suppress` = 15.5 cm, which satisfies the
allocation balance exactly and drafts a skirt whose finished waist is 46.0 cm instead of 74.0 cm (defect
**D33**, recorded in §11). A balance over *declared quantities* cannot see a *constructed point*; only a
check that re-derives the finished dimension from the geometry can. If a future edit changes a measurement
and any check stops holding, the recipe is wrong, not the check.

The band carries the same pair, because its two finished dimensions are constructed too — a band cut to
the wrong length or the wrong height is the same defect class in a smaller piece (defect **D27**, §11):

- **`waistband_length_closure`** — is the band the length of the waist it is sewn to? Its cut length less
  its two CB allowances and its extension must equal the garment waist the body pieces produce:
  `80.0 − 3.0 − 3.0 = 74.0 cm`.
- **`waistband_width_closure`** — does the cut width produce the band that was asked for? Both allowances
  come off and what is left is two layers of the finished height: `10.0 − 1.0 − 1.0 = 8.0 = 2 × 4.0 cm`.

## 5. Drafting recipe

Ordered operations, each producing entities with stable references (ontology §1). All lengths in the
internal unit; the centimetre values above are their display form.

1. **Construct the frame.** Vertical axis = CF (front) / CB (back). Horizontal lines at waist (y = 0),
   hip (y = −`waist_to_hip`), hem (y = −`waist_to_hem`).
2. **Front quadrant.** From CF at the hip line, mark `quarter_hip` = 25.5 cm toward the side. At the
   waist line the side point sits `ss_suppress` = 3.0 cm inside the hip side point — that is, at
   `quarter_hip − ss_suppress` = 22.5 cm from CF — because the side seam is where those 3.0 cm are taken
   out. The waist edge therefore runs 22.5 cm from CF before the dart closes it, and the dart's 4.0 cm
   leaves the `quarter_waist` = 18.5 cm this panel owes (§4.1).
3. **Front dart.** Centre at `front_dart_centre` from CF on the waist line; intake `dart_intake`, split
   equally about the centre; apex `dart_len_front` below the waist line, on the centre line. The two
   legs are straight lines, at 9.25 cm and 13.25 cm from CF; the waist line is trued so the stitched
   length matches `quarter_waist` = 18.5 cm (§4.1).
4. **Back quadrant.** Mirrored construction from CB, with `dart_len_back`. The back piece is *not* cut
   on the fold: CB is a seam edge.
5. **A-line flare.** From the hip side point, the side seam runs straight to a point `a_line_flare`
   outside the hip width at the hem line. The side seam is one straight segment hip→hem; above the hip
   it is the shaped waist curve of step 2.
6. **Hem line.** Straight, perpendicular to CF/CB. Because the side seam flares, the hem corner is
   squared to the grain, not to the seam.
7. **Notches** (§7), **grainlines** (§7), then **allowances** (§7) as derived offsets.
8. **Waistband.** ONE rectangle of `waistband_pattern_length` × `waistband_cut_width`, cut once. Its fold
   line runs the whole length at `waistband_fold_position` from each long edge and is an *internal
   construction line* (ontology §4.1), never a cut edge: all four sides of the rectangle are cut, and its
   label does not say "place on fold". The lower long edge carries `sa_waist` and is sewn to the skirt's
   waist; the free long edge carries `sa_wb_bottom` and is turned to the inside, where stitching from the
   right side in the seam ditch catches it; each short end carries `sa_cb`, which is why the band is
   `2 × sa_cb` longer than the garment waist. Mark the CB seam positions, the hook-and-bar extension
   beyond the left CB, and a notch at each CB and at the quarter points. The fold line sits at the exact
   midpoint by declared construction; the sewing-room habit of pressing it a millimetre toward the free
   edge so the fell stitch cannot miss it is a handling adjustment, not pattern geometry, and is not
   modelled here.
9. **Interfacing.** A rectangle of `wb_interfacing_length` × `wb_interfacing_width` — the band's finished
   dimensions, with no allowance on any edge — fused to the wrong side of the half between the fold line
   and the lower edge, so the half that shows is the stiffened one and the fold stays crisp.
10. **Mirror.** The front piece is used as a half, cut on the CF fold. The back left piece is the mirror
    of the back right piece about CB.

The recipe is replayable: given the same measurement table and constants, evaluation produces the same
geometry on any platform, byte-identically after canonicalization.

## 6. Pieces, multiplicity and labels

| Piece | Cut | Fold | Pair | Material | Layer |
| --- | --- | --- | --- | --- | --- |
| `skirt_front` | 1 | CF edge | — | shell | 0 |
| `skirt_back_right` | 1 | — | with `skirt_back_left` | shell | 0 |
| `skirt_back_left` | 1 | — | mirror of right | shell | 0 |
| `waistband` | 1 | — | — | shell | 1 |
| `waistband_interfacing` | 1 | — | — | interfacing | 2 |

Total cut pieces: 5. Printed label data per piece is complete without consulting anything else: piece
name, size, cut quantity, L/R where paired, "place on fold" for the front, fabric and colorway.

The band is ONE piece, cut once and folded lengthwise; §11 records that decision, the reading it
rejected, and the sources it was read from. Two consequences a reader of the piece list must not miss:
the band's fold column is empty because its fold is an internal construction line and not a fabric fold
edge (§5 step 8), and the interfacing is a piece of its own because it is cut — carrying no allowance on
any edge, which is the allowance-free case that is *not* a fold.

## 7. Seam allowances, notches, grainlines

**Allowances** — a `SeamAllowance` is attached per edge, with its corner treatment and its inclusion
policy (§4.4 of the ontology):

The token column is what §4's formulas refer to these widths by. One token may serve several edges:
`sa_cb` covers the band's two short ends as well as the CB seam, and `sa_waist` covers the band's lower
long edge as well as the body pieces' waist edge, because each pair is sewn together at that width.

| Token | Edge | Width | Corner treatment | Policy |
| --- | --- | --- | --- | --- |
| `sa_side` | side seams | 1.0 cm | miter | profile-resolved |
| `sa_cb` | CB seam, and the band's short ends | 1.5 cm | trim | profile-resolved |
| `sa_waist` | waist edge, and the band's lower edge | 1.0 cm | trim | profile-resolved |
| `sa_hem` | hem | 3.0 cm | envelope | profile-resolved |
| `sa_wb_bottom` | band's free edge, turned under | 1.0 cm | trim | profile-resolved |
| — | CF (front) | none — fold | — | not an allowance |
| — | `waistband_interfacing`, every edge | none — fused | — | not an allowance |

"Included in contour" versus "generated downstream" is a **Factory Profile** parameter, and this
fixture is the one that exercises both: the same design exports with allowances included for one
receiver and excluded for another, and the two artifacts differ only in what the profile asked for.

**Notches** — semantic marks; their physical form is a profile parameter:

- one **single notch** on each side seam of the front, at hip level; the matching back side seam
  carries the same single notch at the same parameter, so walking the seam aligns them;
- one **single notch** on each CB seam at the zipper stop (`zip_len` below the waist), which is where
  the sewing stops;
- one notch on the waistband at each CB and at each quarter point.

**Grainlines** — directed, parallel to CB/CF on every body piece, placed at the midpoint of the piece's
hip-line width, running from the waist line to the hem line. The waistband grain is parallel to its
length. No bias in this fixture; the bias case is exercised by the bodice at G3.

## 8. Sewing graph

| Span | Side A | Side B | Ease | Stop landmark |
| --- | --- | --- | --- | --- |
| `ss_right` | `skirt_front` right side seam | `skirt_back_right` side seam | 0 (declared) | hip notch |
| `ss_left` | `skirt_front` left side seam | `skirt_back_left` side seam | 0 (declared) | hip notch |
| `cb` | `skirt_back_left` CB | `skirt_back_right` CB | 0 (declared) | zipper-stop notch |
| `waist` | skirt waist edge (all pieces) | `waistband` lower long edge | 0 (declared) | quarter notches |

Every piece §6 lists is accounted for below, by the spans it participates in or by a declared non-sewn
attachment from a closed list, which today holds exactly one method: `fused`. A piece with neither is
exactly how defect D27 hid — an inner band nobody sewed — so the account is a table, and the derivation
instrument in §11 refuses a piece that is missing from it.

| Piece | How it is accounted for |
| --- | --- |
| `skirt_front` | spans `ss_right`, `ss_left`, `waist` |
| `skirt_back_right` | spans `ss_right`, `cb`, `waist` |
| `skirt_back_left` | spans `ss_left`, `cb`, `waist` |
| `waistband` | span `waist` |
| `waistband_interfacing` | no span — fused to the band's inner side before the band is folded |

All spans declare **zero ease**: this garment has no eased seams, which is precisely what makes it the
right *first* fixture — the seam-length equality property holds exactly here, and the intentional-ease
case (a sleeve cap into an armscye) is exercised at G3 where it belongs. `walk` on any span of this
fixture must therefore report a differential inside the numerical tolerance class, and any larger
differential is a bug rather than ease.

Assembly order: fuse the interfacing → darts → side seams → CB seam (zipper) → waistband to waist →
hem. Layer index orders the waistband above the body and the interfacing above the waistband for 3D
assembly. The band's two short ends are folded right sides together and stitched across at their `sa_cb`
line, then turned; that closure joins one piece to itself, so it is an edge finish and not a span — and
whether a `SewingGraph` may carry such a self-span at all is a question this fixture deliberately does
not answer (§11).

## 9. Closure

A **centred zipper** at CB, `zip_len` = 18.0 cm, with the CB seam allowance of 1.5 cm; and a **hook and
bar** at the waistband, positioned on the `wb_extension`. The closure object records both, with their
placement references to the CB seam and the waistband, so a package-completeness check can prove the
notions list matches the geometry.

## 10. What this fixture exercises

| Fixture property | Ontology clause | Gate that depends on it |
| --- | --- | --- |
| darts with conserved intake | §4.3 | G2 drafting, G3 closures |
| grainlines parallel to CB | §4.6 | G2 export, G3 bias |
| variable allowances (1 / 1.5 / 3 cm) | §4.4 | G2 offset engine + pathology corpus |
| notches with matched parameters | §4.5 | G2 export, G6 import-diff |
| a fold edge with no allowance | §4.1 | G2 export, G5 piece manager |
| mirrored pair with L/R labels | §4.1 | G2 labels, G5 piece manager |
| included vs excluded allowance | §4.4 | G4 profile policy, G6 receivers |
| zero-ease sewing graph | §4.2 | G2 seam-length property, G3 contrast |
| an interfacing piece, fused and never sewn | §4.7 | G2 export, G4 profile policy |
| a fold line that must never export as a cut line | §4.1 | G2 export, G6 import-diff |
| closure + notions | §4.7 | G5 tech pack, G7 completeness |
| recipe replay determinism | §3.1 | G2 CLI replay, agent gate |

## 11. Verification status of the numbers

- **Body measurements and ease values** — *declared fixture constants*: a plausible mid-range woven
  base chosen for this repository, not copied from a size standard. The measurement-standards chapter
  owns reconciling them with the ISO 8559 / ASTM D5585 landmarks.
- **Every §4 derived value** — *exact arithmetic* from §2, §3 and §7, re-derivable by the formula shown
  beside it, and re-derived by a tracked instrument rather than by reading:
  `docs/tasks/artifacts/reference_fixture/run_fixture_derivation.sh` evaluates every row's formula over
  the tables above it, compares the result with the published number, and refuses a piece §8 does not
  account for. The four closure checks in §4.1 are the oracles; the instrument is the proof that the
  chapter still agrees with itself.
- **`ss_suppress`, `dart_intake`, `dart_len_front`, `dart_len_back`, `a_line_flare`, and the convention
  that places a dart at the midpoint of its waist edge** — *declared drafting constants requiring domain
  review*. A sewing expert should confirm them before the fixture
  is frozen as a golden at G2; until then they are `assumed`, not `known`, and this chapter says so.
- **Zipper length, waistband width and extension** — *declared constants* at common industry values,
  with the same review status. `wb_width` is the one the sources read for D27 disagree about: a usual
  band height of 2–5 cm in one, a maximum of 3 cm for a *straight* band in the other. The fixture's
  4.0 cm is inside the first range and above the second, so it stays `assumed` and the reviewer `.14`
  names settles it; both URLs are in the decision record named below.
- **The waistband is one straight band, folded lengthwise, and every number above follows from it
  (defect D27, resolved).** The chapter carried two garments at once for nine commits after the one
  that recorded the contradiction: §4's
  `waistband_cut_width` was the width of ONE band folded lengthwise, while §6 listed a faced pair —
  an outer band, an inner band and a separate interfacing piece — whose fabric pieces would each be cut
  at 6.0 cm — and §8 sewed only the outer one, so the inner band had no span at all. Both readings are
  real skirt constructions; a fixture may only be one. It keeps the **folded band**, because its waist
  sits at the natural waist, where the drafting sources describe a straight band as one rectangle
  interfaced and folded lengthwise, and the faced two-piece cut is what those same sources prescribe for
  a *contoured* band — which this skirt does not have. Consequences, all re-derived: the piece count is
  5, the faced pair is gone, the interfacing survives as a piece of its own
  cut at the band's finished dimensions with no allowance, §4 gained four rows and §4.1 gained the
  band's two closure checks, and §8 now accounts for every piece. Still `assumed` pending the reviewer
  `.14` names: the band height (above), that the interfacing covers only the visible half, and that the
  fold line is the exact midpoint. The decision, its sources with their URLs and the date they were read,
  and the condition that would re-open it are in
  `docs/decisions/decision_reference-fixture-waistband-straight-folded.md`.
- **One question this fixture deliberately does not answer.** The band's short ends are folded right
  sides together and stitched across, which joins one piece to itself. Whether a `SewingGraph` may carry
  such a self-span — an edge range sewn to another range of the same piece — is an ontology question,
  not a fixture question, and deciding it here would settle it by accident. §8 records the ends as an
  edge finish instead; the question is owned in `docs/tasks/PLANNING.md`.

- **The waist side point was wrong, and the chapter's own oracle could not see it (defect D33,
  corrected).** §5 step 2 placed it at `quarter_waist − ss_suppress` = 15.5 cm from CF; §3's
  `ss_suppress` = 3.0 cm and §4's allocation balance require `quarter_hip − ss_suppress` = 22.5 cm,
  because the side seam takes 3.0 cm off the **hip** width, not off the waist width. Drafted as written,
  the stitched quarter waist is `15.5 − 4.0` = 11.5 cm and the finished garment waist 46.0 cm instead of
  the declared 74.0 cm — a 28.0 cm error in the fixture every G2 golden is built on — while the
  allocation balance still closed exactly, which is why §4.1 now carries `waist_closure` as a second
  oracle. The correction changes three published numbers (`front_dart_centre` and `back_dart_centre`
  7.75 → 11.25 cm, and §5 step 3's truing target `quarter_waist − ss_suppress` → `quarter_waist`) and
  adds one row. The dart centre is the midpoint of the corrected waist edge, which is what the original
  formula meant; *where* a dart sits is a drafting convention, so it joins the `assumed` constants above
  pending the review `G0-CONTRACT.14` names. The sealed changelog entry for `STITCHCAD-G0-0013` lists
  "dart intake and centre" among the derived values and is immutable, so this bullet is the superseding
  record.

The distinction matters: an `assumed` constant is exportable with its assumption recorded, but it is not
evidence, and a golden file frozen over an unreviewed assumption freezes a guess — and a golden frozen
over an arithmetic error freezes the error with the same confidence. `G0-CONTRACT.14` (governance) owns
naming the expert who reviews them.

## 12. Test obligations

- Drafting the fixture through the CLI twice produces byte-identical canonical output (G2 replay).
- All four §4.1 closure checks hold after any measurement change, or the recipe reports the imbalance. A
  construction that satisfies `allocation_balance` but not `waist_closure` is the D33 defect class, and
  is the reason both are obligations rather than one; the band's pair exists for the same reason.
- Dart intake is conserved: closing each dart removes exactly 4.0 cm from its waist edge, within the
  numerical tolerance class.
- Every `walk` on this fixture reports a differential within the numerical class, because all spans
  declare zero ease.
- Allowances produce no self-intersection anywhere on the fixture, at every declared width (the offset
  engine's budget, §6 of the units chapter).
- Exporting with allowances included and excluded differs only by the allowance geometry; the net
  (sew) lines are identical.
- The front piece has no allowance on its fold edge, and the exported label says "place on fold".
- The band's fold line exports as an internal line and never as a cut line, and the band's label does
  **not** say "place on fold" — the two are different facts about different pieces, and swapping them
  is the cut/sew-line hazard the glossary marks ⚠.
- The interfacing piece has no allowance on any edge, is not cut on a fold, and appears in no span: its
  attachment is the declared `fused` account in §8, so package completeness passes without inventing a
  seam for it.
- Package completeness: 5 pieces, 2 notched side-seam pairs, 1 zipper, 1 hook and bar, and a notions
  list that matches.
