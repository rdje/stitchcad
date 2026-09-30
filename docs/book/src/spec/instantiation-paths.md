# Instantiation paths

> **Status:** normative specification, gate **G0** (roadmap §3.3, §15 item 6, ADR-0004). Implemented by
> `sc-grading` at gate **G3** (leaves `G3-GRADING.1`–`.14`). Terms are defined in the
> [glossary](glossary.md); the objects this chapter moves are specified in the [ontology](ontology.md);
> every number it quotes comes from the [reference skirt](reference-skirt.md).

A StitchCAD design can become a sized garment two ways, and **both are required**: by re-evaluating its
recipe for a body or a size (**regeneration**), or by moving the points of a base-size instance by a rule
table (**grade-rule instantiation**). The first is the parametric promise; the second is what a factory's
own system reconstructs from an interchange file. They are not two implementations of one meaning — they
are semantically different, they lose different information, and a product that pretends otherwise will be
contradicted by the first factory that grades a file it received.

## 1. The two paths at a glance

| | Path 1 — regeneration | Path 2 — grade-rule instantiation |
| --- | --- | --- |
| Inputs | the `Design` at a revision, the target's `MeasurementTable` and `Ease`, a `SizeSet` entry, resolved profile parameters | a base-size `Instance`, a grade-rule table of points and X/Y deltas, the size breaks |
| Produces geometry by | evaluating the formula graph, then replaying the ordered operations | moving grade points, then reconstructing the geometry between them |
| Output | an `Instance` for that size, with provenance naming the measurement set, the profile and the revision | an `Instance` per size, plus the `.rul` serialization for interchange |
| Authority | the recipe is authoritative; nothing is inherited from another size | the base instance and the rule table are authoritative; the recipe is not consulted |
| Oracle | re-evaluation determinism, the fixture's declared and constructed checks, POM verification against the size chart | re-import into an independent engine, reconstruction measurement, and the extreme-size checks of §7 |
| Information loss | none — the recipe carries the *why* | the recipe is not recoverable: the rules carry *how much*, never *why* (§5) |

## 2. Path 1 — measurement-driven regeneration

**Inputs.** A `Design` at a revision; a `MeasurementTable` whose entries carry landmarks, procedures and
uncertainty states; the `Ease` set mapping each body measurement to its garment POM; a `SizeSet` entry
naming the size; and a resolved parameter set from the Factory Profile.

**Process, in order.** Resolve the profile parameters against their constraints (roadmap §6.1 pass 1);
evaluate the formula graph in one pass over its acyclic order (pass 2); replay the ordered drafting
operations, producing entities with stable references; derive allowances, notches and grainlines from the
resulting edges; then verify geometrically (pass 3). An `unknown` parameter stops the process here, before
geometry exists, per the artifact policy matrix.

**Output.** An `Instance`: pieces with their boundaries, holes and internal lines; the sewing graph;
closures; label data; and **provenance** — the design revision, the measurement-set identity, the resolved
profile versions and the engine configuration. An instance without provenance cannot be released, because
nothing could say what it was made from.

**Guarantees.** Regeneration is deterministic (same inputs, same bytes after canonicalization); it loses no
information, because the recipe is the authority and every size is a fresh evaluation of it; and it
preserves identity, so a notch, a grade point or a seam span attached to an entity survives regeneration as
a reference to the same entity, never as a re-derived index (ontology §1.1).

**Oracle.** Three checks, in increasing cost: re-evaluating twice produces byte-identical canonical output;
the fixture's own closure checks hold at the target size (the reference skirt's `allocation_balance` and
`waist_closure`, §4.1 of that chapter); and every POM measured on the produced geometry equals the size
chart's value for that size within the physical-acceptance class. The third is the one that matters: the
first two can pass on a garment that does not fit the chart it was made for.

## 3. Path 2 — grade-rule instantiation

**Inputs.** A base-size `Instance` — produced by path 1 or imported; a **grade-rule table**; and the size
breaks of the target size set.

**Grade points are references, not indices.** A grade point is a `PointRef` (ontology §1): the identity of
the operation that constructed the point, plus its persistent tag. A rule table that addressed points by
position in an array would not survive an edit to the base, and could not be re-imported into an
independent engine — which is exactly what gate G3's exit criteria require of `.rul`.

**The rule table** carries, per grade point and per size break: an X and Y delta; whether the delta is
`incremental` (from the previous size) or `cumulative` (from the base); and the three attributes below.
Deltas are lengths in the internal unit, so a rule table is exact and never rounded twice.

| Attribute | StitchCAD semantics | Status of the `.rul` mapping |
| --- | --- | --- |
| `stack point` | the table's anchor: the point whose delta is zero by declaration, so graded pieces stack aligned instead of drifting. A table SHALL name at least one | cited from the roadmap; confirmed at G3 against a receiver |
| `fixed perimeter` | a contour whose length must not change across sizes — a zip opening, a button run. Where grading would change it, the grader SHALL report a conflict, not absorb it | cited from the roadmap; confirmed at G3 |
| `smoothing` | fairing a graded contour instead of moving points independently, bounded: it SHALL NOT move a point past the geometric-approximation class, nor move one another rule fixes | cited from the roadmap; confirmed at G3 |

**Reconstruction.** Moving points is not the whole operation; the geometry between them must be rebuilt, and
how it is rebuilt is normative:

- a straight edge stays straight, between its moved endpoints;
- an arc is re-fitted through its moved endpoints and its moved mid-point, and where no consistent arc
  exists the reconstruction reports it instead of emitting a wobble;
- a cubic Bézier keeps its parameterization and moves its control points by the interpolation of its
  endpoints' deltas — an approximation, and therefore bounded by the geometric-approximation class and
  reported in the instance's provenance;
- internal construction lines follow the edges they were constructed from, by parameter, so a dart leg
  stays a dart leg;
- notches keep their **parameter** on their edge, so a notch at the hip stays at the hip;
- **allowances are re-derived, never graded.** A `SeamAllowance` is a derived object attached to an edge
  (ontology §4.4), so grading moves the net geometry and the allowance is offset again at its declared
  width. The consequence is stated rather than hidden: a factory whose rules were built on *cut* contours
  will differ from this reconstruction at corners, by the corner treatment's own geometry, and that
  difference is what the equivalence report of §6 exists to show.

**Output.** An `Instance` per size, and the `.rul` serialization: base-size geometry plus the rule table.
The three interchange modes — base plus rules, rules embedded in DXF, and all-contours-graded — are named
and separately validated in the interchange chapter (`G0-CONTRACT.10`); this chapter fixes only the
semantics they carry.

**Oracle.** Re-import the `.rul` into an **independent** engine (not this one), reconstruct the sizes, and
measure the result against the instances path 1 produced; then run §7's extreme-size checks on the
reconstruction. A reconstruction that only this product can read is not evidence.

## 4. Worked example: grading the reference skirt

The fixture is graded from its base to the next size with declared breaks of `+4.0 cm` waist girth,
`+4.0 cm` hip girth and `+1.0 cm` skirt length. Every value below is re-derived, not recalled; the deltas
are what a rule table derived from these two instances would carry.

| Quantity | Base | Next size | Delta |
| --- | --- | --- | --- |
| `garment_waist` | 74.000 cm | 78.000 cm | +4.000 |
| `garment_hip` | 102.000 cm | 106.000 cm | +4.000 |
| `quarter_waist` | 18.500 cm | 19.500 cm | +1.000 |
| `quarter_hip` | 25.500 cm | 26.500 cm | +1.000 |
| `suppression` | 7.000 cm | 7.000 cm | 0.000 |
| `dart_intake` | 4.000 cm | 4.000 cm | 0.000 |
| waist side point, from CF | 22.500 cm | 23.500 cm | +1.000 |
| `front_dart_centre` | 11.250 cm | 11.750 cm | +0.500 |
| dart legs, from CF | 9.250 / 13.250 cm | 9.750 / 13.750 cm | +0.500 |
| `quarter_hem_width` | 28.500 cm | 29.500 cm | +1.000 |
| `garment_hem` | 114.000 cm | 118.000 cm | +4.000 |
| `hip_to_hem_drop` | 42.000 cm | 43.000 cm | +1.000 |
| `side_seam_length` | 42.107 cm | 43.105 cm | +0.998 |
| `waistband_pattern_length` | 80.000 cm | 84.000 cm | +4.000 |
| `waist_closure` (stitched quarter waist) | 18.500 cm | 19.500 cm | +1.000 |

**Both paths agree exactly here, and that is a property of the fixture, not a general result.** Seventeen
of the fixture's derived values are *affine* in the measurements (linear plus a constant), so a linear
delta per point reproduces them without residue; the eighteenth, `side_seam_length` =
`√(hip_to_hem_drop² + a_line_flare²)`, is nonlinear in the drop, yet it too reproduces exactly
(`43.104524` cm by both routes, difference `0.00e+00`) because grading moves the *points* the length is
computed from, and the length is a function of those points. The graded `waist_closure` still equals the
graded `quarter_waist` (`19.500 = 19.500`), so the constructed oracle of the fixture chapter holds at the
graded size and not only at the base.

The consequence for testing is concrete and slightly inconvenient: **the reference skirt cannot exercise
the equivalence tolerance**, because there is nothing to tolerate. It is the right first test for path 2 —
exact equality is assertable — and the tolerance itself is exercised at G3 by the bodice and set-in sleeve,
whose cap ease and armscye curve are not affine in the measurements, and by any rule table that came from
a factory rather than from two regenerations.

## 5. Where the paths diverge, and what is lost

Roadmap §3.3 states it plainly: independently drafted sizes are **not** exactly reconstructible from a base
plus rules. The loss has three distinct parts, and hiding any of them produces a product that argues with
its own users.

1. **The recipe is not recoverable from base plus rules.** A rule table says a point moves `+1.0 cm` in X;
   it cannot say that the point moves because the hip girth grew and the side seam takes a constant
   `ss_suppress`. So path 2 cannot serve a body whose proportions differ from the size chart — the MTM case
   — and a design imported as geometry plus rules can be graded but never re-drafted. This is why the
   canonical project stores the recipe and treats interchange files as projections (roadmap §15 item 3).
2. **`delta = 0` is ambiguous.** In the fixture, `ss_suppress`, `dart_len_front`, `dart_len_back`,
   `a_line_flare`, `zip_len` and `wb_width` do not grade: their deltas are zero. So is the zero of every
   point a rule table never mentioned. A round trip through `.rul` therefore cannot distinguish "this
   deliberately does not grade" from "this was omitted", and the canonical form must carry the distinction
   explicitly — a rule table written by StitchCAD marks declared zeros, and one read from a receiver marks
   them `unknown`, with the state that implies (ontology §5).
3. **Nonlinear steps do not commute with point motion.** Re-fitting a curve through moved points, truing a
   seam, squaring a hem corner to the grain: each is a function of the geometry, not of the points alone.
   Where a recipe contains one, path 1 and path 2 differ, and the difference grows with the break. The
   fixture's hem corner is squared to the grain rather than to the seam (§5 step 6 of that chapter), which
   is why its hem stays straight under grading; a bodice's armscye is not so obliging.

## 6. The equivalence contract

Because the paths differ, their agreement is a **measured, reported** property, never an assumption.

- **Compare per quantity, not per file.** The comparison is over named quantities — each POM, each seam
  length, each point position, each allowance width — so a divergence says what diverged and by how much,
  not merely that two files differ.
- **Name the tolerance class for every comparison** (units chapter §3). Point positions produced by the two
  paths from the same recipe are compared at the numerical class: they are integers, so equality is exact
  and anything else is a bug. A reconstruction produced by a *receiver's* system from our `.rul` is
  compared at the importer class, whose value the profile declares per receiver. A physical garment is
  compared at the physical-acceptance class, which is the factory's number and never ours.
- **Report, do not reconcile.** Neither path wins. The equivalence report names the quantity, both values,
  the difference, the class the difference is judged against, and the verdict. A difference beyond its class
  is a **finding** with an owner — the rule table, the recipe, or the receiver — and the report is part of
  the release package's evidence, not a log line.
- **Grade-rule provenance is part of the comparison.** A table derived from two regenerations and a table
  received from a factory are different evidence, and the report SHALL say which it used.

## 7. Extreme sizes are checked after reconstruction

Roadmap §3.3 is explicit that extreme sizes are checked *after target-system reconstruction*, not only at
the base. The reason is that the reconstruction is where a rule table meets reality: a base size that is
valid says nothing about a size five breaks away. The checks run on the reconstructed instance, in this
order, and each produces a typed diagnostic rather than a clamped value:

| Check | Refuses | Class |
| --- | --- | --- |
| declared domain | a coordinate or length outside the unit domain, which means a unit-conversion bug | numerical |
| offset validity | an allowance that self-intersects or cannot meet its error bound at the graded geometry | geometric |
| minimum curvature radius | a corner too tight for a knife, which a cutter discovers on the fabric | geometric |
| bounding box vs fabric width | a piece wider than the material the profile declares | physical |
| seam differential vs declared ease | a span whose differential exceeds the ease its sewing graph declares | geometric |
| notch spacing | two notches closer than the profile's minimum, so a cutter cannot distinguish them | physical |
| dart intake vs the practical maximum | a dart asked to absorb more than a single dart can | physical |
| the fixture's closure checks | a finished dimension that no longer equals what the recipe declares | exact |

The last row is the general form of the lesson defect D33 taught on the reference skirt: a check over
declared quantities cannot see a constructed point, so the extreme-size suite re-derives finished
dimensions from the reconstruction as well as comparing declared ones.

## 8. Serialization and interchange

- **Canonical:** base-size geometry plus the rule table, with declared zeros marked, grade points as
  `PointRef`s, and the recipe retained alongside. Two saves of one state are byte-identical (ontology §7).
- **`.rul`:** the AAMA-era table format, cited from the roadmap. Its incremental and cumulative modes and
  its three attributes are named there; the exact encoding of `stack point`, `fixed perimeter` and
  `smoothing` has **not** been read in this repository and is confirmed at G3 by re-importing into an
  independent engine, which is the gate's own exit criterion.
- **ASTM mode:** grade rules embedded in the DXF, and the all-contours-graded mode as a named option. The
  modes are separate, validated receiver by receiver, and no claim of a universal package is made
  (roadmap ADR-0004; the feature matrix rejects one with `env_universal_package`).

## 9. Verification status of the claims in this chapter

- **§4's numbers** — *exact arithmetic*, re-derived from the fixture's §2 and §3 constants by evaluating
  each formula in table order; the two paths' side-seam lengths agree to `0.00e+00`, and the graded
  `waist_closure` equals the graded `quarter_waist`.
- **The affinity claim (17 of 18 derived values are affine in the measurements)** — *exact arithmetic*:
  each of the fixture's formulas is linear plus a constant, except `side_seam_length`, whose reproduction
  under point grading is computed above rather than assumed.
- **`.rul` semantics — incremental vs cumulative, `stack point`, `fixed perimeter`, `smoothing`** —
  *cited from the roadmap* (§3.3, ADR-0004), which records its own review provenance. The format's text has
  not been read in this repository; §3 states the StitchCAD semantics normatively and marks the mapping as
  confirmed at G3 by an independent engine, not by agreement with a document nobody has opened here.
- **The three interchange modes** — *cited from the roadmap* (ADR-0004); specified by `G0-CONTRACT.10`.
- **The tolerance classes named in §6 and §7** — *project decision*, defined normatively in the units
  chapter; this chapter declares which applies where and invents none.
- **The claim that independently drafted sizes are not exactly reconstructible** — *cited from the roadmap*,
  which attributes it to GRAFIS-documented divergence. §5 states the mechanism (nonlinear steps do not
  commute with point motion) as the reason, and §6 makes the size of the divergence a measured report
  rather than a quoted fact.

## 10. What must be true in tests

- **Regeneration determinism:** the same design, measurement set and profile produce byte-identical
  canonical output, twice and on two platforms.
- **Path agreement where it is exact:** grading the reference skirt with a table derived from two
  regenerations reproduces path 1 exactly, quantity by quantity — the §4 table is the assertion, and the
  `0.00e+00` side-seam difference is part of it.
- **Divergence is reported, not absorbed:** a rule table from a receiver produces an equivalence report
  naming every quantity, both values, the difference and the class it is judged against.
- **Declared zeros survive a round trip:** a constant that does not grade is distinguishable from a point a
  table never mentioned, in both directions.
- **Grade points are references:** an edit to the base instance — a split, a mirror, a re-draft — leaves the
  rule table addressing the same entities, or produces visible repair tasks.
- **Allowances are re-derived, never graded:** exporting a graded size at two allowance widths differs only
  in the allowance geometry, and the net lines are the graded ones.
- **Extreme sizes are checked after reconstruction:** the §7 suite runs on the largest and smallest size of
  every graded set, and each check fails with its own diagnostic rather than clamping.
- **A fixed perimeter is honoured or refused:** grading that would change a contour declared fixed produces
  a conflict naming the contour and the size, never a silently changed length.
