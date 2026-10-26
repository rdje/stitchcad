# Investigate finite rounding-threshold certificates before admitting trig precision

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`; feasibility investigation only, no production algorithm admitted
- **Owner / source:** engineer proposal, G1-SLICE.5d.1.2.2.2.a; full populations remain .b/parent

answers: "what finite certificate could help bound trig rounding work?" · "does the threshold cohort admit a runtime precision?"

## Decision and scope

Investigate a finite certificate over output rounding thresholds, using rigorous intervals and
exact rational-gap predicates. Start with a bounded sin/cos cohort and independent controls.
Only complete population coverage, a justified native-kernel error/resource bound and applicable
integration evidence could admit a production method. This changes no input, quantum, width,
branch, precision limit, dependency or authority rule. Tangent remains a separate resource problem.

The proposal is our derivation from the existing numerical contract. The
[midpoint fact](fact_trig-midpoints-and-resource-bound.md) establishes qualitative termination;
the generic separation estimate there remains insufficient for practical admission.
[NIST DLMF4.24.3](https://dlmf.nist.gov/4.24.E3) supplies the arctangent series;
[DLMF4.23](https://dlmf.nist.gov/4.23) supplies inverse-function branch conventions.
Both pages were opened2026-10-26. They do not certify this prototype, its interval arithmetic,
runtime error accounting, population coverage or performance.

## Exact finite populations and input bounds

Let M=1,000,000 and U=2^128-1. Positive sine/cosine rounding thresholds are
r_j=(2j+1)/(2M), j=0,...,M-1: exactly1,000,000 values. Sign/quadrant symmetries cover the rest.
An admitted rational microdegree angle n/d has a turn representation with denominator360M*d.
Exact period reduction and integer-microdegree quarter/octant shifts preserve that representation.
Therefore Q_phase=360M*U is a conservative denominator bound; its bit length is157.
The admitted angle1/U microdegree attains this turn denominator, so the bound is not a guessed
precision. Cosine's quarter turn is an integer number of microdegrees, adding no denominator factor.

For inverse functions, integer90M-microdegree shifts, sign and reciprocal-slope symmetries reduce
half-microdegree thresholds to the first octant:45,000,000 positive targets
p_k=(2k+1)/(720M) turns, k=0,...,45M-1. Exact axes and branch conventions must be handled separately.
Atan's real input slope n/(M*d) has denominator at most M*U. For atan2, exact division of two
admitted rationals can have numerator/denominator up to U^2 without materializing a public ratio.
Thus Q_slope=U^2, of bit length256, covers both families and reciprocal folding. The Ratio pair
1/U and U attains slope denominator U^2. These bounds do not certify arbitrary wider geometry
intermediates; actual selector argument contracts retain .5f.3 ownership.

The prototype re-derives these integers and exact extremal witnesses with Python Fraction arithmetic.
It does not enumerate either full threshold population in the feasibility leaf.

## Exact gap certificate

Find L=a/b < H=c/d with positive b,d<=Q, bc-ad=1 and b+d>Q. No rational p/q strictly between
them can have positive q<=Q. Indeed bp-aq and cq-dp are positive integers, and
q=d(bp-aq)+b(cq-dp)>=d+b. This elementary identity is the verification predicate; choosing
candidate neighbors by continued fractions is not itself evidence until every predicate passes.

For sine, enclose asin(r_j-delta)/(2*pi) through asin(r_j+delta)/(2*pi) strictly inside such a gap
with Q=Q_phase. Then no admitted phase can produce a true sine within delta of that threshold.
This includes midpoint exclusion. A later rigorous runtime sine enclosure of width at most delta
would have unambiguous nearest-ppm rounding. The complete certificate must cover every j.
This argument avoids enumerating the rational input space; it does not avoid the output population.

For inverse functions, the corresponding planned certificate encloses tan(2*pi*(p_k-delta_turn))
through tan(2*pi*(p_k+delta_turn)) inside a gap of order Q_slope. Monotonicity then excludes
rational slopes whose true inverse answer is inside that angle band. This inverse generator and
its complete population are not implemented or verified by the initial forward feasibility cohort.

## Candidate interval construction

Use positive directed integer intervals at scale S=2^P. Floor lower endpoints and ceil upper
endpoints for products/divisions; bound square roots by integer sqrt of the scaled endpoints.
The existing independently certified640-bit pi enclosure is the input, not a new pi assumption.

For 0<=s<=1, set x=s/(1+sqrt(1-s^2)); then asin(s)=2*atan(x). Five repetitions of
x <- x/(1+sqrt(1+x^2)) leave a true argument at most1/32. The identities follow real principal
branches and the tangent double-angle formula. Consequently asin(s)/(2*pi)=32*atan(x)/pi.
Evaluate the alternating arctangent series with directed power/term bounds. After N terms,
the remainder magnitude is at most x^(2N+1)/(2N+1). Choosing N=ceil(P/10) and x<=1/32 makes
that remainder smaller than one interval quantum. Arithmetic rounding still needs explicit bounds.

The feasibility profile tries P=512 and delta=2^-384 for the forward value band. These are
experimental certificate parameters, not admitted runtime budgets or additional language limits.
A failed gap is a failed experiment; it must not become a formula refusal or a substituted result.
The eventual runtime kernel's full error and workspace argument remains separate from this producer.

## Controls and admission boundary

The directed interval path is checked against exact inverse values0,1/12,1/4 turns at sine0,1/2,1.
An independently written exact Fraction arcsine series supplies additional containment oracles.
Its positive-series recurrence and tail use [DLMF4.24.1](https://dlmf.nist.gov/4.24.E1),
independent of the prototype's half-angle/arctangent path.
For the highest positive ppm midpoint, sqrt((1-s)/2)=1/2000 exactly; the complementary arcsine
identity gives a distinct, fast near-extremum oracle. Small-Q exhaustive rational enumeration
checks the gap selector/verifier. The scale1 sin30-degree midpoint and a deliberately wide band
must fail certification. An always-accepting or always-refusing predicate cannot satisfy both controls.

Run python3 -I -B docs/tasks/artifacts/formula_structure/finite_threshold_feasibility.py.
Its project-local report must say feasibility_cohort_only. Timing is observed maintenance cost;
it is not an all-input runtime bound or a validated full-population duration. No production trig
method is selected until its entire applicable proof, coverage and resource obligations close.

## Feasibility evidence — work unit0277

The command above passes38 distinct forward bands, known3/Fraction4/small-Q32/negative2 controls,
rc=0. The cohort includes both extrema, center neighbors and32 spread indices (duplicates removed).
The observed cohort loop took13,370,083ns on this run; setup/oracles are outside that timer.
It is an observation, not a throughput guarantee. The report retains exact interval/gap endpoints,
candidate parameters and producer/pi hashes under target/trig_threshold_feasibility/0277/.
An import with pathlib.Path.read_text patched to refuse passes, rc=0: helpers require explicit
pi bounds and neither activate the producer profile nor read input/output content on import.
Complete forward coverage remains .b; inverse generation/coverage and tangent resources remain open.
