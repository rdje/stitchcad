# Units and tolerances

> **Status:** normative specification, gate **G0**. Implemented by `sc-units` at gate G1
> (leaf `G1-SLICE.2`) and consumed by every crate that touches a number. Where this chapter states a
> rule with SHALL or MUST, the implementation has no further decision to make.

Everything StitchCAD measures is stored in **one internal unit**, compared against **one of five named
tolerance classes**, and rounded by **one declared rule**. There is no global epsilon, and no
comparison in the codebase may be written without naming its class.

## 1. The internal unit: fixed-point micrometres

**Lengths SHALL be stored as signed 64-bit integers counting micrometres (µm, 10⁻⁶ m).** One metre is
`1 000 000` units; one millimetre is `1 000`; one centimetre is `10 000`; one inch is `25 400` exactly.

Why fixed-point rather than floating point:

- **Determinism.** The same operations on the same inputs produce the same integers on every platform
  and every compiler. Golden-file comparison and cross-platform conformance (gate G2) depend on this;
  floating-point association and library differences do not survive a byte-comparison.
- **Exactness where it matters.** A pattern piece boundary is a polygon of rational coordinates. In
  integer micrometres, orientation and segment-intersection predicates are *exactly* computable
  (§5), which removes the entire class of "the point is almost on the line" failures that plague
  floating-point geometry kernels.
- **No accumulated drift.** Re-evaluating a construction recipe a hundred times, or grading a size and
  re-grading it, cannot accumulate representation error.

### 1.1 Domain limits (declared tighter than the type)

The type can hold ±9.22 × 10¹⁸ µm. The **declared domain** is far tighter, so that intermediate
arithmetic (products, sums, offsets) cannot overflow:

| Quantity | Declared limit | In human units |
| --- | --- | --- |
| a coordinate or a length | \|v\| ≤ 10⁹ µm | ±1 km |
| an area (a product of two lengths) | ≤ 10¹⁸ µm² | 1 km² |
| a piece bounding box | ≤ 10⁷ µm per side | 10 m |

A value outside the declared domain is a **typed error** naming the quantity and the operation, never a
silent clamp, wrap or saturation. Garment geometry lives inside three metres; a coordinate of one
kilometre means a unit-conversion bug, and the domain limit exists so that bug is reported at the
operation that produced it.

### 1.2 Angles: fixed-point microdegrees

**Angles SHALL be stored as signed 64-bit integers counting microdegrees (µ°, 10⁻⁶ degree).** A full
turn is `360 000 000`; a right angle is `90 000 000`; a half-degree is `500 000`.

Microdegrees rather than radians because the domain speaks in degrees and fractions of degrees — grain
deviation, dart intake angles, notch directions, bias at 45° — and because a rational number of degrees
is stored **exactly**, while π-based storage would make the commonest angles approximate. Trigonometric
functions are evaluated in `f64` at the point of use and their results are rounded into the internal
units (§2); an angle *value* is never stored as a float.

Direction fields on entities SHALL be normalized to `[0, 360 000 000)` µ° and compared as normalized
directions. Formula angles and sweep fields SHALL preserve sign and complete turns; formula bindings
round at their declared quantum without direction modulo, and equality compares their signed values.
Thus a formula sweep of 360 degrees differs from zero even though both give the same direction.

This is the director's D84 clarification. The normalized `sc-units::Angle` represents a direction;
product formula storage/evaluation remains pending. The reference's signed inverse-trig outputs and
binding/equality controls remain owned by G1-SLICE.5a.3b.3c; the
[angle annex](../annexes/formula-syntax.md#reference-angle-conversion-and-direction-controls) states
current verification boundaries.

### 1.3 Dimensional typing

Every quantity carries its dimension, and dimensions are checked at compile time where the type system
allows it and at expression-evaluation time everywhere else:

| Dimension | Internal unit | Notes |
| --- | --- | --- |
| Length | µm | coordinates, widths, allowances, seam lengths |
| Angle | µ° | grain, dart intake, arc sweeps |
| Area | µm² | only as a derived product; never an input |
| Ratio | dimensionless, scaled 10⁻⁶ | ease ratios, shrinkage percentages, grade multipliers |
| Count | integer | pieces, plies, notches, stitch counts |

An expression that adds a length to an angle, or multiplies two lengths and calls the result a length,
SHALL be rejected with a typed diagnostic naming both operands. Non-finite values (NaN, ±∞) cannot exist
in the internal representation; they are rejected at the boundary where a float enters (§3).

## 2. Rounding and conversion

**One rounding rule:** round **half away from zero** — `+0.5` → `+1`, `−0.5` → `−1`. It is the rule a
patternmaker expects, it is symmetric about zero (so mirroring a piece cannot shift it), and it is
exact to implement on integers.

**One conversion rule:** every conversion SHALL be a **single multiplication followed by a single
division**, using an exact integer ratio. Chained conversions are forbidden: converting inches →
centimetres → millimetres → micrometres rounds three times and produces a different answer than
inches → micrometres.

| From | To µm | Exact ratio | Note |
| --- | --- | --- | --- |
| millimetre | µm | × 1 000 | exact |
| centimetre | µm | × 10 000 | exact |
| metre | µm | × 1 000 000 | exact |
| inch | µm | × 25 400 | exact |
| HPGL plotter unit | µm | × 25 | exact: 1 016 plotter units per inch, and 25 400 ÷ 1 016 = 25 |
| PostScript/PDF point | µm | × 25 400 ÷ 72 | **not** an integer number of µm (≈ 352.78 µm) — see §2.1 |

### 2.1 Formats whose quantum is not an integer number of micrometres

Some target formats cannot represent our internal unit exactly. The PDF point is the case in point:
1 pt = 1/72 in = 352.7 µm. For these formats:

1. Geometry is computed **entirely** in internal units.
2. The conversion to the format's coordinate space happens once, at serialization, with the declared
   rounding rule applied to the format's own quantum.
3. The resulting quantization is recorded in the artifact's **format-quantization tolerance** (§3, class
   T3), so a round-trip comparison compares at the format's resolution and never claims precision the
   format cannot carry.

A format is never allowed to feed precision back into the model: importing a PDF or a DXF produces
values quantized to that format, marked as imported primitives with no fabricated history, and the loss
report (gate G6) states the quantization applied.

### 2.2 Display and input units are presentation

Users work in millimetres, centimetres, inches and fractional inches (¼, ⅛, ⅙₄). Display formatting and
input parsing are **presentation concerns**: they convert to and from internal units with the rules
above and never store a float. Locale handling is specified in the internationalization chapter; in
particular a decimal comma in input SHALL NOT change the stored meaning of a value, and canonical
project files are locale-independent.

### 2.3 Public rounding endpoints

The shared `sc_units::round::div_round_half_away_from_zero` accepts signed i128 numerator and
denominator and returns an i64 or UnitError. A zero denominator returns DivisionByZero; a rounded
quotient wider than i64 returns Overflow. Both i64 endpoints remain valid, including the negative
endpoint whose magnitude is one greater than the positive endpoint's. There is no narrower input
precondition, clamping or wrapping.

D78's extreme negative i128 input previously panicked before the checked conversion. The repair
narrows an unsigned magnitude safely before reconstructing its sign, with the negative i64 endpoint
handled explicitly. Quotient/remainder arithmetic stays exact: absolute i128 values are at most
2^127, and twice a remainder below that denominator fits u128. The half-away rule is unchanged.

```bash
cargo test -p sc-units --test round_contract
cargo test -p sc-units --release --test round_contract
bash docs/tasks/artifacts/formula_structure/run_round_mutations.sh
```

Four public contracts cover wide inputs, signed endpoints, denominators, ties and zero. Thirty-six
explicit rows agree with an independent arbitrary-precision Fraction oracle; five actual production
guard mutations require assertion failures and exact restoration. Run mutation checks alone, without
other builds/probes/gates. These are primitive numeric checks, separate from formula evaluation,
geometry, physical acceptance and release proofs.

## 3. The five tolerance classes

There is no global epsilon. **Every comparison names its class**, and each class's value is *derived
from a downstream requirement and recorded with it* — a tolerance chosen to make a test pass is a test
that proves nothing.

### T1 — numerical (`ε_num`)

- **Bounds:** equality of values that are identical by construction: the same length computed
  twice, a mirrored piece against its original, save → load → save
- **Who sets the value:** derived from the representation — one quantum
- **Default:** **1 µm**
- **A violation is reported as:** an internal invariant failure (a bug), not a user-facing
  condition

### T2 — geometric approximation (`ε_geo`)

- **Bounds:** deviation introduced by approximating one curve with another: arc → Bézier, Bézier
  offset, tessellation for internal use
- **Who sets the value:** derived from the tightest downstream consumer of that geometry
- **Default:** **10 µm** for internal geometry; **100 µm** chordal for polyline-only targets
- **A violation is reported as:** a diagnostic naming the edge and the achieved deviation

### T3 — format quantization (`ε_fmt`)

- **Bounds:** the quantum of the target format: DXF decimal resolution, 25 µm plotter units,
  1/72 in PDF points
- **Who sets the value:** fixed by the format and the writer's declared precision
- **Default:** per format, computed not chosen
- **A violation is reported as:** not a failure — it is the resolution the artifact carries, and
  it is published with the artifact

### T4 — importer comparison (`ε_imp`)

- **Bounds:** differences between our artifact and the same artifact after a target system
  imported and re-exported it
- **Who sets the value:** `ε_fmt` of both sides plus the receiver's own rounding, declared per
  receiver in the Factory Profile
- **Default:** per receiver, recorded with the evidence
- **A violation is reported as:** a conformance finding scoped to that receiver and version

### T5 — physical acceptance (`ε_phys`)

- **Bounds:** deviation of a printed, plotted or cut artifact from its canonical geometry
- **Who sets the value:** **the factory's own tolerance**, recorded as profile evidence — never
  invented by this project
- **Default:** unset until a profile supplies it
- **A violation is reported as:** a blocked release or a recorded human disposition

Rules that make the classes usable rather than decorative:

- A class MAY be tightened by a Factory Profile or by an export mode; it SHALL NOT be loosened silently.
  A loosening is a profile parameter with an artifact effect, visible in the release manifest.
- Comparing a value against the wrong class is a defect. T1 is for values that *should* be equal;
  using T2 there would hide real bugs, and using T1 across a tessellation would report noise as failure.
- T5 is the only class whose value comes from outside the software. A printed scale square measured
  with a ruler is a T5 check; a DXF round-trip is T4; neither may be recorded as the other.
- Each declared value carries its derivation — what requirement produced it — so a reviewer can ask
  "why 10 µm?" and get an answer that is not "it passed".

## 4. Curve representation

The supported curve set is exactly three primitives, chosen to match what the interchange formats
actually carry:

1. **Line segment** — two endpoints.
2. **Circular arc** — stored canonically as centre, radius, start angle, end angle and direction
   (counter-clockwise positive). The DXF *bulge* form (start, end, tangent of a quarter of the included
   angle) is a serialization detail, converted with the single-conversion rule of §2.
3. **Cubic Bézier** — four control points. Quadratics are promoted to cubic exactly (a standard
   degree-elevation, no approximation).

**NURBS and higher-order curves are deferred.** An imported or requested NURBS entity produces a typed
rejection naming the unsupported class and the loss-report entry, never a silent flattening.

Curve joins carry a declared continuity: **G0** (touching), **G1** (tangent-continuous) or **G2**
(curvature-continuous). Continuity is verified with the geometric approximation class (T2), because it
is a property of approximated geometry; endpoint coincidence is verified with T1, because it is a
property of the model.

## 5. Robust predicates: exact where possible, adaptive where not

Because coordinates are integers, the predicates that decide *topology* are computed exactly:

- **Orientation** of three points (sign of the cross product) is exact in 128-bit integer arithmetic:
  with \|v\| ≤ 10⁹ µm, a product fits in 60 bits and a difference of two products in 61. No epsilon is
  involved, and the answer is never "nearly collinear".
- **Segment intersection** — including the on-segment and shared-endpoint cases — is decided by
  orientation tests, so it is exact as well.
- **Point-in-polygon** and winding are sums of exact orientation tests.

Predicates that genuinely involve curved geometry (arc/arc, arc/Bézier, Bézier/Bézier intersection,
closest point on a curve, arc length) use adaptive-precision arithmetic (Shewchuk-style expansions or
an equivalent vetted crate) with the T2 class bounding the reported result. Where a curve operation
cannot meet its declared bound, it fails explicitly (§6) instead of returning a plausible value.

A consequence worth stating for implementers: **the numeric tolerance class T1 does not apply to
segment topology.** Two segments either intersect exactly or they do not. Code that writes
`if (distance < EPSILON)` for integer coordinates is a defect, because it re-introduces the ambiguity
the representation exists to remove.

## 6. The offset error budget

Offsetting a boundary — for a seam allowance, a hem, a facing — is the highest-risk geometric operation
in the product, and it is treated as such:

- An exact offset of a Bézier curve is not a Bézier curve. The engine therefore produces an
  **approximation with a declared error bound**, defaulting to T2 (10 µm internally, 100 µm chordal for
  polyline-only targets), and the bound is part of the operation's contract.
- When the bound cannot be met — a cusp, a near-tangency, a self-intersection introduced by a concave
  corner, an allowance wider than the local radius of curvature — the operation **fails explicitly**
  with a diagnostic naming the edge, the achieved deviation and the requested bound. It never emits
  out-of-tolerance geometry and never silently repairs topology.
- Corner treatment (miter, slant, envelope, trim, step) is a declared parameter of the seam allowance,
  not an engine default, and the choice is recorded in the release manifest.
- Topology repair, where an export mode requires it (a polyline-only receiver cannot represent a
  self-intersecting allowance), is a **declared, logged transformation** with its own error accounting;
  the loss report classifies it as approximated.
- The pathology corpus at gate G2 is the oracle for this section: acute angles ≤ 45°, cusps,
  near-tangencies, self-intersecting loops, corner joins and hem allowances, each with the bound it
  must meet. An offset engine that passes its own happy-path tests and not the corpus has not been
  tested.

## 7. Determinism at the serialization boundary

Artifacts pass through a canonicalizer before comparison (gate G2). The numerical part of that policy:

- Floats appear only where a format demands them, formatted with a **declared fixed precision** — enough
  decimal places to represent one internal quantum with margin, and never more than the format's own
  resolution justifies.
- No wall-clock value, no locale-dependent decimal separator, no platform-dependent float printing
  enters a canonical artifact.
- Two runs of the same commands produce byte-identical artifacts; this is a testable claim and is
  tested on more than one platform.

## 8. Verification status of the external claims in this chapter

Per the claim-verification standard, each external fact is labelled with how it was established.

- **1 in = 25 400 µm; 1 cm = 10 000 µm; 1 mm = 1 000 µm** — exact by definition — arithmetic
- **HPGL default plotter unit = 25 µm, from 1 016 units per inch** — arithmetic on the roadmap's
  stated 1 016 units/inch; the plotter-unit default is **cited from the roadmap**, to be
  confirmed against a plotter manual when the HPGL writer lands (gate G4)
- **1 PostScript/PDF point = 1/72 in ≈ 352.78 µm** — arithmetic on the definition of the point;
  the PDF coordinate-space behaviour is **to be confirmed** against the PDF reference during the
  writer evaluation (gate G2)
- **DXF carries no intrinsic unit; interpretation comes from the header/profile** — **cited from
  the roadmap's dialect decision**; the header variables that carry it per DXF version are to be
  confirmed when the writer lands (gate G2)
- **ASTM D6673-10 withdrawn; AAMA/ASTM layer conventions** — recorded in the interchange-
  dialects chapter with its own verification status

An unverified claim is not a reason to omit the rule — it is a reason to name who confirms it and when.

## 9. What must be true in tests

The following are conformance requirements, not suggestions; each maps to an oracle in the conformance
matrix (gate G2):

- **Conversion round-trip:** for every supported display unit, converting to internal units and back
  yields the original value within T1, except where the display quantum is coarser than 1 µm, in which
  case the round-trip is within that quantum and the test says so.
- **Single-conversion rule:** a chained conversion that disagrees with the direct one is a test failure
  in the chained code path, not a curiosity.
- **Rounding symmetry:** rounding is symmetric about zero, and mirroring a piece then rounding equals
  rounding then mirroring, within T1.
- **Predicate exactness:** a corpus of degenerate and near-degenerate integer configurations
  (collinear points, shared endpoints, touching segments) is classified correctly with no epsilon.
- **Class separation:** a test suite that deliberately uses the wrong class produces the wrong verdict,
  proving the classes are load-bearing rather than decorative.
- **Offset bounds:** the pathology corpus meets its declared bound or fails loudly (§6).
- **Overflow and invalid input:** values beyond the declared domain, division by zero, and non-finite
  floats at a boundary all produce typed diagnostics, never wraps or NaNs.
