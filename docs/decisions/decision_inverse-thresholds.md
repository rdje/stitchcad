# Inverse rounding-threshold certificate feasibility

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`; bounded feasibility only, no full population or native inverse admission
- **Owner / source:** engineer proposal, G1-SLICE.5d.1.2.2.2.b.2.a; complete proof .b/.c

answers: "what bounds inverse-function rounding work?" · "does the inverse cohort admit atan or atan2?"

## Decision

Continue the finite-threshold approach with a bounded inverse cohort. The
[forward decision](decision_finite-threshold-feasibility.md) derives45,000,000 first-octant
half-microdegree targets p_k=(2k+1)/(720M), k=0,...,45M-1, M=1,000,000. Sign/quadrant and
reciprocal-slope symmetries cover other non-axis cases only when their exact runtime contracts
are implemented. Axes and principal branches retain separate obligations.

Atan2's private rational slope can have denominator up to Q=(2^128-1)^2,256 bits. Atan's
ppm-ratio slope is covered by that conservative bound. No public128-bit ratio is materialized
by the proposed certificate. Actual geometry selectors retain their separate input contracts.

For each p_k, enclose tan(2*pi*(p_k-delta)) through tan(2*pi*(p_k+delta)). Strict containment
inside a determinant-one rational gap a/b<c/d with b,d<=Q and b+d>Q excludes all admitted
rational slopes from that angle band, by monotonicity and the exact gap identity in the forward
decision. Complete coverage could prove a uniform inverse-answer margin delta in turns.
This does not itself supply a native inverse algorithm or its error/workspace proof.

## Candidate construction

Try delta=2^-600 turns, scale2^640, the existing certified640-bit pi enclosure and64 directed
sin/cos terms. Rational target endpoints convert to radian intervals without floating point.
Their first-octant arguments are below1 radian. The same width invariant as the forward plan
gives term width<=12 scaled units;64 terms plus outward alternating tail fit770 units.
The exact128! exceeds2^640, so each series tail is smaller than one quantum. Divide the sine
enclosure by the strictly positive cosine enclosure outward to bound tangent. A small extension
beyond pi/4 exists only for the rational45-degree negative control, with argument still below1.

These are experimental certificate parameters, not language limits or admitted runtime budgets.
Any failed experimental gap is investigated; it never becomes a formula refusal or substituted
value. Tangent as a production formula function remains a different resource problem.
[NIST DLMF4.19](https://dlmf.nist.gov/4.19) supplies the series and
[DLMF4.21](https://dlmf.nist.gov/4.21) the angle identities, opened2026-10-26.

## Focused evidence — work unit0282

Run python3 -I -B docs/tasks/artifacts/formula_structure/inverse_threshold_feasibility.py.
It passes38 distinct bands, including endpoint/central neighbors and spread indices, rc=0.
The report scope is inverse_feasibility_cohort_only, retaining exact slope/gap endpoints, source
hashes, candidate parameters and observed cohort timing under target/trig_inverse_feasibility/0282/.
Its timing is maintenance observation, not a guaranteed full-population or native duration.

Four distinct algebraic oracles check tangent at turns0,1/24,1/16,1/8:0,2-sqrt(3),sqrt(2)-1,1.
Square-root bounds use768-bit integer sqrt, independent of the series constructor. Two exact
Fraction arctangent series at slopes1/4 and1/2 independently bracket the phase whose tangent
must contain that rational slope. The actual tan45 rational and a deliberately wide band must
refuse gap certification. Thus an always-accepting predicate cannot pass the controls.
Pure import with Path.read_text patched to refuse also passes, rc=0; pi input remains explicit.

Complete45M coverage, its retained cost/source/hash evidence and an independent exact payload
reader remain .b.2.b. Native atan/atan2 folding/branches/error/workspace admission remains .b.2.c,
then implementation .5d.3.3. No native inverse function, full recipe execution or platform claim
is made by this cohort. The existing forward certificate and sin/cos APIs remain unchanged.
