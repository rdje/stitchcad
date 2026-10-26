# Bounded native atan and atan2 algorithm plan

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`; algorithm plan admitted and native values implemented0288
- **Owner / source:** G1-SLICE.5d.1.2.2.2.b.2.c, after complete inverse certificate0286

answers: "how will atan and atan2 round without precision cutoffs?" · "what bounds inverse workspace?" · "how are principal branches retained?"

## Admitted plan and source basis

Use576 fractional bits, five half-angle reductions and64 directed arctangent terms. The
[complete inverse certificate](decision_inverse-thresholds.md) and its tracked compact witness
prove distance greater than2^-540 turns from every relevant half-microdegree threshold.
This is an algorithm budget, not an extra language limit or a precision-failure result.
The focused plan producer checks the exact certificate profile and all five live source hashes.
It does not repeat the45M run or promote cached metadata into a substitute for its completed reader.

Atan converts internal ppm Ratio n/d to real slope n/(M*d), M=1,000,000, in private workspace.
Atan2 forms exact y/x from two admitted homogeneous Length or Ratio values without constructing
a public Ratio. Its two numerator/denominator products fit256 bits. These argument families,
including reciprocal folding, are covered by Q=(2^128-1)^2. Wider geometry inputs stay .5f.3.

## Exact folds, axes and branches

Reduce positive slope to[0,1] by reciprocal when needed. After computing its base angle phi in
turns, reciprocal folding gives1/4-phi. Atan restores the original sign without direction modulo.
For atan2, obtain the first-quadrant magnitude from abs(y/x); negative x gives1/2-theta and
negative y restores the negative sign. Exact zero y with negative x is positive1/2 turn;
zero x gives the signed quarter turn. Both zero refuse formula_domain after actual kind checks.
The true branch is(-180,+180] degrees; rounded negative near-axis results may equal-180.
No modulo may turn that rounded result into+180. Binding and provenance remain separate contracts.

## Directed half-angle enclosures

Set S=2^576. Convert the folded exact rational to floor/ceiling scaled endpoints, width<=1.
Outwardly truncate the existing certified640-bit pi enclosure to576 bits, width1. For each
half-angle step use x <- x/(1+sqrt(1+x^2)); thus atan(original)=32*atan(final).

If the input interval has scaled width w<=11, its squared interval Z has width<=2w+2.
For radical endpoints use N_lo=S*(S+Z_lo), N_hi=S*(S+Z_hi). Compute r=floor(sqrt(N_lo))
by integer Newton. Set h=ceil((N_hi-r^2)/(2r)) and radical upper=r+h. Its square is at
least N_hi. Since r>=S and N_lo-r^2<=2r, h<=ceil(width_Z/2+1)<=w+2.
This supplies the upper enclosure with a small exact division rather than another root solve.

The denominator lower is at least2S. Outward division gives next width
<=w/2+(w+2)/4+2<=11. The upper endpoint halves, so after five steps it is at most S/32.
All intervals remain nonnegative and include the true value; no low-width public rational is made.

## Integer Newton termination

Every root input N is in[S^2,2S^2]. Initialize t=2S and iterate
t_next=floor((t+floor(N/t))/2), stopping when t_next>=t. With r=floor(sqrt(N)),
t>=r is invariant. When t>r the next integer is strictly smaller; a stopping point is r.
For error e=t-r, e_next<=floor(e^2/(2S))+1. Starting E=S, the tracked exact recurrence
reaches E<=1 after11 updates. At error1, the next step is exactly r, then one stopping test
finishes: at most13 divisions per root. This is a proved bound, not a runtime iteration cutoff.

## Arctangent series and rounding error

Evaluate the interval's alternating series directly. Its square has width<=3 and upper<=S/1024.
The first power has width<=11. Every later power has width<=3 because
11/1024+3/32+2<3; subsequent recurrence is tighter. The first term has width<=11 and
each later divided term width<=5. With one outward unit per side for the alternating remainder,
64 terms have total width<=11+63*5+2=328 units. The remainder is bounded by
(1/32)^129/129<2^-645<1/S. Clipping the lower bound to zero preserves containment.

Base phase is16*atan(final)/pi. Its numerator upper is below S and pi lower exceeds3S.
The quotient width is below16*328/3+1/9+2<4096 units: phase width<2^-564 turns.
Exact binary quarter/half complements and sign restoration preserve that width. It is narrower
than the certified2^-540 threshold distance. Exact half-away rounding of the final magnitude's
lower endpoint times360,000,000 therefore yields the true nearest microdegree for every admitted
operand; axes use their exact constants. No adaptive precision loop, libm or substituted value.

## Work, space and verification boundary

There are five bounded Newton roots, five algebraic upper enclosures and one64-term series,
plus fixed-width input/fold/final divisions. Every intermediate magnitude is below2^1154:
root arguments/products are at most2S^2 plus small endpoint rounding, input products fit832 bits,
and term/final-angle work is smaller. Results have magnitude at most180,000,000 microdegrees.
No dependency, scalar domain, rational width, tolerance or authority contract changes.

[NIST DLMF4.24](https://dlmf.nist.gov/4.24) supplies arctangent series, derivative and addition
identities; the real half-angle identity follows the principal tangent double-angle formula.
The focused independent oracle uses768-bit rational addition reduction around atan(1/2) and
512-term series, with a distinct exact Fraction pi identity; it uses no Newton or half-angle path.

python3 -I -B docs/tasks/artifacts/formula_structure/atan_runtime_plan.py passes exact envelopes,
14 atan/16 atan2 oracle cases,2 zero-vector refusals,4 math.isqrt comparisons and2 insufficient
budget controls, rc=0. Observed maximum Newton steps is11;13 is the proved bound. Full-width
private slopes, signed quadrants, axes and near negative-axis endpoints are included. This is
plan/model evidence, separate from the native implementation receipt below.
source-bearing calls/AST and broader/platform proof retain .5e/.5d.4/pre-push ownership.

## Native transcription0288

Private recipe/inverse.rs follows the admitted plan with shared pi endpoints and no new dependency
or precision/domain refusal. FormulaNumericValue exposes checked_atan/checked_atan2; actual kind
priority and typed Atan2ZeroVector precede numeric special cases. The native target checks27/54
independent values, signed principal endpoints, full-width private slopes and hard midpoint neighbors.
Its standalone768-bit addition oracle uses no native root/half-angle path and reproduces the fixture.
Focused native/format/selected strict Clippy evidence belongs to G1-SLICE.5d.3.3.1.
