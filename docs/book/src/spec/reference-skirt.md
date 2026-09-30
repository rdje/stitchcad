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
- a straight **waistband**, cut twice plus interfacing, closing with a hook and bar;
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
| `allocation_balance` | the balance check (§4.1) | `4 × (ss_suppress + dart_intake)` = `garment_hip − garment_waist` | 28.0 cm = 28.0 cm |
| `side_seam_total` | side-seam intake, 2 seams × 2 quadrants | `4 × ss_suppress` | 12.0 cm |
| `dart_total` | dart intake, 4 darts | `4 × dart_intake` | 16.0 cm |
| `front_dart_centre` | front dart centre, from CF | `(quarter_waist − ss_suppress) / 2` | 7.75 cm |
| `back_dart_centre` | back dart centre, from CB | `(quarter_waist − ss_suppress) / 2` | 7.75 cm |
| `quarter_hem_width` | quarter hem width | `quarter_hip + a_line_flare` | 28.5 cm |
| `garment_hem` | garment hem circumference | `4 × quarter_hem_width` | 114.0 cm |
| `hip_to_hem_drop` | hip-to-hem drop | `waist_to_hem − waist_to_hip` | 42.0 cm |
| `side_seam_length` | side-seam length, hip to hem | `√(hip_to_hem_drop² + a_line_flare²)` | 42.107 cm |
| `waistband_pattern_length` | waistband pattern length | `garment_waist + 2 × sa_cb + wb_extension` | 80.0 cm |
| `waistband_cut_width` | waistband cut width — **disputed, see §11** | `2 × wb_width + sa_waist + sa_wb_bottom` | 10.0 cm |

### 4.1 The balance check

`allocation_balance` is the fixture's own invariant: waist suppression must be fully allocated. If a
future edit changes a measurement and the check no longer balances, the recipe is wrong, not the check.

## 5. Drafting recipe

Ordered operations, each producing entities with stable references (ontology §1). All lengths in the
internal unit; the centimetre values above are their display form.

1. **Construct the frame.** Vertical axis = CF (front) / CB (back). Horizontal lines at waist (y = 0),
   hip (y = −`waist_to_hip`), hem (y = −`waist_to_hem`).
2. **Front quadrant.** From CF at the hip line, mark `quarter_hip` toward the side. At the waist line,
   mark `quarter_waist − ss_suppress` from CF; the difference is taken out at the side seam, so the
   waist side point sits `ss_suppress` inside the hip side point.
3. **Front dart.** Centre at `front_dart_centre` from CF on the waist line; intake `dart_intake`, split
   equally about the centre; apex `dart_len_front` below the waist line, on the centre line. The two
   legs are straight lines; the waist line is trued so the stitched length matches
   `quarter_waist − ss_suppress`.
4. **Back quadrant.** Mirrored construction from CB, with `dart_len_back`. The back piece is *not* cut
   on the fold: CB is a seam edge.
5. **A-line flare.** From the hip side point, the side seam runs straight to a point `a_line_flare`
   outside the hip width at the hem line. The side seam is one straight segment hip→hem; above the hip
   it is the shaped waist curve of step 2.
6. **Hem line.** Straight, perpendicular to CF/CB. Because the side seam flares, the hem corner is
   squared to the grain, not to the seam.
7. **Notches** (§7), **grainlines** (§7), then **allowances** (§7) as derived offsets.
8. **Waistband.** A rectangle of `waistband_pattern_length × waistband_cut_width`, with the CB seam
   positions marked, the hook-and-bar extension marked beyond the left CB, and a notch at each CB and
   at the quarter points so the band can be matched to the skirt.
9. **Mirror.** The front piece is used as a half, cut on the CF fold. The back left piece is the mirror
   of the back right piece about CB.

The recipe is replayable: given the same measurement table and constants, evaluation produces the same
geometry on any platform, byte-identically after canonicalization.

## 6. Pieces, multiplicity and labels

| Piece | Cut | Fold | Pair | Material | Layer |
| --- | --- | --- | --- | --- | --- |
| `skirt_front` | 1 | CF edge | — | shell | 0 |
| `skirt_back_right` | 1 | — | with `skirt_back_left` | shell | 0 |
| `skirt_back_left` | 1 | — | mirror of right | shell | 0 |
| `waistband_outer` | 1 | — | — | shell + interfacing | 1 |
| `waistband_inner` | 1 | — | — | shell | 1 |
| `waistband_interfacing` | 1 | — | — | interfacing | 2 |

Total cut pieces: 6. Printed label data per piece is complete without consulting anything else: piece
name, size, cut quantity, L/R where paired, "place on fold" for the front, fabric and colorway.

The three waistband rows are one of the two readings of a band this chapter has not settled: §4's cut
width describes a single band folded lengthwise, this list describes a faced two-piece band. Defect
**D27** and §11 own the choice; until it is made, this list and the count of 6 are provisional.

## 7. Seam allowances, notches, grainlines

**Allowances** — a `SeamAllowance` is attached per edge, with its corner treatment and its inclusion
policy (§4.4 of the ontology):

The token column is what §4's formulas refer to these widths by. `sa_wb_bottom` is declared here because
`waistband_cut_width` uses it; its width is the one §4's published arithmetic already requires, and it is
as disputed as the band it belongs to (§11).

| Token | Edge | Width | Corner treatment | Policy |
| --- | --- | --- | --- | --- |
| `sa_side` | side seams | 1.0 cm | miter | profile-resolved |
| `sa_cb` | CB seam | 1.5 cm | trim | profile-resolved |
| `sa_waist` | waist edge | 1.0 cm | trim | profile-resolved |
| `sa_hem` | hem | 3.0 cm | envelope | profile-resolved |
| `sa_wb_bottom` | waistband lower edge | 1.0 cm | trim | profile-resolved |
| — | CF (front) | none — fold | — | not an allowance |

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
| `waist` | skirt waist edge (all pieces) | `waistband_outer` lower edge | 0 (declared) | quarter notches |

All spans declare **zero ease**: this garment has no eased seams, which is precisely what makes it the
right *first* fixture — the seam-length equality property holds exactly here, and the intentional-ease
case (a sleeve cap into an armscye) is exercised at G3 where it belongs. `walk` on any span of this
fixture must therefore report a differential inside the numerical tolerance class, and any larger
differential is a bug rather than ease.

Assembly order: darts → side seams → CB seam (zipper) → waistband to waist → hem. Layer index orders
the waistband above the body and the interfacing above the waistband for 3D assembly.

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
| closure + notions | §4.7 | G5 tech pack, G7 completeness |
| recipe replay determinism | §3.1 | G2 CLI replay, agent gate |

## 11. Verification status of the numbers

- **Body measurements and ease values** — *declared fixture constants*: a plausible mid-range woven
  base chosen for this repository, not copied from a size standard. The measurement-standards chapter
  owns reconciling them with the ISO 8559 / ASTM D5585 landmarks.
- **Every §4 derived value** — *exact arithmetic* from §2 and §3, re-derivable by the formula shown
  beside it; the balance check in §4.1 is the internal oracle. The exception is `waistband_cut_width`,
  whose construction is disputed (below).
- **`ss_suppress`, `dart_intake`, `dart_len_front`, `dart_len_back`, `a_line_flare`** — *declared
  drafting constants requiring domain review*. A sewing expert should confirm them before the fixture
  is frozen as a golden at G2; until then they are `assumed`, not `known`, and this chapter says so.
- **Zipper length, waistband width and extension** — *declared constants* at common industry values,
  with the same review status.
- **The waistband is internally inconsistent, and no waistband number here is a fact yet (defect
  D27).** §4's `waistband_cut_width = 2 × wb_width + sa_waist + sa_wb_bottom = 10.0 cm` is the cut
  width of ONE band folded lengthwise, finished 4.0 cm with 1.0 cm turned at each raw edge. §6's piece
  list instead carries `waistband_outer`, `waistband_inner` **and** `waistband_interfacing` — a faced
  two-piece band, whose pieces would each be cut at `wb_width + sa_waist + sa_wb_bottom = 6.0 cm`, and
  §6 also lists the outer band's material as "shell + interfacing" while a separate interfacing piece
  exists. §8 compounds it: the `waist` span sews only the outer band's lower edge, so the inner band
  has no span at all and the piece count of 6 in §12 is the faced reading's. Both readings are real
  skirt constructions; they are **different garments**, and a fixture may only be one. Choosing is a
  domain decision, so it belongs to the review `G0-CONTRACT.14` names the expert for, and until it is
  made the affected numbers — `waistband_cut_width`, the §6 piece list and count, and the §8 `waist`
  span — are provisional. A G2 golden must not be frozen over them.

The distinction matters: an `assumed` constant is exportable with its assumption recorded, but it is not
evidence, and a golden file frozen over an unreviewed assumption freezes a guess. `G0-CONTRACT.14`
(governance) owns naming the expert who reviews them.

## 12. Test obligations

- Drafting the fixture through the CLI twice produces byte-identical canonical output (G2 replay).
- The §4.1 balance check holds after any measurement change, or the recipe reports the imbalance.
- Dart intake is conserved: closing each dart removes exactly 4.0 cm from its waist edge, within the
  numerical tolerance class.
- Every `walk` on this fixture reports a differential within the numerical class, because all spans
  declare zero ease.
- Allowances produce no self-intersection anywhere on the fixture, at every declared width (the offset
  engine's budget, §6 of the units chapter).
- Exporting with allowances included and excluded differs only by the allowance geometry; the net
  (sew) lines are identical.
- The front piece has no allowance on its fold edge, and the exported label says "place on fold".
- Package completeness: 6 pieces, 2 notched side-seam pairs, 1 zipper, 1 hook and bar, and a notions
  list that matches.
