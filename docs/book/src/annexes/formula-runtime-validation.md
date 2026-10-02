# Formula runtime validation evidence

This annex describes the book reference evaluator's runtime controls, separate from the
[static checks](formula-static-validation.md). Product libraries currently expose syntax, input
normalization and canonical identity; production execution remains G1-SLICE.5c–.5g work.
The [formula chapter](../spec/formula-language.md) supplies the normative rules. This instrument
verifies scoped numerical examples and diagnostics; it does not certify a physical garment.

## Reference runtime assertion controls

An assertion uses an inclusive tolerance: it holds when the absolute difference is at most the
class value in the operands' internal units. The reference preserves the successful five-element
assertion result, including the evaluated operands. A larger difference raises formula_assertion
before any false result can be returned. For example:

```text
assert width_closure: eps_num = 1 cm == 1 cm
assert false_closure: eps_num = 1 cm == 2 cm
```

The first holds. The second raises formula_assertion with false_closure, length values10000 and
20000, and eps_num value1. The diagnostic's owned arguments dictionary contains assertion_name,
left_kind, left_value, right_kind, right_value, tolerance_class and tolerance_value. Values remain
exact rational results, in authored operand order; this instrument does not invent a recipe ordinal
or canonical source identity. These reference fields are not a production typed diagnostic API.

Syntax/name/kind checking still precedes runtime work. Once static checking succeeds, the existing
operand and tolerance evaluation order is preserved: division by zero, unknown values and missing
class context retain their own diagnostic tokens rather than becoming assertion failures. The
book consumer reports the raised token, label, values and class through its existing error path.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/assertion_contract.py --mutations
```

The existing structural runner watches this producer.262 independently authored controls cover all
five arithmetic kinds and all five classes, zero/below/at/above thresholds in both operand orders,
signed values where permitted, unchanged caller bindings, static acceptance of false checks and
earlier static/runtime errors. Context-dependent classes use explicit fixture values25/40/100;
these are controls, not new defaults. Canonical integer input bindings divided by ratio2.0 supply
exact half-unit results. The grammar deliberately makes Count*Ratio a Ratio and Count/Ratio a Count;
the controls follow those declared kinds rather than changing the table to fit a fixture.

A copied worked book falsifies waistband_width_closure: the actual consumer returns1 and reports
formula_assertion with length80000 versus40000 at eps_num1. Eight actual compiled predicate/payload
faults must fail body assertions: false guard, inclusive boundary, token, label, operand order/kind,
class and class value. The reference source stays byte-identical during these in-memory faults.
G1-SLICE.5e.3a owns this repair. D122 reference missing-value routing is described below.
D121 reference contribution provenance is verified below. Product evaluation, geometry and API/MCP/release remain pending.

## Missing values by origin

A name's kind can be known when its value is unavailable. Preflight can accept the name and still
leave a runtime read to refuse. The reference now follows contract3's origin rules:

| Missing input | Runtime diagnostic |
| --- | --- |
| A measurement, Ease, design parameter, profile or material value | formula_unknown |
| A geometry, earlier recipe or size value | formula_unbound_name |
| A tolerance value | formula_tolerance_unbound |

For example, body_width declared as an unknown length from measurement can pass the static check
for let doubled:length=body_width*2. Executing it raises formula_unknown. Missing geometry p raises
formula_unbound_name when read; missing eps_fmt raises formula_tolerance_unbound. None receives a
replacement value. The original state is retained for an absent fact, even when a malformed record
claims known but supplies no value; this routing is not approval of that record as canonical data.

### Optional context values

The reference's reserved tuple contains kind, always-visible metadata and a supplied value. The
always flag records the normative availability rule; it does not prevent an optional context from
supplying a value. A None value means unavailable. For example, eps_fmt with length/False/25 is
readable when the fixture supplies25µm. A size context supplying is_base_size=False is readable
and selects a conditional's else branch. False and zero are values, not absence tests.

The reference accepts an optional explicit context label. A missing tolerance diagnostic includes
that label when supplied; no label is guessed for a caller that supplies none. This label is
instrument context, not proof of a real export target, profile, size set or authority grant.

### Diagnostic arguments and malformed declarations

The owned error arguments retain name and declared origin. Unknown-fact errors retain the actual
state; unbound errors retain the reference's closed, sorted origin vocabulary as origins_searched.
A missing undeclared name has no invented origin. Tolerance errors retain explicitly supplied
context. Artifact-blocking policy, object evidence, statement ordinals and canonical expression
context remain product requirements; these reference fields are not their typed production API.

Missing declaration kind/origin fields, non-string or undeclared metadata and invalid record shapes
raise formula_parse rather than leaking KeyError or TypeError. An explicit state requires one of the five authored tokens; malformed state raises formula_parse.
An unknown record carrying a value is also malformed and refused. Unknown lazy geometry is refused
before resolution or cache publication. An absent fact requires an explicit valid authored state. Static namespace checks
still observe only kind/origin. They do not read values or uncertainty states. Existing populated
numerical fixtures without state metadata remain readable; full canonical input adapters are pending.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/origin_value_contract.py --mutations
```

The structural runner watches1466 independent controls: nine origins/eight kinds/five missing states,
four populated states, zero/false values, all eight reserved names, explicitly supplied contexts,
taken-only missing reads, malformed declarations and populated-state consistency. Lazy point/edge resolution, subsequent cached
reads and upstream failures retain their established behavior. No partial cache is published after
a failed resolution. The reference geometry values are arithmetic fixtures, not real entity references.

Two actual copied worked books pass all21 static statements, then refuse missing size_index and
eps_fmt at runtime with their distinct tokens. Thirteen compiled predicate/payload faults require body
assertion reds; the reference source remains unchanged during those in-memory faults. Existing
namespace1139/thirteen fault controls still trap all numerical/state/geometry access.
G1-SLICE.5e.1a closes D122 origin/context routing and D127 malformed-input exceptions and D128 uncertainty-state bypass. Product
adapters and complete production diagnostic/provenance context retain their separate owners.

## Approximation contributions and admissible classes

A value retains an immutable set of approximation-producing calls in its executed dependencies.
The set is evidence about how the result was obtained: sin(90 deg) remains a sine contribution
even though its rounded answer equals ratio1.0. Named T1 comparisons require an empty set.
T2 or looser admits those contributions and then applies the usual inclusive threshold.

```text
assert direct: eps_num = sin(90 deg) == 1.0
let rounded: ratio = sin(90 deg)
assert stored: eps_num = rounded == 1.0
within(rounded, 1.0, eps_geo)
```

Both assertions raise formula_domain. The within comparison holds. The owned diagnostic arguments
are comparison (the assertion label or within), tolerance_class and sorted contribution_sources.
These examples retain sin as the source. Choosing eps_num with a larger supplied numeric value
still refuses: admissibility follows the symbolic class. The director's D121 ruling is recorded
in ADR-0003; these instrument fields are separate from the pending production typed diagnostic API.

The reference marks sqrt, hypot, sin, cos, tan, atan, atan2, arc_length, dist and dir, and the
authored round_to quantization operation from contract4.2. Exact rational operations and input or
binding rounding alone add no approximation source. Arithmetic carries all executed inputs,
including cancellation to zero. min/max/clamp retain evaluated candidates/bounds; an evaluated
Boolean condition also contributes to the branch it selects. Bare exact comparisons retain their
existing numeric behavior and carry their inputs' sources into the Boolean result.

Only the taken branch executes. For example, within(if(1 == 1, 1.0, sin(90 deg)), 1.0, eps_num)
holds with no sine contribution. A binding stores and subsequently reads both its integer value
and contribution set; rounding and later reads cannot erase that evidence. The actual book consumer
publishes the returned sources alongside each stored binding.

Lazy point/edge operations retain the sources of their evaluated arguments after successful
resolution. Point coordinates retain separate sets: if x=1 um and y=hypot(3 um,4 um), a comparison
of x against1um may use T1, while y requires T2 or looser. Cached reads preserve the same distinction.
An error in a later point coordinate publishes neither a value nor new provenance. The opaque
point_at and length-only edge fixtures remain arithmetic models, with no curve-accuracy proof.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/provenance_contract.py --mutations
```

The structural runner watches425 independent controls across all approximation signatures, five
arithmetic kinds/classes, operand orders, named refusals, ordinary operators, stored/Boolean reads,
lazy branches, geometry/cache/coordinate separation, earlier errors and exact controls. A copied
worked book compares equal side_seam_length bindings at T1: all21 statements preflight, then the
actual consumer refuses with allocation_balance, eps_num and hypot.26 compiled actual guard/source/
cache/consumer faults must fail body assertions; the reference source remains byte-identical.
G1-SLICE.5e.3b closes D121's reference scope. Arbitrary-input transcendental rounding, product
execution, real geometry, replay, API/MCP and release correctness remain separate obligations.
