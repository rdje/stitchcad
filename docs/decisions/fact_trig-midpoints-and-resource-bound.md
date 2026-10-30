# Trig midpoint exclusion,convergent enclosures and bounded tangent admission

- **Type:** `fact`
- **Date:** `2026-10-25`
- **Status:** `active`; sin/cos and inverses implemented;tangent bounded-request plan selected,kernel pending
- **Owner / source:** G1-SLICE.5d.1.2.2.1;bounded tangent strategy .5d.1.2.2.2.d.2 selected;native values0616;source calls0617;scoped139-row native acceptance0618;AST/platform pending.

answers: "can trigonometric formula results land exactly on a rounding midpoint?" · "does convergence prove usable trig resources?" · "what bounds tangent folding and quotient conditioning?" · "how do unrounded tangent enclosures converge?"

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
interval error accounting or resource bounds. Sources opened2026-10-25;Calcut tangent Corollary1 rechecked2026-10-30.

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

G1-SLICE.5d.1.2.2.2 owns each practical interval/termination/resource strategy before its kernel.
Sin/cos and inverse plans/kernels are implemented;the bounded tangent strategy is selected in decision_tangent-resources.md. It preserves the
existing numerical domains/refusals;explicit execution resources remain separate from language diagnostics.
Native tangent implementation and full-family evaluator/resource integration remain separately owned.

## Tangent range and certificate boundary0290

Set A=90M microdegrees, U=2^128-1. For an admitted magnitude n/d, choose its nearest pole
p=A*(2j+1)>0. If it is not a pole, m=abs(n-p*d)>=1 and distance delta=m/d<=A.
Since p*d<=n+m<=U+m, delta>=p*m/(U+m)>=p/(U+1)>=A/(U+1).
This uses both public component bounds; an angle A-1/U would have numerator A*U-1>U and is
not an admitted worst-case witness. Signs and whole periods do not weaken this integer proof.

Let t=pi*delta/(2A). For0<t<pi/2, tan(t)>=t by its derivative>=1, so
abs(tan(angle))=cot(t)<=1/t<2*(U+1)/3, using pi>3. At t=pi/2 the result is zero.
Thus its rounded integer ppm magnitude is below ceil(2M*(U+1)/3)+1, a148-bit bound.
This bounds output magnitude; it does not bound the precision needed to decide rounding.

Finite admitted tangent inputs can legitimately exceed the reduced128-bit result limit. Choose
d=floor((U+1)/A), n=A*d-1: both components are admitted and delta=1/d. Here
t=pi/(2A*d)<2/(A*d)<1/2. The elementary series bounds sin(t)<=t and cos(t)>=1/2 give
M*cot(t)>A*M*d/4>U+1/2, so the rounded result requires formula_rational_width, not formula_domain.
Exact poles retain their distinct formula_domain refusal; this adds no language restriction.

Representable positive-output midpoint levels k+1/2 cover k=0,...,U, including the final width
boundary U+1/2:2^128 targets. Their number differs fundamentally from the1M sine thresholds.
The45M inverse certificate excludes rational slopes near half-integer microdegree angles; tangent
accepts arbitrary admitted rational microdegrees, such as1/3. It cannot directly supply their
rounding certificate. This explains why simply repeating the existing finite population method
does not produce a practical tangent plan. It is not an impossibility proof for sharper methods.

python3 -I -B docs/tasks/artifacts/formula_structure/tangent_resource_profile.py verifies these
exact bounds/witnesses,8798 finite-model nonpole distances against independent direct pole
enumeration,586 exact poles and2 negative controls, rc=0. The algebra/elementary inequalities
provide the universal derivation; the small enumeration supports transcription only.
The bounded-request decision now selects that logical strategy;native .5d.3.2.2 remains pending.
This historical range profile alone still grants no numerical kernel or full execution certificate.

## Exact tangent reduction and conditioning0612

This is a prerequisite derivation,not admission of a rounding algorithm or production kernel.
The [NIST addition identities](https://dlmf.nist.gov/4.21),opened2026-10-30,support the exact
period/sign/complement identities. The component and conditioning bounds below are our derivation.
Keep A=90M,U=2^128-1 and the original admitted signed magnitude n/d,0<=n<=U,1<=d<=U.
Set q=A*d,r=n mod(2q),negative=original sign. If r>q,replace r by2q-r and flip negative.
Exactly r=q is an original odd-quarter pole;exactly r=0 is canonical zero. If2r>q,replace r by
q-r and select reciprocal;otherwise select direct. The remaining r/d lies in(0,A/2]. At2r=q
return the exact magnitude M;otherwise evaluate unrounded sin(x)/cos(x) or cos(x)/sin(x),
x=pi*r/(2*A*d),then round its ppm magnitude once and restore the selected sign.

Modulo gives r<=n. Reflection2q-r decreases r when r>q;complement q-r decreases it when2r>q.
Thus the folded numerator never exceeds the original n and the unchanged denominator remains
admitted128-bit data. The private period2*A*d needs at most156 bits. No authored angle is
normalized or stored,and no intermediate public128-bit rational or binding is manufactured.

For direct mode,cos(x)>=cos(pi/4)>1/2. In reciprocal mode r/d is the original nearest-pole
distance,so the proven0290 bound gives r/d>=A/(U+1). Concavity of sin on[0,pi/2] places it
above the chord2*x/pi. Hence sin(x)>=r/(A*d)>=1/(U+1)=2^-128. This is a strictly positive
analytic denominator floor over every admitted finite input,not an empirical sine approximation.
A valid enclosure may intersect this known lower bound without inventing a denominator value.

For numerator/denominator intervals[Nlo,Nhi],[Dlo,Dhi],0<=Nlo<=Nhi<=1 and Dlo>=L>0,the
outward quotient is[Nlo/Dhi,Nhi/Dlo]. Its width equals epsN/Dlo+Nlo*epsD/(Dlo*Dhi),so is
at most epsN/L+epsD/L^2. Direct mode permits L=1/2;reciprocal mode permits L=2^-128.
The latter can amplify absolute component uncertainty by factors2^128 and2^256. This is no
rounding-midpoint separation bound;the selected bounded-request strategy is in decision_tangent-resources.md.

Dividing already ppm-rounded sine/cosine is unsound as a tangent implementation,even at30 degrees. Exact square inequalities
prove cos30 rounds866025,tan30 rounds577350 and sin30 is exactly500000 ppm. The quotient of
these rounded components instead rounds577351. Positive120 degrees also requires a sign flip,
so retaining only the original input sign is incorrect. Exact poles still require formula_domain;
the existing finite nonpole overflow witness still requires formula_rational_width,not a new domain.

Run python3 -I -B docs/tasks/artifacts/formula_structure/tangent_reduction_plan.py. It compares
29928 finite/372 pole cases against independent signed Fraction period reduction,including3642
reciprocal cases,plus92 actual full-width/signed witnesses. It verifies144 rational quotient
rectangles and two wrong-composition/sign controls,rc=0. These support transcription;the identities
and algebra above supply the universal argument. The producer is registered in existing probes
and exact producer/file inventories. No latency,all-input precision or runtime-resource admission
is inferred from this prerequisite. The bounded-request decision now selects the logical strategy;native implementation remains pending.

## Convergent per-precision tangent model0613

This is a diagnostic model,not production resource admission. For integer precision B>=128,set
S=2^B. [NIST sine/cosine series](https://dlmf.nist.gov/4.19) and
[arctangent series](https://dlmf.nist.gov/4.24.E3),opened2026-10-30,support the identities;
error/width/iteration bounds here are our derivation. No fixed640-bit pi certificate is extended.
Each directed B-term atan(1/5),atan(1/239) sum has width<=B+1 scaled units:one unit per rounded
term and at most one alternating tail unit. Machin combination16a-4b gives fresh pi width
<=20(B+1). Intersect with3<=pi<=4. Exact first-octant conversion gives radian width<=5B+7.
Squaring outward gives width<=10B+16. Each sine/cosine term width is at most10B+20:the next
width is <=(old+square_width)/2+2,which preserves that bound. Magnitudes stay<=S. B alternating
terms plus a one-unit tail on either side give component width<=10B^2+20B+2 scaled units.
The tail bound follows from(2B)!>2^B and x<=1. Clip component enclosures to[0,1].

With eps=(10B^2+20B+2)*2^-B,the prior analytic denominator floors give quotient width at most
M*eps*(2^128+2^256),which tends to zero. Original rational-angle tangent midpoint exclusion
then proves eventual one-cell rounding for every admitted nonpole if precision is increased.
Both closed endpoints must round alike before a diagnostic rounded value is certified;otherwise
it remains undecided. An entire interval above U+1/2 instead certifies actual rational-width
excess. Exact poles remain separate. There is no substituted result or precision-domain refusal.

One nontrivial precision uses at most4B series terms and a fixed number of candidate integer
slots. Each explicit candidate arithmetic operand is bounded by16B+bit_length(2B+1)+16 bits:239<2^8
bounds the largest atan power/denominator;term products need at most2B+1 bits and original
angle conversion needs B+130/156 bits. The model checks these invariants. This is mathematical
integer extent/per-iteration work,not allocator bytes,latency or an applicable full runtime budget.

The registered tangent_reduction_plan.py passes three independent alternate-identity Fraction pi
bounds,42 direct-power/factorial Fraction component controls and54 coupled exact endpoint/pole/
width cases. Its independently constructed admitted near-midpoint Angle is
247385123034593398372138492992328993860/8635369834810561370044731507908276509 microdegrees.
At128/256 bits this model correctly remains undecided;512 certifies zero. Independent arctangent/
alternate-pi bounds place the original angle below the true half-ppm threshold,rc=0. This is no
lower bound on another algorithm's required precision or a universal512-bit guarantee.
Bounded-request admission is selected in decision_tangent-resources.md;native tangent values0616;source calls0617;scoped139-row native acceptance0618;AST/platform remain pending.
