# Annex: investigating bounded trig rounding certificates

The sin/cos value kernel is implemented under the certificate and admitted plan below;
tan and atan/atan2 remain unimplemented. Whole formula execution is still pending. The plan owner is
G1-SLICE.5d.1.2.2.2; full execution also retains the other dependencies in the
[execution plan](formula-execution-plan.md).

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

The tracked Python producer uses directed512-bit integer intervals, the existing certified640-bit
pi enclosure, integer square roots, half-angle identities and an alternating arctangent series.
The candidate value band is delta=2^-384. These are experimental certificate parameters.
The independent oracle uses exact Fraction arithmetic and a different, positive arcsine series.
The series are documented by [NIST DLMF4.24](https://dlmf.nist.gov/4.24).

Run from the repository root:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/finite_threshold_feasibility.py
```

Work unit0277 certified38 sampled forward bands. Controls cover three exact inverse values,
four independent series enclosures, exhaustive rational checks at32 small denominator bounds,
and two required refusals. One refusal deliberately places the real sin30-degree value1/2
inside a band; another uses a band wide enough to contain admitted inputs.
The report explicitly says feasibility_cohort_only and retains exact endpoints and source hashes
under target/trig_threshold_feasibility/0277/. It does not claim the full million thresholds.

## Remaining admission work

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
workspace; a tangent-of-angle-band certificate must account for that bound. Its bounded generator
cohort is available below; complete inverse coverage remains open. Axes, poles and
signed principal branches still need their existing exact rules. Tangent's much larger output
threshold population retains a separate practical resource obligation.

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

The inverse experiment encloses tangent over a2^-600-turn band on either side of each
half-microdegree target, using640-bit directed sine/cosine intervals and their quotient.
The candidate uses64 terms with the certified pi input; its positive cosine denominator and
alternating tail are checked. Strict rational-gap predicates use Q=(2^128-1)^2, covering the
private full-width atan2 slope. These are experimental certificate parameters.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/inverse_threshold_feasibility.py
```

Work unit0282 passes38 sampled bands, four independent algebraic tangent enclosures and two
exact Fraction-arctangent checks. The controls reject a band containing rational tan45=1 and
a deliberately wide band. The project-local report under target/trig_inverse_feasibility/0282/
explicitly says inverse_feasibility_cohort_only, retaining exact endpoints and source hashes.

This is neither complete45-million-threshold coverage nor a native atan/atan2 implementation.
Full coverage uses the resumable controller below. Native sign/reciprocal/
quadrant/axis/principal-branch handling, error and workspace remain separate admission work.
Production tangent has a separate resource obligation; this experiment does not close it.

## Native inverse chunk probe

Work unit0283 adds a diagnostic Rust chunk generator and a guarded Python probe:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/native_inverse_probe.py
```

It builds the example with project-local stores and uses the artifact path reported by Cargo.
The Rust child reads only its protocol on stdin and emits exact288-byte rows on stdout.
Each row contains two slope endpoints and four rational gap components. The child is bounded
to100,000 rows; this is a diagnostic chunk boundary, not a formula-language restriction.

Instead of evaluating tangent twice, it encloses the center and widens by16*2^-600 in slope.
Every complete target angle band stays in the first octant, where tan'=sec^2<=2;2*pi<8 then
bounds the whole displacement by that padding. The Python check confirms that the native band
contains the endpoint-based band, rather than requiring their different enclosures to be equal.

The bounded probe passes10,035 exact row checks,67 Python-band comparisons and five actual
invalid-input refusals before output. The report/prefix/build log is under target/native_inverse_probe/0283/.
This supplies generator/parity/cost evidence only. The resumable controller is available below;
complete45M coverage and native atan/atan2 admission remain open. No inverse formula API is available yet.

## Resumable inverse controller and exact reader

The controller retains450 chunks of100,000 rows, frozen build inputs, a verified producer copy,
source/binary hashes and an atomic manifest under target/inverse_threshold_certificate/0284/.
It can use up to four owned children. Only completed chunks that pass independent exact reading
enter the manifest; resume rechecks published bytes. A partial manifest never becomes complete
merely because an invocation exits successfully.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/inverse_threshold_certificate.py --chunks 1 --workers 1
python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py --allow-partial
# After full coverage, require the default complete-population reader:
python3 -I -B docs/tasks/artifacts/formula_structure/verify_inverse_certificate.py
```

Work unit0284 exercises one100k chunk, a verified resume to200k, partial reading and the default
reader's required refusal. Corrupted-row/coverage/source and owned-child cleanup controls pass.
The frozen executable/build inputs preserve actual provenance when unrelated features later
change. Live native source, pi fixture, controller and reader must still match their recorded basis.
This does not claim that the reader independently recomputes every tangent enclosure.

An interrupted run leaves an exclusive lock and child registry. Establish that the recorded
processes are gone before removing its project-owned lock and resuming. Error cleanup reaps
owned children; differing unpublished final bytes are preserved and refused. Complete45M coverage
remains a separate task. Native inverse-function branches/error/workspace admission is still open.

A read-only inspection found generic .bin cleanup would select retained certificate chunks.
Protection is owned as P1 immediately before the full run/next cleanup; no certificate was deleted.
