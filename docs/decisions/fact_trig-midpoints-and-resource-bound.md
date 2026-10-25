# Trig midpoint exclusion establishes qualitative termination; practical resources remain open

- **Type:** `fact`
- **Date:** `2026-10-25`
- **Status:** `active`; no production trig algorithm admitted
- **Owner / source:** G1-SLICE.5d.1.2.2.1; resource strategy .5d.1.2.2.2 is P1 before .5d.3.2/.3.

answers: "can trigonometric formula results land exactly on a rounding midpoint?" · "does convergence prove usable trig resources?"

## Scope and independent theorem

The normative formula-language contract admits rational microdegree operands with numerator and
denominator magnitudes at most128 bits. Sin/cos/tan produce nearest ppm; atan/atan2 produce nearest
microdegrees. Exact odd-quarter-turn tangent poles and atan2(0,0) refuse. This record changes no
domain, branch, tolerance or precision limit, and supplies no implementation/latency evidence.

[Calcut, Rationality and the Tangent Function, Corollary5](https://www2.oberlin.edu/faculty/jcalcut/tanpap.pdf)
and [Nunn's independently published proof](https://scholar.rose-hulman.edu/rhumj/vol22/iss2/3/)
establish the rational-value restriction at rational multiples of pi: sine/cosine can be rational
only at0, ±1/2, ±1; finite rational tangent values can only be0, ±1. These are theorem sources,
not numerical samples. NIST supplies [principal inverse branches](https://dlmf.nist.gov/4.23)
and [arctangent series/identities](https://dlmf.nist.gov/4.24); it does not certify StitchCAD's
unimplemented interval error accounting or resource bounds. Sources actually opened2026-10-25.

## Consequence for the actual language quanta

This application of the theorem is our derivation. Let M=1,000,000. A half-output target for
sin/cos/tan has real value (2k+1)/(2M). It is rational. Every permitted rational value listed above,
multiplied by M, is an integer, so none can be a half integer. Hence an admitted finite forward
trig result cannot equal any rounding midpoint. This depends on the actual quantum: at M=1,
sin(30 degrees)=1/2 would be an exact tie. It is not a scale-independent fact.

For atan the input ratio is rational. For atan2 with x!=0, the exact slope y/x is rational even
when forming it in private workspace would exceed the public128-bit rational result bound. If a
true inverse answer were a half microdegree, its degree measure would be rational and its tangent
would be rational. The theorem restricts it to0 or ±1. On the specified principal branches the
corresponding angles are integer multiples of45 degrees, all integer microdegrees. Axes at ±90
and the negative x axis at +180 are integers too, not midpoints. Canonical zero y with x<0
selects +180; negative nonzero y may round to -180. Both-zero still refuses. No modulo is added.

Consequently, after exact pole/axis/branch handling, rigorous enclosures whose widths tend to zero
will eventually fit within one rounding cell for each admitted finite input. This is a qualitative
termination argument. It is conditional on correct converging enclosure arithmetic, not a proof of
an implementation that does not exist. The half-away contract remains in force for the language.

## Why a generic separation estimate is insufficient for admission

A constructive coarse bound illustrates the resource gap for sin/cos alone. Define integer
polynomials C0(X)=2, C1(X)=X, C(n+1)(X)=X*C(n)(X)-C(n-1)(X). For n>=1 these are monic of degree n,
and Cn(2cos(t))=2cos(nt), by the cosine addition identity. If t=p*pi/q with integers p,q>0,
P(X)=C(2q)(X)-2 is monic and P(2cos(t))=0. The numerator may also be zero or negative; the same
identity holds, with positive q. A rational root of a monic integer polynomial is an integer:
substituting coprime a/b and multiplying by b^n forces b to divide a^n, hence b=1.

At a ppm midpoint r, the target X=2r=(2k+1)/M is noninteger because M is even. Thus P(X)!=0.
Its reduced denominator D<=M gives abs(P(X))>=D^(-N)>=M^(-N), where N=2q. On [-2,2],
abs(CN'(X))<=N^2: at X=2cos(t) the derivative is N*sin(Nt)/sin(t), and
abs(sin(Nt))<=N*abs(sin(t)); the endpoint limits obey the same bound. The latter inequality follows
by induction from sine addition and abs(cos)<=1. The mean-value theorem therefore gives
abs(cos(t)-r)>=1/(2*M^N*N^2) for midpoints inside [-1,1]. Outside midpoints have distance at least1/(2M) from
sine/cosine. Sine uses cos(pi/2-t), increasing the angle denominator by at most a factor2.

An admitted microdegree denominator d<=2^128-1 gives q<=180,000,000*d<2^156. A uniform bound
covering both functions has N<=4*180,000,000*(2^128-1)<2^158. Since M<2^20, an interval width
below 2^(-(20*N+318)) is sufficient to separate its true value from these midpoints. The exact
arithmetic producer checks the derived worst-case sufficient-precision ceiling is below2^163 bits.
This estimate is far too large to serve as a usable implementation budget. It is an upper bound
from a coarse proof, not a lower bound on required work or evidence of any hard actual input.
It neither proves impossibility nor licenses an arbitrary smaller cutoff, and does not cover all
tangent/inverse resource details. Ordinary finite precision or host libm is still no substitute.

## Evidence and remaining owner

Run python3 -I -B docs/tasks/artifacts/formula_structure/trig_midpoint_certificate.py. It checks
the exact quantum substitutions, the scale1 falsifier, the width-bound arithmetic, and compares
161 recurrence evaluations against an independent binomial expansion plus three known roots.
These finite checks support the algebraic derivation; the cited theorem supplies completeness.
They do not constitute an all-input runtime/performance test. No new dependency is selected.

G1-SLICE.5d.1.2.2.2 owns a practical, justified interval/termination/resource strategy before
production sin/cos/tan/atan/atan2. A sharper certificate or validated adaptive method must meet the
existing admitted domains and refusal contracts; changing them would require its own decision.
Root/arc features and independent evaluator-adapter work can proceed while that obligation is open.
