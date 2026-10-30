# Coupled irrational function review

The root,trigonometric and inverse kernels are implemented. This chapter tracks their coupled
rounding,error and provenance review under G1-SLICE.5d.4. Each function keeps its own numerical
rounding boundary;an algebraically equivalent expression need not have the same rounded result.
Scoped fixtures do not grant platform identity,full resource bounds or production approval.

| Family | Available implementation | Coupled review boundary |
| --- | --- | --- |
| sqrt/hypot | Exact integer roots and sourced calls | Native85 rows/five actual body faults pass below;shared platform review remains |
| arc_length | Signed/full-turn fixed-pi certified rounding | Pi proof/sourced23/five actual body faults pass below;shared platform review remains |
| sin/cos | Exact folding and complete forward threshold certificate | Complete certificate/sourced88/seven actual body faults pass;shared platform remains |
| atan/atan2 | Principal branches and complete inverse threshold certificate | Source-gated proof/actual sourced81 pass below;body faults/shared platform remain |
| tan | Explicit per-call quotas,native139 and real paired24 acceptance | Coupled adaptive/error/fault review remains .5d.4.tangent |

The [canonical numerical contract](formula-rationals.md)
records the algorithms. [Tangent resources](formula-trig-certificates.md) retain
separate request admission. Original forward/inverse certificate stores are protected evidence;
this review reuses them read-only and never reruns an old installation transaction.

## Exact root boundaries

For a nonnegative radicand N/D,the native root is floor(sqrt(floor(N/D))) followed by the exact
midpoint comparison4N>=D(2k+1)^2. Python math.isqrt supplies an independent integer root algorithm;
Fraction compares the exact half-square and its two-sided neighbors. Both avoid floating point.
The existing all-admitted argument bounds area/Ratio/hypot bisection by30/74/31 iterations.
Hypot may require513/512 raw numerator/denominator bits and a576-bit midpoint product;this is
private function arithmetic,not an intermediate Area value or another language limit.

The extended root fixture preserves every byte of its original25-row prefix and adds60 rows.
All85 cases cover51 square roots and34 hypotenuses:exact ties,neighbors beyond float precision,
full128-bit Ratio components,wide positive Ratio results,negative roots,signed Length operands,
final Length-domain refusal and all four signs of a513-bit private hypot expansion.
The exact large Ratio half-square uses odd m=2^63-1 and input(m^2+delta)/256 for delta=-1,0,1.
After the Ratio scale,its root is adjacent to125m/2,well beyond signed binding capacity.
Area/Ratio neighbors at several scales include an exact rounded maximum Length;hypot with a
maximum Length and a tiny full-width second operand must not fail an intermediate Area bound.

The actual sourced watcher runs every row through real upstream cos/sin zero arithmetic,then the
actual sqrt/hypot calls. No contribution flags or computed kernel outputs are supplied. Successful
zero,perfect and rounded roots keep every executed upstream/root call contribution. T1 rejects
these contributions even when the final value is exact;actual binding/read retains the integer
and sources. All three original mathematical refusals stay typed,with their original input or
rounded final-domain witness. Five wider successful Ratio roots fail only at signed binding.

```rust
use sc_core::{name::MachineToken,recipe::{FormulaEvaluatedScalar as V,FormulaNumericValue as N,
    FormulaKind as K,FormulaRational as R,FormulaBuiltin as B}};
let input=V::from_numeric(N::new(K::Ratio,R::new(u128::MAX,1,false)?)?);
let root=input.checked_sqrt()?;
assert!(root.sources().contains(B::Sqrt));
assert!(root.numeric().expect("actual Ratio").rational().numerator_magnitude()>i64::MAX as u128);
let name=MachineToken::new("wide_root")?;
assert!(root.bind(&name).is_err());
# Ok::<(),Box<dyn std::error::Error>>(())
```

Six native tests check every numeric row,actual wrong-kind priority,the sourced/storage cases
and complete independent regeneration of6476 frozen bytes. The fixture SHA256 is
e4b701eb22add56a126b980cced076b4b694223e71bebcdfa650f17d2145765d.
The producer reads no Rust,native result or frozen expected fixture. Its explicit assertions
verify the513/512-bit witness and the original128-bit components before emitting rows.
These finite checks support the existing exact all-input argument;they are not exhaustive input
coverage. Actual implementation-body faults are recorded below.
Mixed-call chains,cross-platform execution and complete coupled closure remain separate tasks.

## Actual native root body faults

The0625 root_body_faults.py run prepared an exact542-file project-local workspace snapshot and
ran the actual sourced85-row Rust test through ordinary Cargo. The copy preserves the original
private crate scope. The unchanged baseline passes. Each of five original-body faults compiles
successfully,executes the watched test and fails an assertion containing an original independent
fixture row. Compiler failure cannot count as a rejected mathematical fault.

| Actual copied-body fault | Independent witness that rejects it |
| --- | --- |
| Half ties round toward zero | Exact9/4 Area half-square requires2 micrometres |
| Ratio square root loses its1M scale | Fractional Ratio root requires the original ppm quantum |
| Hypot loses its right coordinate | Signed3/4 Length inputs require5 micrometres |
| Hypot loses the right denominator | Original full-width fractional lengths preserve both denominators |
| Root rounds after an early quotient | Exact9/4 cannot be replaced by its integer radicand quotient |

That run verified all544 original probe/workspace inputs byte-exact. A finally block restores the copied
root body after the variants,including variant refusal. On success every copied input hash equals
its original,and the snapshot census contains only planned files. The551 output
paths include the complete snapshot,logs,report and owned timeout controls. Generated files and
compiler/test scratch stay under target through the existing local producer profile. Cargo's
reported executable identity and each variant's source/binary hashes are retained in
target/root_body_faults/0625-workspace/report.json.

Actual parent/child timeout controls also pass. A single-parent kill leaves its child able to
write the planned marker;stopping the probe's owned process group prevents that write. Native
compiler/test timeout or raised interruption stops owned descendants before root-body restoration.
This repairs the probe's own cleanup boundary;it grants no transitive compiler/locality certificate.
That complete producer review remains G1-SLICE.5b.4c.h2.b.p.o.f.n16.

After normal native checks have populated the project-local dependency store,inspect or execute:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/root_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/root_body_faults.py
```

The formula-structure probe suite also runs this check. These are native POSIX process-group
controls and finite fault witnesses. They close the scoped native root review;actual cross-platform
numerical execution remains .5d.4.platform. Forward/inverse,Tan and mixed-chain review remain
owned before the complete coupled irrational gate can close.

## Arc length certificate and sourced boundaries

The complete admitted arc argument remains A*pi/B with A=n_angle*n_radius<2^256 and
B=180000000*d_angle*d_radius>0. Zero returns canonical zero;other signs are restored after
magnitude rounding. Signed radius and complete turns stay part of the original operands.
Every half-output target maps to a rational pi target with reduced denominator below2^257.
The retained640-bit pi enclosure lies strictly inside adjacent bounded-denominator fractions
whose determinant is1 and denominator sum exceeds2^257. Any rational strictly between them
has a larger denominator. Thus no admitted target lies inside the pi enclosure,and either exact
endpoint rounds exactly as the true product. This proves the admitted family without input
sampling or a runtime precision cutoff.

The original993-byte pi certificate regenerates exactly. A private native watcher verifies the
actual compiled640-bit scale and all little-endian limbs against that regenerated artifact.
The constant producer uses Machin Fraction bounds and independent directed integer bounds
from4*(atan(1/2)+atan(1/3)). Its145 observed continued-fraction steps belong to constant generation,
not a runtime input bound. The private native workspace retains the existing898/924-bit
numerator/denominator bounds. Final rational width precedes final Length-domain admission.

The independent arc oracle instead evaluates the1/2+1/3 identity with exact Fraction sums and
requires both endpoints to agree at each original fixture. All23 original rows remain unchanged:
18 admitted integer arcs,three rounded Length-domain refusals and two exact rational-width
refusals. They include signed/full turns,both operand signs,zeros,full128-bit components and
hard rational neighbors around half-quanta and the final scalar bound.

Every row also passes through actual cos/sin zero arithmetic,then the original sourced arc call.
No flags or numerical outputs are seeded. Both upstream contributions and ArcLength survive
successful zero/signed/full-turn results,real binding and later reads. T1 refuses those actual
contributions even when the stored integer is exact. Failures retain the original typed rounded
value or exact measured width;no successful replacement value or new diagnostic is produced.

Four public native tests and the private compiled-pi watcher pass,with complete original1411-byte
arc fixture regeneration. Arc fixture SHA256:
af861fc5f0288f4389aea9eb4e6a1ce61ee4d064ef797f5eb0de0892e0eeffac.
Pi certificate SHA256:
c43023fdccac9912c0cb7adf2250492003d4c11c7a88fb3056db3b156462eb54.
Actual copied-body arc faults are recorded below. Shared platform/full resource/provider/
replay/physical and complete coupled acceptance stay separately owned;this grants no production approval.

## Actual native arc body faults

The arc_body_faults.py entry uses the shared original-scope workspace controller. Its actual
23-row sourced watcher passes unchanged. Five copied arc-body faults each compile successfully
and fail an assertion containing an original independent row:incorrect product-sign OR(row10),
discarded full turns(row6),wrong pi scale(row4),missing radius(row11) and truncation(row4).
The production body remains unchanged;no compiler failure can count as a mathematical fault.

The guarded arc run retains545 exact original inputs,an exact542-file snapshot and551 complete
planned outputs. Its report records actual Cargo binary identity,source/binary hashes and failing
fixture rows at target/arc_body_faults/0627-workspace/report.json. Original and copied inputs
match after restoration. Both owned timeout controls pass before native variants execute.

The root entry retains its original CLI and re-passes baseline/five actual faults after shared
controller reuse,including both timeout controls and all544 original input hashes. Family source,
fixture,target,watcher and output roots are selected by tracked entry configuration and admitted
before any copy/build write. The formula-structure probe suite runs both entries.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/arc_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/arc_body_faults.py
```

The primary arc target's four tests and scoped strict lint pass after adding explicit independent
row context to failure assertions. This closes scoped native arc review. Complete producer runtime/
locality remains n16;other families,mixed chains,actual platform execution and full coupled/
resource/provider/replay/physical/production acceptance keep their separate owners.

## Forward sine and cosine boundaries

For an admitted signed Angle n/d in microdegrees,the phase in turns has denominator at most
Q=360,000,000*(2^128-1). Exact period reduction,quarter-turn shifts and quadrant/octant folds
preserve this bound without binding or narrowing the authored operand. Cosine uses the exact
quarter-turn identity;negative sine restores the original sign after magnitude rounding.

The complete original certificate covers every positive half-ppm threshold,one million bands.
For each band,its outward asin/(2*pi) enclosure lies strictly between determinant-one neighbors
with denominator sum greater than Q. Any intermediate rational has denominator at least that
sum,so no admitted phase enters a band. Every true output stays more than2^-384 from a midpoint.
The independent reader rechecks all100 contiguous chunks,encoded endpoints,hashes,strict gaps
and source identities. It does not reconstruct inverse intervals:their derivation and independent
known3/Fraction4/small-denominator32/negative2 controls remain a separate proof leg. Three actual
corrupt payload controls also refuse. The original manifest and payloads remain unchanged.

The directed native series uses512 fractional bits and64 terms on the first octant. Input width
at most3 implies square width at most8 and term width at most12;the alternating sum plus tail
has width at most770*2^-512,below2^-502 and the certified2^-384 exclusion. The64-term remainder
is bounded by1/128!,which is smaller than2^-512. Clipping to[0,1] keeps the true value enclosed.
Therefore the lower endpoint and true output round to the same ppm. Exact axes return immediately;
otherwise fixed64 terms and a conservative2^1026 integer workspace bound cover every admitted
input. No precision cutoff,adaptive refusal or platform libm is introduced. The runtime-plan
instrument rechecks these inequalities,14 distinct Fraction enclosures,50 signed-octant comparisons
and two deliberately insufficient profiles. Compiled pi limbs retain the separately watched640-bit
certificate reproduced during the arc review;this review changes none of those inputs.

The original44-angle fixture remains2404 bytes with SHA-256
5ee377fefbaac4684d403f5c2a44fb00f5bf8ad3f0d873409ab13f322dffcfa1.
Its separate768-bit,128-term direct-power/factorial oracle uses a different pi identity and signed
principal-interval reduction. It regenerates every original byte and supplies88 native answers,
including huge signed turns,full-width fractions,tiny values,axes and midpoint neighbors.
The actual sourced watcher also executes all88 outcomes. A real perfect Ratio sqrt and zero atan
produce an upstream zero Angle through multiplication;adding it preserves the original full-width
Angle. Each successful sin/cos keeps Sqrt,Atan and its own call contribution,including exact zero
or unit outputs. Every actual signed Ratio binding/read matches the result and all three flags;
T1 remains refused. Four original wrong numeric kinds retain their typed actual-call/argument0
refusal even at zero. No source flags or computed values are seeded into this watcher.

The native target passes4 tests,including independent fixture reproduction. Seven actual kernel
body faults pass below;declared native/browser execution parity remains
G1-SLICE.5d.4.platform. These focused native/certificate checks grant no full integration,
allocator,physical or release approval.

## Exact native fault evidence and forward body controls

D343 exposed two misleading identities in the original shared fault report. A negative forward
row contained the entire zero and positive row strings,so substring matching reported1/10/19 for
one row1 panic. Root rows4 and10 have identical original fixture bytes;the first panic occurred at4.
Cargo JSON also contains debug_assertions,so finding the word assertion in combined output did
not establish a watched-test failure. The original0625/0627 reports and the preliminary0632
forward report remain retained;their source/binary evidence is not rewritten. Their genuine
mathematical rejections remain distinct from the corrected ordinal-reporting claim.

The independent native watchers now enumerate the unchanged fixture rows and add an explicit
fixture-row ordinal plus complete original row to numerical assertion messages. The controller
requires exactly one stderr panic from the named watched test,one marker,a bounded ordinal and
the entire row at that ordinal. It permits only the documented trailing assertion context after
a colon. Substring matches,duplicate row text,unrelated JSON,wrong-test panics,truncation and
multiple markers cannot substitute for that evidence. Each accepted report retains the exact
independent_failure payload as well as its single failed_fixture_rows ordinal.

Three direct controls cover negative,zero and duplicate-row identities. Two original substring
counterexamples reproduce the old ambiguity;thirteen unrelated or malformed transcripts refuse.
Every real probe still requires a successful Cargo build,an actual one-test execution and a
failed watched test before classifying the numerical assertion. Compiler failure never counts.
The shared timeout controls and exact original/snapshot restoration remain mandatory.

The forward entry uses the same original-scope copied-workspace controller with44 original
angles/88 actual sourced outputs. Its seven copied trig.rs faults cover cosine evenness,
period length,the cosine quarter shift,octant complementation,ppm scaling,series truncation and
final integer rounding. Independent frozen fixture values reject incorrect numerical behavior;
source contributions and actual storage/read remain part of the watched native baseline.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/forward_body_faults.py --plan-only
python3 -I -B docs/tasks/artifacts/formula_structure/forward_body_faults.py
```

All workspaces,build stores and logs remain repository-local. Fresh0632 replay identities keep
the original reports intact. An initial sandbox replay refused an owned process-group kill
before copying or compiler dispatch;the OS-visible census proved no remaining descendant.
The fresh native replay uses the access required for its POSIX timeout controls. These controls
remain finite native evidence;transitive producer/locality and browser parity stay separately owned.

The fresh runs pass three baselines and seventeen successfully compiled numerical faults.
Forward rejects cosine-sign,period,quarter-shift,octant,scale,series and rounding faults at exact
original ordinals1/4/1/1/1/7/7. Root rejects its five faults at4/9/17/24/4;arc at10/6/4/11/4.
Each report contains one explicit failure ordinal and its complete original assertion payload.
Current reports are under target/forward_body_faults/0632-verified-forward/,
target/root_body_faults/0632-exact-root/ and target/arc_body_faults/0632-exact-arc/.
Their original source populations545/544/545 and542 copied files each remain byte-exact;planned
outputs553/551/551 and both timeout controls per family pass. Original kernels/fixtures are unchanged.
The14 affected primary tests and strict lint/formatting pass after restoration,including all three
independent fixture regenerations. This closes scoped native forward review;inverse,Tan,mixed-call,
actual platform execution and complete coupled/resource/provider/release acceptance remain owned.

## Inverse principal and whole-ratio boundaries

Atan takes the real Ratio n/(1,000,000*d). Atan2 divides homogeneous Length or Ratio operands
privately:the cross products may need256 bits,so constructing a public Ratio first would invent
a refusal. Reciprocal reduction keeps the positive slope at most1;exact complements reconstruct
quadrants and the sign of y. Zero y with negative x returns+180 degrees;small negative y may
round to-180. Signed scalar Angle binding preserves that endpoint without direction modulo.

The original complete proof covers45 million half-microdegree thresholds in450 chunks. Its
minimum scaled slope gap20936389198509991007691322699578 exceeds2^104 at scale2^640.
Determinant-one neighbors exclude every admitted slope denominator up to(2^128-1)^2. On[0,1],
the atan turn derivative is greater than1/16,so the angular midpoint distance exceeds2^-540.
This slice reproduces the entire source-gated1299-byte compact witness exactly:SHA-256
6b60d154f4160b0599f0ad13731b943e3e8dbd05f29496e33a186dea2d06840b.
Frozen producer/build inputs and all critical live math sources retain their expected identities.
This reuses the original full0286 proof;it is not a fresh45-million-row scan. A deliberate full
reader rerun remains owned by G1-SLICE.5d.4.acceptance after the numerical feature set is complete.

The native profile uses576 fractional bits,five half-angle steps and64 alternating terms.
Each half-angle uses x/(1+sqrt(1+x^2)). Integer Newton on[S^2,2S^2] reaches its exact floor in at
most13 divisions;an outward upper-root correction encloses the other endpoint. Width at most11
survives each step,and x is at most1/32 after five steps. The series width is at most328 units;
its tail is below2^-645. Scaling by16/pi gives turn width below4096*2^-576=2^-564,smaller than
the certified midpoint distance. Half-away rounding of the lower magnitude endpoint is therefore
correct. Complements/signs preserve the bound;private workspace remains below2^1154,without a
new input restriction or precision refusal. Independent addition-series controls cover14 atan/
16 atan2 intervals,two zero-vector refusals,four integer-root comparisons and two weak budgets.
The observed Newton maximum11 is a finite control result;13 is the analytic all-input bound.

The original81-row fixture remains5654 bytes with SHA-256
902d696133a25bfd8277f31e6a8d86eeb75cb3ef76e4f9fff03829dfb5ae0129.
Its separate768-bit,512-term addition-reduction oracle avoids native half-angle roots and uses
a different pi identity. Every original byte regenerates. All27 atan and54 atan2 cases also run
through actual upstream sqrt/cos zero arithmetic,with both full-width operands unchanged.
Atan retains Sqrt/Atan;atan2 retains Sqrt/Cos/Atan2. Every actual Angle binding/read matches the
signed result and source flags,including zero and both180-degree endpoints;T1 remains refused.

| Original operation and inputs | Stored signed microdegrees |
| --- | --- |
| atan(0ppm) | 0 |
| atan(1,000,000ppm) | 45,000,000 |
| atan(500,000ppm) | 26,565,051 |
| atan2(0,-1) Lengths | +180,000,000 |
| atan2(-1,-1) Lengths | -135,000,000 |
| atan2(-1/(2^128-1),-1,000,000,000) Lengths | -180,000,000 |
| atan2(+1/(2^128-1),-1,000,000,000) Lengths | +180,000,000 |

Five additional actual sourced checks preserve wrong-kind/second-kind priority and both
homogeneous zero-vector refusals. A failed atan2 adds no successful call contribution to its
inputs. The complete native target passes4 tests with strict affected lint/formatting. Actual
inverse body faults remain G1-SLICE.5d.4.inverse.faults;platform,mixed-call and complete coupled/
resource/provider/physical/release acceptance remain separately owned.
