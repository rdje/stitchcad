# Tangent uses bounded adaptive certification with explicit execution resources

- **Type:** `decision`
- **Date:** `2026-10-30`
- **Status:** `active`; algorithm/resource contract selected, native implementation pending
- **Owner / source:** engineer under the director's delegated implementation authority;
  G1-SLICE.5d.1.2.2.2.d.2.2.2, implementation G1-SLICE.5d.3.2.2

answers: "how will tangent terminate within selected resources?" · "does insufficient tangent precision invalidate a formula?" · "why does tangent require explicit execution limits?"

## Decision and scope

Select bounded adaptive interval certification for tangent. The caller explicitly supplies a
maximum candidate-integer bit extent and a cumulative series-term budget. There is no default,
hard-coded maximum precision, new mathematical input restriction or changed rounding quantum.
The bit quota bounds the largest candidate integer, not total heap bytes. The term quota bounds
charged series work, not wall-clock time. Complete allocator/CPU/Design envelopes remain separate.

A request returns only a certified correctly rounded value, an original mathematical failure
(actual pole or result width), or an explicit resource refusal. A resource refusal carries the
original input, selected limits, charged work, prior reservations and next scheduled demand. It is
outside the formula-language diagnostic families: no invented formula_domain, rational-width,
Unknown, cached value or precision-refusal code may stand in for insufficient execution resources.

This preserves the distinction already used by expression/source/recipe workspace admission.
An authored input remains mathematically valid when its requested execution cannot be admitted.
The numerical contract still requires the true nearest ppm with half-away rounding and the
original128-bit input/result boundaries. Resources cannot enlarge those numerical domains.
This decision admits a method for native implementation; it does not claim an existing tangent API.

## Complete bounded request

After the existing actual-kind check, reserve the complete156-bit folding extent before private
period/reflection arithmetic. Exact zero, signed unit and pole cases need no approximation series.
They retain their original mathematical meaning even with a zero series-term allowance. An
insufficient folding reservation instead produces its genuine earlier resource refusal.

For other inputs start at B=128,which represents the2^-128 denominator floor exactly at dyadic
scale,and double B after an undecided enclosure. At each iteration
preflight both complete demands before charging or computing it:

- Candidate integer bits: I(B)=16B+bit_length(2B+1)+16, using the proved per-iteration extent.
- Additional series terms: T(B)=4B, including fresh pi and both unrounded components.

Check the bit quota first, then the remaining cumulative term quota. Only after both fit may the
iteration run. Original input values/providers are never re-read; refinement uses the same actual
completed operand. The chosen precision is internal progress, not a language limit or authored
model field. Endpoint cases use the exact classification rather than demanding an unused series.

Only whole closed-interval agreement certifies a rounded value. An entire interval beyond the
final output-width boundary certifies the original width failure. Otherwise refine or return the
next scheduled resource refusal. No partial source/recipe result or approximate numerical value
becomes accepted. Work counters charge admitted iterations;the bit counter records the highest
admitted reservation,not an observed heap measurement. A failed reservation is never charged.
Resource payloads must not pretend their next scheduled demand is a mathematical minimum.

Every selected finite request terminates: before any unbounded refinement, increasing I(B) and
positive charged T(B) must exceed a finite selected quota. Actual charged terms never exceed the
term quota; no candidate integer exceeds the admitted per-integer extent. Fixed-count folding
and per-iteration bookkeeping remain bounded by the same input/iteration extents. Native checked
counter/extent arithmetic must refuse before allocation on overflow; it must not wrap or create a
numerical rational-width diagnostic. Primitive arithmetic and allocation contracts must be checked
in the native implementation. These are logical resource guarantees, not latency/heap-byte claims.

## Correctness and completeness of the strategy

The [midpoint/resource fact](fact_trig-midpoints-and-resource-bound.md) derives exact input
folding, original pole/width handling, positive denominator floors, fresh convergent pi and
unrounded component enclosures. Their widths tend to zero. Calcut's rational-angle theorem
excludes all finite tangent half-ppm ties. Consequently each admitted finite input eventually
has a unique rounding certificate, or a genuine width certificate, under sufficiently increased
mathematical resources. No single practical universal precision budget is claimed or needed for
the bounded-request contract. Available platform resource/counter capacity may still refuse.

Successful results must agree across resource selections and platforms; smaller limits may
produce a resource refusal. The selected execution policy belongs with request/result/error
metadata, not canonical authored values. Load/replay must retain the policy and its scope before
full runtime closure. No normal policy choice may silently publish a lower-precision result.

For the actual near-midpoint witness, precisions128/256/512 charge512+1024+2048=3584 series
terms. I(128)=2073, I(256)=4122 and I(512)=8219 candidate bits. Limits8219/3584 certify zero;
a4122-bit quota refuses the next8219-bit reservation after1536 terms. A1536-term quota refuses
the next2048 terms. These are examples of the algorithm's reservations, not universal required
budgets or minimum-resource claims for that numerical input.

## Native implementation requirements

Keep explicit immutable limits and rich resource refusals separate from existing numerical
failures. Preserve actual operand, selected policy, unit, maximum, consumed and requested data.
Check all complete extents before creating the corresponding operands or executing a series.
No precision ceiling, fallback libm, guessed value, model clone or diagnostic allocation during
allocator failure is licensed. Accepted results must be constructed only by successful execution.

The source-bearing wrapper must retain the actual executed tangent contribution even for exact
zero/unit outcomes. Whole expression/source/recipe/Design plan adoption must select and retain
kernel policies before values; missing policy is a preparation/resource capability issue rather
than a new formula-language failure. Existing non-tangent entry points keep their scope until
that integration is implemented. Current plan/currentness/atomic-result guarantees stay binding.

## Focused proof and remaining verification

The registered tangent_budget_plan.py uses the actual convergent model. Nineteen exact controls
cover endpoint/pole/width values, early and later bit/work refusals, first-failure precedence,
original input/policy retention,counters,successful-result invariance under larger limits and
absence of values on refusal. Three in-memory
actual guard-body omissions must fail their intended controls; tracked source bytes remain exact.
The near-midpoint expected side is independently computed by arctangent/alternate-pi intervals.

Native kernel/contracts, original typed failures, budget preflight and source provenance follow
G1-SLICE.5d.3.2.2. Full evaluator/Design policy integration, whole resource/allocator envelopes,
load/replay, broad/platform/physical acceptance and production approval retain their existing
owners. This record grants no human release approval or unobserved CI verdict.
