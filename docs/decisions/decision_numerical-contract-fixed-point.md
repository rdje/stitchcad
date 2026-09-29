# The numerical contract: fixed-point micrometres, microdegrees, exact predicates

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G0-CONTRACT.2`; the normative specification is
  `docs/book/src/spec/units-and-tolerances.md` (roadmap §4.2, §2.7, locked decision §15.7)

answers: "what unit does StitchCAD store lengths in?" · "why not floating point?" · "how are angles stored?" · "how do we round?" · "is there a global epsilon?" · "why are intersections exact?" · "what tolerance do I compare two seam lengths with?"

## The fact / decision

1. **Lengths are signed 64-bit integers of micrometres (µm).** One inch is `25 400` exactly; one
   millimetre is `1 000`.
2. **Angles are signed 64-bit integers of microdegrees (µ°).** A full turn is `360 000 000`. Rational
   degree values — the domain's own vocabulary — are stored exactly; trigonometry is evaluated in `f64`
   at the point of use and rounded into internal units, never stored as a float.
3. **The declared domain is tighter than the type:** \|length\| ≤ 10⁹ µm, area ≤ 10¹⁸ µm². Exceeding it
   is a typed diagnostic, never a clamp, wrap or saturation.
4. **Rounding is half away from zero**, and **every conversion is a single multiply-then-divide** with an
   exact integer ratio. Chained conversions are forbidden — they round more than once and disagree with
   the direct conversion.
5. **There is no global epsilon.** Five named tolerance classes (numerical, geometric approximation,
   format quantization, importer comparison, physical acceptance) bound five different questions, and
   every comparison names its class. Each value is derived from a downstream requirement and carries
   that derivation.
6. **Topology predicates are exact.** With integer coordinates, orientation, segment intersection,
   point-in-polygon and winding are computed in 128-bit integer arithmetic with no epsilon at all.
   Curved-geometry predicates (arc and Bézier intersections, closest point, arc length) use adaptive
   precision bounded by the geometric-approximation class.
7. **Offsets carry a declared error budget** and fail explicitly when they cannot meet it. Out-of-
   tolerance geometry is never emitted and never silently repaired.

## Why

Determinism is a product requirement, not a preference: gate G2 freezes golden bytes, gate G6 runs the
same suite cross-platform, and an agent-driven CLI replay must reproduce an artifact byte-for-byte.
Floating point cannot promise that across compilers, platforms and vectorization decisions — and the
failure mode is the worst kind, because it is a one-micrometre difference that shows up as a diff in a
release artifact nobody can explain.

Fixed-point integers also buy something floats cannot at any precision: **decidability of topology.**
"Is this point on this segment?" has an exact answer, so the entire class of near-boundary ambiguity
that forces floating-point kernels to carry a fuzzy epsilon — and to disagree with themselves after a
mirror — disappears. That is why the numerical tolerance class explicitly does *not* apply to segment
topology, and why `if (distance < EPSILON)` over integer coordinates is recorded as a defect rather
than a style choice.

Microdegrees over radians: the domain speaks in degrees (grain deviation, dart intake, bias at 45°),
and a rational number of degrees is exact in microdegrees while π-based storage makes the commonest
angles approximate. Radians remain the internal currency of trigonometric evaluation, which is where
they belong — at the point of use, not in storage.

Half-away-from-zero over banker's rounding: it is what a patternmaker expects, and it is symmetric about
zero, so mirroring a piece cannot shift it by one quantum. Symmetry matters more than statistical
neutrality here, because patterns are mirrored constantly and a systematic asymmetry would show up as a
left/right mismatch on the cutting table.

Five tolerance classes rather than one, because they answer five different questions and a single number
cannot: two values that should be identical by construction (1 µm — anything more is a bug), a curve
approximated by another curve (10 µm internally, 100 µm chordal for polyline-only receivers), the
quantum a format can carry (fixed by the format), what a receiver's round-trip changes (declared per
receiver), and what a physical cut piece may deviate by (the factory's number, never ours). Collapsing
them is how a project ends up with an epsilon chosen to make a failing test pass.

## How to apply

- Reaching for a float in domain code is a design question, not a shortcut: it is allowed at format
  boundaries and inside trigonometric evaluation, and the result is rounded back into internal units
  immediately.
- Every comparison states its class in code and in the test that exercises it. A reviewer who cannot
  tell which class a comparison uses should treat it as a defect.
- A new tolerance value arrives with its derivation ("the tightest consumer of this geometry is X, which
  requires Y"), recorded in the spec chapter. A value with no derivation is not adopted.
- The offset engine's error budget is part of its contract, and the G2 pathology corpus is its oracle —
  acute angles ≤ 45°, cusps, near-tangencies, self-intersections, corner joins, hem allowances.
- Conversions to formats whose quantum is not an integer number of micrometres (the PDF point at
  ≈ 352.78 µm) happen once, at serialization, and the quantization is published with the artifact.

Related: [[decision_product-work-takes-the-frontier]] · the normative chapter
`docs/book/src/spec/units-and-tolerances.md` · `docs/tasks/G0-CONTRACT.md` (leaf `.2`) ·
`docs/tasks/G1-SLICE.md` (leaf `.2`, `sc-units`).
