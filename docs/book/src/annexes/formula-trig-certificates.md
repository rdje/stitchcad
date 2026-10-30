# Annex: investigating bounded trig rounding certificates

Native sin/cos and atan/atan2 values are implemented under the complete certificates and admitted
plans below. Tangent has explicit-resource native preparation and certified value execution. Whole formula
execution and platform evidence remain separate [execution-plan](formula-execution-plan.md) work.

## Why output thresholds might help

Sine and cosine produce ratios rounded to integer parts per million. Their positive rounding
midpoints are (2j+1)/2,000,000 for j from0 through999,999: one million thresholds. For example,
the first midpoint is0.0000005 and the last is0.9999995. Symmetry covers negative results.

An admitted rational angle has at most128-bit numerator/denominator magnitudes in microdegrees.
Writing it in turns gives a denominator bounded by Q=360,000,000*(2^128-1), a157-bit integer.
Integer-microdegree period/quadrant/octant reductions preserve this bound. This is an input
denominator bound, not a runtime precision choice.

For each output midpoint r, enclose both inverse values asin(r-delta)/(2*pi) and
asin(r+delta)/(2*pi). If the entire interval lies strictly between a/b and c/d where
bc-ad=1, both denominators are at most Q, and b+d>Q, then it contains no admitted rational phase.
Indeed any strictly intermediate rational p/q has
q=d*(bp-aq)+b*(cq-dp)>=b+d>Q. Both integer factors are positive.
This would prove every true admitted sine stays more than delta away from that midpoint.

Once every threshold passes, a separately justified runtime sine enclosure narrower than delta
could round unambiguously. Neither the candidate interval constructor nor a small sample closes
that complete argument by itself.

## What the feasibility cohort establishes

The complete original forward feasibility derivation, command, source/endpoint receipts and controls
are retained whole in `docs/history/stitchcad-devnotes-part479.md`. That sampled pilot was preliminary;
the complete certificate and admitted runtime plan below supply the later coverage and native API.

<a id="remaining-admission-work"></a>

## Forward coverage and remaining boundaries

The full forward producer is a separate source-locked, resumable command:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py --chunks 1
python3 -I -B docs/tasks/artifacts/formula_structure/forward_threshold_certificate.py
python3 -I -B docs/tasks/artifacts/formula_structure/verify_forward_certificate.py
```

It writes100 contiguous10,000-row chunks and a manifest under target/trig_forward_certificate/0278/.
Every row retains exact phase endpoints and rational gap components. Resume verifies existing
chunks and refuses changed source hashes. The independent reader checks all gap predicates and
contiguous coverage, without importing the inverse-interval constructor. That constructor retains
its separate derivation/oracle obligations. Work unit0278 completed all1,000,000 bands, and the
independent reader passed all100 chunks, rc=0. A deliberately stopped one-chunk run was correctly
refused as complete evidence before a verified resume. This closes the forward population step;
the native error/workspace plan and implementation are recorded below. Never remove an active.lock merely to
bypass a running producer: after an interrupted process, establish that its recorded PID is gone
before removing that project-owned lock and resuming.

The complete forward certificate retains exact predicates, source identity and measured cost.
A native kernel also needs its own complete error, termination and workspace argument, followed
by focused integration evidence; sin/cos now provide those below. A failed experimental gap cannot become a new language input
restriction or a silently substituted numeric answer.

Inverse angle rounding has45 million half-microdegree thresholds in its first octant after
sign/quadrant/reciprocal symmetry. Exact atan2 slopes can require256-bit denominators in private
workspace; the complete inverse certificate below accounts for that bound. Its admitted native plan
also covers axes and signed principal branches. Tangent's much larger output threshold population
retains a separate practical resource obligation.

## Admitted sine/cosine runtime plan

Work unit0279 admits the following algorithm plan; work unit0280 implements the native value API. There is
no new dependency, input restriction or language precision limit. This closes only the forward
resource-plan obligation, using the complete certificate above.

Keep angle numerator/denominator in private integer workspace. For sine, reduce the magnitude
modulo360,000,000*d and retain its original sign. Cosine is even: reduce the magnitude, add the
integer90,000,000*d quarter turn, and reduce again. Fold at180 and90 degrees, toggling the sine
sign in the negative half. Above45 degrees, complement against90 and exchange sin/cos.
This leaves a first-octant angle without constructing a narrower public rational or binding it.
For example, sin(-450 degrees) becomes negative cos(0); cos(-180 degrees) becomes negative cos(0).
Zero and quadrant axes retain their exact values; a zero result has canonical positive sign.

At scale S=2^512, truncate the existing certified pi enclosure outward. For folded microdegrees
n/d<=45,000,000, set X to the floor/ceiling enclosure of n*pi/(180,000,000*d).
Its scaled integer width is at most3: the pi coefficient is at most1/4, with less than two units
of endpoint rounding. Since pi<4, X lies in[0,S]. The directed square Z has width at most8:
(X_high^2-X_low^2)/S<=2*(X_high-X_low), plus at most two outward rounding units.

Start the sine term at X and cosine at exact S. The next positive term interval divides the
endpoint product T*Z once by S*k, outward in each direction. Sine uses
k=(2j+2)(2j+3), cosine uses k=(2j+1)(2j+2). All endpoints stay in[0,S], and k>=2, so

```text
width_next <= (width_T + width_Z)/k + 2
           <= (12 + 8)/2 + 2 = 12.
```

This invariant covers every term. Summing64 alternating intervals gives width at most768 scaled
units. On[0,1], the alternating remainder magnitude is below the next term, hence below1/128!.
The exact factorial exceeds2^512; widening by one scaled unit on both sides is sufficient.
The resulting width is at most770*2^-512, strictly below2^-502 and the certificate's2^-384 band.
Clipping to the known nonnegative function range[0,1] preserves the true value and narrows width.
The [NIST series](https://dlmf.nist.gov/4.19) and
[addition identities](https://dlmf.nist.gov/4.21) underlie these derivations.

Every true admitted sine is more than2^-384 away from a half-ppm threshold. A valid interval
narrower than that distance cannot place a threshold between its lower endpoint and the true
value. Exact integer rounding of the lower endpoint times1,000,000 therefore yields the true
nearest ppm; restoring the sign preserves half-away symmetry. The argument applies to cosine
through its exact quarter-turn identity. There is no adaptive loop or precision-failure result.

The operation count is64 series terms plus fixed bounded reduction, products and divisions.
Term/square products are at most S^2; partial-sum magnitudes are below64S+2; coefficient,
angle/pi and ppm-rounding products are smaller than the conservative2^1026 workspace bound.
Every result magnitude is at most1,000,000ppm. This bounds private workspace without imposing
the public128-bit rational-result limit on internal reduction or series arithmetic.

The focused plan instrument is:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/sincos_runtime_plan.py
```

It checks the exact envelope inequalities and factorial,14 direct-power/factorial Fraction
enclosures,50 comparisons against a distinct signed eight-octant table, and two deliberately
insufficient profiles. Full-width rational/turn/near-axis cases are included. These support the
derivation; they are not themselves native or cross-platform evidence. The
[native API and focused fixture results](formula-rationals.md#correctly-rounded-sine-and-cosine)
are available separately. Sourced sin/cos calls are also available; whole execution and broader integration stay owned.

## Inverse certificate feasibility

The original sampled inverse derivation, exact gap bound, probe command and controls remain whole
in `docs/history/stitchcad-devnotes-part479.md`. The pilot supplied no native API or complete coverage;
those were subsequently completed and are documented below.

## Native inverse chunk probe

The original bounded diagnostic protocol, enclosure argument, parity/cost receipts and invalid-input
controls remain whole in `docs/history/stitchcad-devnotes-part479.md`. This was generator evidence;
the subsequent complete certificate, admitted plan and native API below supersede its pending status.

## Resumable inverse controller and exact reader

The controller retains450 chunks of100,000 rows, frozen build inputs/producer and source/binary hashes
under target/inverse_threshold_certificate/0284/. At most four owned children run. Resume verifies bytes; only independently
checked chunks enter its atomic manifest. Partial coverage is never a complete certificate.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py --chunks 1 --workers 1
python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py --allow-partial
python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py
```

The last command requires the full population. Preserve frozen provenance and critical live source identity.
The reader checks exact predicates; it does not independently recompute every tangent enclosure.
After interruption, establish recorded lock/child PIDs are gone before removing a project-owned lock.
Error cleanup reaps owned children; conflicting unpublished bytes are retained and refused.
Both certificate stores/payloads/frozen inputs are protected from generic artifact cleanup.
Complete original controller/probe/cleanup receipts remain in `docs/history/stitchcad-devnotes-part481.md`.

## Complete inverse certificate

Work unit0286 completes all45,000,000 targets in450 chunks and passes the default full reader,
rc=0. Every encoded strict gap, payload hash, contiguous index range and cross-chunk order is
checked. Frozen producer/build provenance and critical source identity remain exact; the lock
is absent and no owned child remains. The retained payload is12.96GB on the repository volume.

The minimum slope margin is20936389198509991007691322699578 scaled by2^640, greater than2^104.
An admitted rational slope cannot lie inside its target's gap. Its distance from the true target
tangent is therefore greater than2^-536. Throughout the folded slope range[0,1], the derivative
of atan(r)/(2*pi) is at least1/(4*pi)>1/16. The mean-value theorem gives a uniform angular
distance greater than2^-540 turns from each half-microdegree target. The
[NIST derivative](https://dlmf.nist.gov/4.24.E9) supports that last conversion.

A compact witness is tracked at crates/sc-core/tests/fixtures/inverse_rounding_certificate.tsv.
It records population, minimum, combined payload digest and exact source identities. The tracked
inverse_certificate_witness.py reproduces it from complete metadata, without replacing the full
reader. Later budget checks can use this source-gated witness instead of repeating full coverage
after every ordinary feature. The full reader remains available for deliberate re-verification.

This closes inverse population coverage. The concrete algorithm plan below uses its stronger
margin; native implementation evidence follows below. Production tangent and whole execution remain open.

## Admitted atan/atan2 runtime plan

Work unit0287 admits576 fractional bits, five half-angle steps and64 directed terms against the
verified2^-540-turn threshold margin. The plan instrument checks the compact witness's exact
profile and five source hashes. This is not a new language input limit or precision-refusal rule.

Atan's internal ppm Ratio becomes real n/(1,000,000*d). Atan2 forms y/x privately from two
homogeneous Length or Ratio operands; it never constructs a public128-bit Ratio intermediate.
The private products fit256 bits, covered by the certificate. Fold positive slopes above1 by
reciprocal, then complement the base turn against1/4. Atan restores its sign. Atan2 reconstructs
its first quadrant, complements against1/2 for negative x, then restores y's sign.

Axes use exact values. Zero y with negative x gives+180 degrees; a small negative y may round
to-180. Both-zero atan2 refuses formula_domain. The functions preserve the signed principal
answer without direction modulo, checking actual argument kinds before value special cases.

At scale S=2^576, enclose the folded rational by outward integer division. Each step applies
x/(1+sqrt(1+x^2)). For the radical's lower scaled input N_lo, integer Newton computes its
floor root r. For upper input N_hi, r+ceil((N_hi-r^2)/(2r)) is a valid upper root bound.
Squaring it proves containment. This needs one bounded Newton solve per half-angle step.

All root inputs are in[S^2,2S^2]. Newton starts at2S and decreases above the floor root.
Its error recurrence e_next<=floor(e^2/(2S))+1 reaches at most1 after11 updates; one exact
floor-root step and a stopping test give at most13 divisions. No iteration cutoff is introduced.

If the current scaled x width is at most11, its square width is at most2w+2, radical width
at mostw+2, and next x width at mostw/2+(w+2)/4+2<=11. Upper endpoints halve; after five
steps they are at most S/32. The direct alternating atan series then has initial power width11,
later powers width3, and total64-term interval width at most328 units including its tail.
The tail is smaller than(1/32)^129/129<2^-645, hence smaller than one576-bit unit.

The base turn equals16*atan(final)/pi. Its outward quotient width is below4096 scaled units,
or2^-564 turns. Exact complements and sign preserve that width. It is narrower than the certified
distance from every rounding threshold, so exact half-away rounding of its magnitude endpoint
times360,000,000 gives the true nearest microdegree. No libm, adaptive precision failure or
premature public-width refusal is involved. All intermediate magnitudes are below2^1154.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/atan_runtime_plan.py
```

This focused model passes14 atan and16 atan2 comparisons against an independent768-bit
rational-addition/series oracle, two zero-vector refusals, four standard-library integer-root
comparisons and two insufficient-budget controls. The oracle uses a distinct pi identity and
no Newton/half-angle reduction. Exact inequalities support the whole admitted argument family;
the sample checks support transcription. Native code is available below; platform evidence remains pending.

## Native inverse values

FormulaNumericValue::checked_atan() accepts a Ratio in internal ppm and returns an Angle rounded
to the nearest integer microdegree. Thus1,000,000ppm represents real1 and returns45,000,000µdeg;
500,000ppm returns26,565,051µdeg. Negative inputs retain their sign. Tiny inputs may round to
canonical zero and very large finite inputs may round to a signed90-degree endpoint.

FormulaNumericValue::checked_atan2(x) treats its receiver as y. Both operands must be Lengths
or both Ratios. Common units cancel; there is no ppm rescaling of y/x. The exact quotient stays
private even when it needs256-bit components, so admitted full-width operands do not fail at a
public Ratio intermediate. Results follow the principal branch(-180,+180] degrees before rounding.

```rust
use sc_core::recipe::{FormulaKind as K, FormulaNumericValue as V, FormulaRational as R};
let ratio = V::new(K::Ratio, R::from_signed_integer(1_000_000))?;
assert_eq!(ratio.checked_atan()?.rational(), R::from_signed_integer(45_000_000));
let y = V::new(K::Length, R::from_signed_integer(-1))?;
let x = V::new(K::Length, R::from_signed_integer(-1))?;
assert_eq!(y.checked_atan2(x)?.rational(), R::from_signed_integer(-135_000_000));
let zero = V::new(K::Length, R::ZERO)?;
assert_eq!(zero.checked_atan2(x)?.rational(), R::from_signed_integer(180_000_000));
assert_eq!(zero.checked_atan2(zero).unwrap_err().diagnostic_code(), "formula_domain");
# Ok::<(), Box<dyn std::error::Error>>(())
```

Exact axes are0,+180 or signed90 degrees. A tiny negative y with negative x may round to-180;
that signed rounded endpoint is preserved. No angle modulo changes it into+180. Both-zero inputs
return Atan2ZeroVector with the actual homogeneous kind and both exact operands, diagnostic
formula_domain. Kinds are checked first in argument order, including zero-valued arguments:
checked_atan rejects a non-Ratio with BuiltinInputKind; checked_atan2 rejects an unsupported first
kind with BuiltinInputKind and a differing second kind with BuiltinDimension. Both are
formula_dimension. No source expression or statement identity is invented at this value boundary.

The native implementation follows the576-bit/five-half-angle/64-term plan above. It uses the
shared certified pi endpoint, directed private integer arithmetic and integer Newton; no platform
libm or new precision-refusal rule is involved. Actual sourced inverse81 and signed storage/read
pass in the [coupled inverse review](formula-irrational-review.md#inverse-principal-and-whole-ratio-boundaries).
Scoped AST/source/recipe/Design scalar execution is available;complete coupled/platform proof remains.

Complete0288 native inverse/oracle/threshold-neighbor receipts remain in
`docs/history/stitchcad-devnotes-part487.md`. Coupled/platform proof remains .5d.4/integration/pre-push.

## Tangent resource boundary

Tangent uses the explicit-resource native worker below. With M=1,000,000 and U=2^128-1, an admitted nonpole rational
microdegree input is at least90M/(U+1) microdegrees from its nearest pole. This bounds rounded
ppm output magnitude to148 bits, while the public result component limit remains128 bits.
For example d=floor((U+1)/(90M)), n=90M*d-1 is admitted yet its true tangent rounds beyond U:
its correct refusal is formula_rational_width. Exact poles instead refuse formula_domain.

This magnitude bound does not resolve correct rounding. There are2^128 relevant positive
half-ppm thresholds including the final width boundary, so direct enumeration would differ
radically from the completed1M sine and45M inverse certificates. Inverse coverage concerns
half-integer microdegree targets; tangent's rational angles need their own argument.

The universal integer/elementary proof is recorded in
docs/decisions/fact_trig-midpoints-and-resource-bound.md. Its focused tangent_resource_profile.py
checks exact width/pole witnesses,8798 independently enumerated nonpole distances,586 exact
poles and2 negative controls. These support transcription; no practical runtime precision or
latency bound is claimed by this range profile. The bounded-request strategy below now supplies
logical arithmetic admission;the native kernel remains separate.

## Exact tangent folding and quotient conditioning

A prerequisite model now reduces each actual admitted rational Angle through exact integer period,
reflection and complement operations. It handles original odd-quarter poles,canonical zero and
exact signed unit results separately. Other magnitudes use unrounded sin/cos in the first octant,
with reciprocal selection near a pole;the sign is restored only after true-value ppm rounding.
Authored signed/full-turn Angle state is unchanged. This model alone supplies no production tangent value API.

With A=90,000,000 and input magnitude n/d,set q=A*d and r=n mod(2q). Reflect r>q to2q-r and
flip the result sign. Exact r=q is a pole;exact r=0 is zero. For2r>q,complement to q-r and
select cos/sin;otherwise select sin/cos. Exact2r=q gives magnitude1,000,000 ppm. Modulo never
increases n,and each selected reflection/complement decreases r. The private folded numerator
and original denominator therefore remain at most128 bits;the period needs at most156 bits.
These are our bounds derived from the [standard addition identities](https://dlmf.nist.gov/4.21).

The direct quotient's cosine denominator is greater than1/2. For reciprocal mode,the earlier
nearest-pole distance bound and sin(x)>=2*x/pi imply its sine denominator is at least2^-128.
A rigorous enclosure can be tightened by that analytic floor. Given component widths epsN,epsD
and denominator floor L>0,the outward quotient interval[Nlo/Dhi,Nhi/Dlo] has width at most
epsN/L+epsD/L^2. Near-pole uncertainty can therefore amplify by2^128 and2^256;ordinary ppm-rounded
sine/cosine values cannot supply tangent rounding. At30 degrees,dividing rounded500000/866025
instead rounds577351 ppm,whereas exact square bounds prove the correct tangent rounds577350.

The tracked tangent_reduction_plan.py checks29928 finite/372 pole finite-model cases against
independent signed Fraction reduction,including3642 reciprocal cases,and92 actual full-width/
signed witnesses. It also checks144 quotient rectangles and two wrong-composition/sign controls.
Universal identities/algebra supply the argument;finite controls check its transcription. The
producer is registered in the existing probe suite and exact source/file inventories. Full
rounding separation is not inferred from these finite checks. Convergent enclosures and selected
request-resource admission follow below;the native kernel and full runtime readiness remain separate.

## Convergent unrounded tangent enclosures

A pure diagnostic model now supplies a rigorous enclosure at each explicit precision B>=128.
It recomputes pi through directed Machin/arctangent series,then uses the exact reduction above,
unrounded sine/cosine series and the proved positive denominator floors. It does not divide the
public rounded sine/cosine results or reuse a640-bit pi certificate at greater precision.
The [arctangent series](https://dlmf.nist.gov/4.24.E3) and
[sine/cosine series](https://dlmf.nist.gov/4.19) support the identities;the following error bounds
are this project's derivation.

At scale2^B,each B-term arctangent interval has width at most B+1;the Machin pi combination has
width at most20(B+1). Exact radian conversion has width at most5B+7,and its square has width at
most10B+16. The sine/cosine term recurrence preserves width at most10B+20. B alternating terms
plus outward tails therefore enclose each component within(10B^2+20B+2)*2^-B. The tails are
bounded by(2B)!>2^B. These widths shrink to zero;the denominator floor gives a uniformly shrinking
quotient enclosure even near an admitted pole.

A diagnostic result is certified only when both closed ppm endpoints round to the same integer.
An interval wholly above the final U+1/2 boundary instead proves actual rational-width excess;
exact poles are classified separately. Otherwise the status stays undecided. The midpoint theorem
above proves eventual separation under increasing precision. It does not supply a practical uniform
precision bound. One iteration uses at most4B series terms and a fixed number of candidate integer
slots,with explicit operands bounded by16B+bit_length(2B+1)+16 bits. These mathematical extents
supply neither allocator-byte nor latency guarantees,or a production runtime admission.

Independent controls compare three alternate-identity Fraction pi bounds,42 direct-power/factorial
component enclosures and54 coupled endpoint/pole/width cases. A separately constructed legal Angle
247385123034593398372138492992328993860/8635369834810561370044731507908276509 microdegrees
remains undecided in this model at128 and256 bits. At512 bits its whole interval certifies zero,
matching independent arctangent/alternate-pi bounds below the actual half-ppm threshold. This does
not prove that every algorithm needs512 bits,or that512 suffices for all inputs. The tracked pure
model runs in the existing probe suite. The bounded-request strategy below now selects logical
resource admission. Production tangent is still unavailable.

## Selected bounded adaptive tangent admission

The selected strategy uses explicit caller limits for the largest candidate integer and cumulative
charged series terms. There is no default policy or fixed maximum precision. An insufficient
execution budget produces a separate resource refusal carrying the original operand,selected
limits,unit,maximum,prior reservation/work and next scheduled demand. It supplies no approximate
value or formula-language precision/domain diagnostic. Valid mathematical inputs stay valid.
Native preparation/value/source execution follows below;Design policies are available;scoped combined24-row acceptance passes.

After the existing actual-kind check,reserve the complete156-bit folding extent. Exact zero,
signed unit and pole cases need no approximation terms. Other cases start at B=128 to represent the2^-128 denominator floor,and double
only after an undecided enclosure. Before each iteration,check I(B)=16B+bit_length(2B+1)+16
candidate bits,then4B additional series terms. Charge them only after both complete demands fit.
The bit counter retains the highest admitted reservation;the work counter retains cumulative
charged terms. Neither measures physical heap bytes or wall-clock duration.

Every finite selected request terminates:increasing reservations or positive charged work must
exhaust a finite quota unless a certificate completes first. Actual charged work stays within the
selected limit,and the proved integer extents stay within the admitted candidate reservation.
Native extent/counter overflow must refuse before allocation. Successful values are the same
under every sufficient policy;resource selections may change whether an execution is admitted.
The convergence/midpoint proof gives eventual mathematical certification under sufficiently
increased resources,without claiming one practical uniform precision or budget for all inputs.

For the original near-midpoint witness,128/256/512-bit iterations charge512+1024+2048=3584
terms. The largest admitted integer reservation is8219 bits. Those explicit limits certify zero.
A4122-bit limit instead refuses the next8219-bit reservation after1536 charged terms. With8219
bits but only1536 terms,the next2048-term reservation refuses. These are next scheduled costs,
not minimum mathematical requirements. No accepted value/source/recipe prefix escapes refusal.

The tracked tangent_budget_plan.py passes19 exact cases covering endpoint/pole/width outcomes,
early/later bit/work refusals,priority,original operand/policy identity,counters,successful-value invariance and absence of a
value on refusal. Three actual in-memory guard omissions fail their intended controls;source
bytes remain unchanged. Expected near-midpoint rounding comes from the independent inverse
oracle. The decision is recorded in docs/decisions/decision_tangent-resources.md. It licenses
native implementation of this bounded strategy,with explicit limits and rich resource errors;
this model alone grants no native,full allocator/CPU,platform or release approval certificate.

## Native tangent preparation with explicit quotas

FormulaNumericValue::prepare_tangent accepts an actual completed Angle and an explicit
FormulaTangentLimits. FormulaTangentPlan privately retains that original operand,its exact folded
metadata and the selected policy. It cannot be assembled from caller-supplied reduction data.
Preparation validates the actual kind,reserves folding space,classifies exact cases and admits the
complete initial approximation demand when needed. It runs no pi or component series and supplies
no numeric result. The native value worker below consumes this actual prepared plan.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational, FormulaTangentLimits};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let angle = FormulaNumericValue::new(
    FormulaKind::Angle, FormulaRational::from_signed_integer(30_000_000),
)?;
let limits = FormulaTangentLimits::new(2073, 512);
let prepared = angle.prepare_tangent(limits)?;
assert_eq!(prepared.input(), angle);
assert_eq!(prepared.limits(), limits);
assert_eq!(prepared.initial_integer_bits(), 2073);
assert_eq!(prepared.initial_series_terms(), 512);
# Ok(())
# }
```

The initial term count is admitted planned work,not charged work:preparation evaluates no series.
Exact zero and signed unit inputs need only limits156/0;an exact pole with those limits returns
FormulaTangentError::Pole with the original complete signed Angle. Even a zero or pole first
refuses a155-bit folding quota. The actual-kind error precedes all resource checks:passing a Ratio
with limits0/0 returns the original Tan dimension error,not a resource refusal.

For a nontrivial Angle,limits2072/0 refuse2073 integer bits before considering series work.
Limits2073/511 instead refuse512 additional terms. FormulaTangentResource selects IntegerBits or
SeriesTerms. FormulaTangentResourceError retains the original rational,selected limits,refused
unit and maximum,next scheduled demand,prospective precision128,prior folding reservation156 and
zero charged terms. A folding refusal has no prospective precision and zero prior reservations.
The whole first iteration is admitted atomically:neither its reservation nor term charge is recorded
as completed when either quota fails. Increasing limits admits preparation without changing input.

FormulaTangentError forwards an existing numerical cause or the exact pole domain family through
its optional diagnostic_code. Resources return None;the error chain retains the original resource
payload. No numerical width error,Unknown,approximate value or source contribution replaces a
resource refusal. There is no default policy. Logical bit/work quotas do not certify complete
allocator bytes,CPU latency,source execution or production readiness.

Focused native checks cover actual kind priority,folding-before-endpoint/pole priority,inclusive
quotas,atomic first-iteration admission and unchanged full128-bit operands. A private test compares
actual folded signs,reciprocal choice and exact cases with873 independent signed centered residues.
Independent combined native value/resource acceptance is recorded below;
Expression/source/recipe policies are available;Design policies are available;scoped combined24-row acceptance passes.

## Certified native tangent value execution

FormulaTangentPlan::evaluate consumes the actual prepared operand and its explicit limits. It
recomputes outward pi at every admitted precision,then both unrounded sine/cosine intervals and
the outward direct or reciprocal quotient. Whole closed-interval rounding agreement alone accepts
a Ratio in integer parts per million. Exact zero/unit cases use their admitted folding result and
charge no series. The plan never re-reads an authored provider or mutates canonical Angle state.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational, FormulaTangentLimits};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let input = FormulaNumericValue::new(
    FormulaKind::Angle, FormulaRational::from_signed_integer(-60_000_000),
)?;
let result = input.prepare_tangent(FormulaTangentLimits::new(2073, 512))?.evaluate()?;
assert_eq!(result.value().rational(), FormulaRational::from_signed_integer(-1_732_051));
assert_eq!(result.plan().input(), input);
assert_eq!(result.precision(), Some(128));
assert_eq!(result.reserved_integer_bits(), 2073);
assert_eq!(result.charged_series_terms(), 512);
# Ok(())
# }
```

FormulaExecutedTangent retains the original plan and explicit policy beside its certified value,
precision,highest admitted reservation and cumulative charged terms. Exact results have no
approximation precision. These are execution metadata,not authored model fields or binding output.
FormulaTangentExecutionError retains that same original plan,progress and typed mathematical or
resource cause. Its error chain forwards the actual cause;resource admission has no language
diagnostic. No uncertain value or partial numerical result is exposed through the failure API.

At B=128 and each doubling,the worker computes complete integer/work demands with wider native
extent bookkeeping,checks native counter/factor conversions,and admits both quotas before
allocating the iteration's integers or charging work. Only the current interval state is retained;
there is no growing iteration history or hard-coded precision ceiling. A larger sufficient policy
must yield the same certified value. Logical integer/work limits still grant no total heap-byte or
wall-clock certificate. Full provider,Design and platform envelopes retain their existing owners.

The native rational-width cause requires the exact bit count of the rounded integer. Merely
proving that the entire result exceeds128 bits cannot invent that count. For the actual near-pole
width witness above,the B128 rounded endpoints have142 and148 bits. The worker refines;B256
proves148 at both ends and returns the original numerator RationalWidth cause. Limits2073/512
instead return the next4122-bit resource demand after512 terms. Limits4122/1536 certify the
148-bit width error. The diagnostic model's earlier width classification carried no bit count;
its smaller classification budget cannot license a false native measured_bits field.

Five focused worker tests cover exact endpoints,signed quadrants and reciprocals,the independently
bounded tan30 rounding counterexample,tiny full128-bit inputs,actual512-bit midpoint refinement,
later bit/work/priority refusals,policy-independent successful values and exact148-bit width.
The earlier preparation/folding contracts remain separate. Source-bearing supplied-value calls
are implemented below,followed by independent combined native acceptance. Expression/source/recipe policies are available;
Design policies are available;scoped combined24-row acceptance passes. No full CI,platform,physical or production approval
follows these scoped worker checks.

## Sourced native tangent calls

FormulaEvaluatedScalar::checked_tan takes an actual supplied sourced operand and explicit
FormulaTangentLimits. It invokes the native preparation and worker itself;there is no caller-supplied
numerical answer or contribution set. FormulaEvaluatedTangent retains that exact original input
and FormulaExecutedTangent. Its value getter returns the certified Ratio with every input
contribution plus the successful tan call. Exact zero/unit cases still contribute tan.

```rust
use sc_core::recipe::{FormulaBuiltin as B, FormulaEvaluatedScalar as V, FormulaKind as K,
    FormulaNumericValue as N, FormulaRational as R, FormulaTangentLimits, FormulaToleranceName as T};
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let angle = V::from_numeric(N::new(K::Angle, R::ZERO)?);
let result = angle.checked_tan(FormulaTangentLimits::new(156, 0))?;
let value = result.value();
assert!(value.sources().contains(B::Tan));
assert!(value.sources().check_tolerance(T::Numerical).is_err());
value.sources().check_tolerance(T::Geometric)?;
assert_eq!(result.execution().charged_series_terms(), 0);
# Ok(())
# }
```

The T2 check above admits only the symbolic source guard. It supplies no quantitative tolerance,
provider,statement or physical certificate. Actual cancellation,comparison,binding and read use
the existing scalar methods;tan survives even when the resulting stored Boolean is false or a
numeric result is zero. No class or storage boundary erases a successful dependency.

FormulaEvaluatedTangentError retains the actual original supplied operand,its existing source
set,the explicit limits and attempted Tan operation. It exposes the original preparation error
or borrows the original native execution error with its plan and progress. The standard error
chain points to that original cause. Wrong-kind input,including Boolean false,precedes resources;
folding refusal precedes actual pole classification. Later quota and exact-width causes retain
all already executed input contributions. Resource refusal still has no language diagnostic.

A failed call supplies no successful output and adds no successful tan contribution. Attempted
operation metadata is distinct from a completed-result source set. If the input already carried
a prior successful tan call,that original contribution remains on the retained input. This follows
the existing successful-call source contract;failure does not fabricate or erase an input value.

Five focused tests construct real upstream round_to and cos calls,without manually attaching
sources. They check exact zero/unit Tan contributions through cancellation,false storage/read and
T1 refusal/T2 source admission;actual negative tan30;Boolean/prior-tan kind priority;original
fold/pole failures;and late native resource/148-bit width failures with exact original source-chain
identity. Combined independent native acceptance is recorded below. Expression/source/recipe/Design policies are available;scoped
combined24-row acceptance passes.

## Independent native tangent acceptance

The tracked tangent_value_oracle.py derives139 fixture rows using exact standard-library
Fraction arithmetic. It centers signed rational angles directly,uses the different identity
pi=4*(atan(1/2)+atan(1/3)),and evaluates direct powers/factorials at exact rational interval
endpoints. Its monotone sine/cosine bounds do not reuse native Machin or fixed-point recurrences.
It reads no Rust implementation,native output or frozen expected fixture. An unresolved selected
fixture fails the producer;its offline term counts impose no native mathematical input limit.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/tangent_value_oracle.py --check
python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test formula_tangent_oracle_contract
```

The population has125 certified values,10 exact poles and four exact numerator-width failures.
It includes signed quadrants and complete turns,full128-bit components,near-pole inputs on both
sides,tiny fractions,finite Ratio values beyond signed binding range,and actual neighboring
inputs on both sides of the selected half-ppm boundary. Exact sqrt(3) midpoint inequalities also
establish tan30's577350 ppm and distinguish the wrong577351 rounded-component composition.

Two native watchers use the actual public preparation/worker and supplied-scalar wrapper under
two explicit policies:16412 candidate bits/7680 series terms and70000/40000. Every outcome must
match the independent reference. They check original operands/policies,Ratio payloads,successful
Tan contributions,typed poles,exact width counts and native progress retained by the wrapper.
They separately regenerate every one of7141 frozen bytes. The fixture SHA256 is
cf1a803c1efd405dcd84d6af73340b675824c979b793bfbb6112862218a39d7b.

All18 selected native tests pass across preparation,value,source-wrapper,independent-oracle and
private admission/folding targets. This includes873 independent signed residue folds and the
focused early/later resource-priority and counter controls. Strict affected-target lint and
formatting pass. The pure oracle is registered in the existing formula probe suite;its source
has explicit read-only/stdout inventory ownership. Full producer locality review remains separate.

This closes scoped native tangent implementation and finite combined acceptance. The rigorous
admitted strategy supplies result certification;these139 rows are not exhaustive128-bit input or
cross-platform coverage. The coupled irrational review remains .5d.4. Expression/source/
recipe/Design policies are available;scoped combined24-row acceptance passes. Full heap/CPU,
provider,platform,replay,physical and production approval retain their separate owners.
