# Finite rounding-threshold certificates and the sine/cosine algorithm plan

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`; sin/cos plan admitted0279, native implementation pending; inverse/tangent unresolved
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
At0277, complete forward coverage remained .b; inverse generation/coverage and tangent resources remain open.

## Full forward producer — work unit0278

The .b.1 producer iterates every j=0,...,999999 directly, with no interpolation or sampled skips.
It retains100 contiguous10,000-row chunks under target/trig_forward_certificate/0278/. Each208-byte
row stores both512-bit phase endpoints and four160-bit Farey components. Source hashes bind the
producer, pure helpers, independent reader, local-environment profile and pi fixture. A manifest
checkpoint publishes only after the chunk passes exact reading; resume rechecks every published
chunk and refuses changed sources. A stopped partial run never counts as complete evidence.

The independent reader imports no interval constructor. It checks every encoded size/hash,
contiguous index range, determinant, denominator bound/sum, strict containment and positive phase
range. It does not independently recompute asin enclosures; those remain the directed arithmetic
derivation and distinct oracle/control evidence. Implicit payload row order is bound to actual
threshold indices by the retained producer, rather than an unrelated list of passing gaps.

Run python3 -I -B docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py.
The optional --chunks1 deliberately stops after one new chunk. Re-run without it to resume.
Then run python3 -I -B docs/tasks/artifacts/formula_structure/verify_forward_certificate.py.
Both commands completed, rc=0:1,000,000 rows/100 contiguous chunks, every strict gap passing.
The one-chunk stop/resume and complete-reader's partial refusal were exercised before completion;
three corrupted-row controls were also rejected. The final lock is absent. The retained payload
is208,000,000 bytes. Recorded generation-loop intervals sum to363,094,133,583ns on this run;
this excludes setup/resume/reader costs and is not a performance guarantee.

The independent reader's minimum integer scaled phase margin is
2580829885918382794552188695284784878828128897754713946, strictly positive. The scale is2^512;
this is distance from an encoded phase band to its gap boundary, not a native sine error budget.
SHA256 of the concatenated100 binary chunk digests in index order is
90f7f6cc9fb3ae3f84ad3389454d6dfc1f3d391038697b068397968f7e6d2406.
The manifest retains each digest and all five source hashes. The producer/reader are tracked;
regeneration and source/hash refusal make this reproducible rather than a sampled extrapolation.

Together with the directed interval derivation and distinct oracle controls, complete forward
coverage establishes the proposed delta=2^-384 separation from positive sine/cosine ppm thresholds
for the stated rational phase bound. Independent full interval recomputation is not claimed.
Native sine/cosine precision/error/workspace admission is next .c. Inverse/tangent remain open;
neither this certificate nor its maintenance timing selects their runtime algorithms.

## Native sine/cosine plan admitted — work unit0279

Choose scale S=2^512 and64 directed terms after exact first-octant reduction. This is an algorithm
budget proved against the full forward certificate, not a new language limit or precision refusal.
Keep the rational microdegree denominator in private integer workspace. For sine, reduce its
magnitude modulo360M*d, retaining the original sign; for cosine use evenness and an additional
integer90M*d shift. Fold half/quadrants, then complement above45M*d and exchange sin/cos.
Axes are exact; canonical zero has no sign. No public rational intermediate or angle binding is made.

Truncate the certified640-bit pi enclosure outward to512 bits. For the folded n/d<=45M,
x=n*pi/(180M*d) has directed scaled endpoints of width at most3 and lies in[0,1]. Its square
Z has width at most8. Each term uses T_next=T*Z/k with a single outward division by S*k:
k=(2j+2)(2j+3) for sine, (2j+1)(2j+2) for cosine. Start at X or exact1. All term endpoints
stay in[0,S]. In scaled units, width_next<=(width_T+8)/2+2<=12 whenever width_T<=12.
The64 alternating term intervals have total width at most768. Their remainder is less than
1/128!<2^-512, so one additional unit on each side gives width at most770*2^-512<2^-502.
Clipping the final interval to the known[0,1] function range preserves enclosure and this bound.

The complete forward certificate excludes every half-ppm threshold by more than2^-384 in true
sine value. The native enclosure is narrower than that separation, so true value and its lower
endpoint cannot straddle a rounding boundary. Round the lower endpoint times M exactly to an
integer ppm and apply the saved sign. This proves nearest-quantum/half-away behavior for every
admitted Angle, without a runtime convergence loop, empirical cutoff or substituted answer.

The work is fixed64 terms plus bounded integer reduction/division. All intermediate magnitudes
are below2^1026: term/square products are at most S^2, signed partial-sum magnitudes below64S+2,
and angle/pi, coefficient denominators and ppm-rounding work are smaller. Result magnitude<=M
fits the existing Ratio representation. Binding and source provenance retain their own contracts.
[DLMF4.19](https://dlmf.nist.gov/4.19) gives the series and
[DLMF4.21](https://dlmf.nist.gov/4.21) the identities; both opened2026-10-26.
The detailed [book derivation](../book/src/annexes/formula-trig-certificates.md) explains the bounds.

python3 -I -B docs/tasks/artifacts/formula_structure/sincos_runtime_plan.py passes the exact envelope,
14 direct-power/factorial Fraction enclosures,50 signed octant-table comparisons and2 insufficient
profile controls, rc=0. This is a plan/model check, not native or platform execution. Production
sin/cos code and focused oracle fixtures follow .5d.3.2.1; broader proof stays .5d.4/pre-push.
