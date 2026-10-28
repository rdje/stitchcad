# The numerical contract: fixed-point micrometres, microdegrees, exact predicates

- **Type:** `decision`
- **Date:** `2026-09-29` (absolute)
- **Status:** `active`
- **Owner / source:** repo-local workflow, leaf `G0-CONTRACT.2`; the normative specification is
  `docs/book/src/spec/units-and-tolerances.md` (roadmap §4.2, §2.7, locked decision §15.7)

answers: "what unit does StitchCAD store lengths in?" · "why not floating point?" · "how are angles stored?" · "how do we round?" · "is there a global epsilon?" · "why are intersections exact?" · "what tolerance do I compare two seam lengths with?" · "how will sqrt and hypot round exactly?" · "how can arc_length round exactly with bounded pi precision?"

## The fact / decision

1. **Lengths are signed 64-bit integers of micrometres (µm).** One inch is `25 400` exactly; one
   millimetre is `1 000`.
2. **Angles are signed 64-bit integers of microdegrees (µ°).** A full turn is `360 000 000`. Rational
   degree values — the domain's own vocabulary — are stored exactly. Formula function results must
   satisfy the true-value quantum contract; floating point alone is no rounding proof or storage form.
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
  boundaries and inside proven trigonometric algorithms. Formula calls still require the correctly
  rounded true value; ordinary platform libm or a fixed precision alone cannot certify that result.
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

## Exact root-family algorithm — 2026-10-25

Leaf `G1-SLICE.5d.1.1` selects exact integer arithmetic for sqrt/hypot before their implementation.
The contract remains formula-language §4.2: nearest internal quantum, ties away from zero. The
legacy float summary above is clarified against that existing formula-specific requirement.
Transcendental algorithms stay unresolved/owned by `.5d.1.2`; this is no certificate for them.

For nonnegative rational z=N/D with D>0, compute q=floor(N/D), then k=floor(sqrt(q)) by integer
bisection. Because k² is integer, k²<=z iff k²<=q: this is also floor(sqrt(z)). Start lower=0 and
upper=2^ceil(bits(q)/2), a strict upper root bound, including upper=1 for q=0. Each comparison of
mid² with q preserves lower²<=q<upper² and halves the integer interval until upper-lower=1.
Return k+1 exactly when 4N>=D(2k+1)²; equality is the positive half tie. Otherwise return k. No
floating point, approximate root or empirical precision cutoff is involved.

Area sqrt uses N=n,D=d in square-micrometre units. Ratio sqrt uses N=n*1,000,000,D=d to return
ppm, since sqrt((n/d)/1,000,000)*1,000,000=sqrt(n*1,000,000/d). Negative operands refuse before
root work. Hypot of signed lengths n1/d1,n2/d2 uses N=n1²*d2²+n2²*d1²,D=d1²*d2². It never
constructs an area value or applies an area domain/width limit inside the function. Only its final
rounded Length is checked. Both zero and exact half roots are covered by the same integer rule.

Admitted operands bound all workspace. Area/ratio numerators have at most128/148 bits; their
scalar quotient bounds require at most30/74 bisections. Hypot has at most513 numerator and512
denominator bits; completed length domains give q<=2*10^18<2^61, hence at most31 bisections.
Its largest conservative midpoint product is below2^576. Ratio midpoint products need at most278
bits. All integer work is finite and bounded by admitted inputs, with no new dependency or platform
libm. Integer outputs fit at most75 bits; binding i64 remains its own later check.

Independent focused fixtures will use standard-library math.isqrt plus authored exact midpoint
squares and rational neighbors, including near ties beyond ordinary float precision, full128-bit
Ratio, signed/zero hypot and maximum-length cases whose squared sum exceeds the area domain.
Broader/platform/mutation work stays `.5d.4`/pre-push. Source-free numeric kernels do not attach
provenance; later evaluator `.5e.3` must retain every root-call contribution, including perfect roots.

Implementation0255: checked_sqrt now follows this plan for area/ratio; focused native2/0 includes
15 independent fixture rows. Hypot and source-bearing evaluator provenance remain later owners.

Implementation0256: checked_hypot reuses the exact root kernel on its private squared sum; focused
native4/0 covers all25 root rows. Value kernels are available; source provenance remains .5e.3.

## Bounded arc_length rounding certificate — 2026-10-25

Leaf `G1-SLICE.5d.1.2.1` chooses a fixed certified pi enclosure for arc_length before implementation.
This proves the entire admitted rational-input family for this function; no precision guess or new
floating library is required. Sin/cos/tan/atan/atan2 remain owned by `.5d.1.2.2` before their code.

For Angle n1/d1 in microdegrees and Length n2/d2 in micrometres, work in magnitudes:
arc=A*pi/B, A=n1*n2<2^256, B=180,000,000*d1*d2>0. Apply the product sign after rounding; zero A
returns canonical zero. Neither angle turns nor radius sign are discarded. The existing reference
and normative signature apply this signed product without adding a positive-radius restriction.

An output half-quantum k+1/2 corresponds to pi=(2k+1)B/(2A). Its reduced denominator is below2^257.
Find adjacent fractions a/b<c/d with determinant bc-ad=1, b,d<=Q=2^257 and b+d>Q, enclosing a
closed certified pi interval [L,U] strictly. Any reduced p/q inside (a/b,c/d) satisfies
q=b(cq-dp)+d(bp-aq)>=b+d>Q because both parenthesized integers are positive. Therefore no admitted
half-quantum target is anywhere in [L,U]. Every admitted nonzero coefficient rounds identically at
L, at true pi and at U. One endpoint's exact rational half-away rounding is sufficient at runtime.
This argument needs no exhaustive input enumeration and no assumed distance from a rounding tie.

The retained certificate uses L=floor(pi*2^640)/2^640 and U=L+2^-640. Its neighboring denominators,
strict containment and determinant are verified exactly by the pure certificate producer, along
with public Fraction.limit_denominator endpoint checks. The producer's145 continued-fraction steps
are an observed constant-generation result, not a runtime loop or new language limit.

Pi is enclosed independently in two ways. Exact Fraction alternating sums use
pi=16*atan(1/5)-4*atan(1/239) with160 terms. Directed integer sums at672 bits use
pi=4*(atan(1/2)+atan(1/3)) with512 terms and per-term outward division/remainder bounds. Each
alternating remainder lies strictly between zero and its next signed term, by integrating the
finite geometric identity for1/(1+x²). Rational tangent identities give1 for both principal angle
combinations; their values lie in(0,1), inside the principal tangent interval, so both equal pi/4.
The two independent arithmetic/identity paths certify the same640-bit floor.

The exact artifact is crates/sc-core/tests/fixtures/pi640_certificate.tsv, reproduced by
python3 -I -B docs/tasks/artifacts/formula_structure/pi_rounding_certificate.py. The standard-library
[closest-fraction API](https://docs.python.org/3/library/fractions.html#fractions.Fraction.limit_denominator)
is an additional endpoint check. The certificate contains the enclosure and both adjacent fractions;
its determinant/denominator/enclosure predicates are sufficient to check midpoint exclusion.

Runtime uses at most898 numerator and924 denominator bits: pi's scaled numerator fits642 bits,
A fits256, and B fits284. There is a fixed-size exact quotient/remainder rounding, followed by
reduced128-bit and completed Length-domain checks; binding i64 remains later. Rounded magnitude
fits at most133 bits under the admitted Angle/Length domains. No intermediate width/domain refusal
or direction modulo is introduced. Negative coefficient signs use the same magnitude certificate.

Production arc_length remains `.5d.3.1`; source-call provenance remains `.5e.3`. Focused future
fixtures must cover signed/full turns, zero, bounded rational neighbors around half quanta, scalar
and width refusals, and invalid actual kinds. Broad/platform/mutation work remains `.5d.4`/pre-push.

Research also located published correctly-rounded float APIs in dashu-float and astro-float.
They are unselected candidates for the remaining family: exact rational input enclosures,
threshold termination, resource bounds, cached-version licenses and platform behavior still require
review before adoption. No new dependency/license or native/WASM capability is claimed here.

Implementation0258: arc_length now uses the certified endpoint, exact wide rounding and final width/
scalar checks. Native arc2/0 with23 independent rows, numeric6/0 and selected library67/0 pass; the
compiled endpoint is tied to the reproduced certificate. Source provenance remains .5e.3.

Remaining trig: fact_trig-midpoints-and-resource-bound.md proves midpoint exclusion; practical resources remain .5d.1.2.2.2.

## Wide-ratio shared rounding0498

answers: "how can a wide exact workspace reuse rounding without narrowing its ratio?"

sc-units accepts a u128 magnitude floor plus exact fractional-half Ordering through
round_magnitude_from_half_comparison. Ratio entry points derive that comparison from remainder
versus denominator-minus-remainder;all paths use one private increment decision. The consumer owns
exact division/nonzero denominator,workspace,sign and scalar/storage checks. The decomposed API
cannot validate an omitted ratio and never grants authority over a model or physical correspondence.
Required increment beyond u128 returns the new named Overflow;old signed/unsigned values and error
operation names remain unchanged. No dependency or float enters sc-units. Focused native11/source
doctest1/Decimal138 pass;two new contracts include125 nonzero Decimal rows and explicit floor limits.
Old5+9 mutation anchors remain unique after adapting remainder/tie sites;compiled mutation verdict
for the refactor is deferred to full paired acceptance or required pre-push probes. Original campaigns
retain their historical scope;shared primitive implementation alone does not execute weighted ease.
