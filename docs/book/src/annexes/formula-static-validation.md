# Static formula validation evidence

> **Status:** reference-instrument evidence at G1-SLICE.5b.1a/.1b.1. The product libraries still stop at
> [syntax, inputs and canonical identity](formula-recipe-inputs.md). A valid syntax tree is not yet
> a statically accepted recipe, a computed garment or a production approval.

Static checking asks whether names and kinds fit the language before computing any value. A
measurement can be unknown and still have the kind length: the checker may inspect that kind,
while numerical execution must refuse a read of its missing value. The normative rules remain
[contract §§2–6](../spec/formula-language.md) and
[grammar §§5–7](../spec/formula-language/grammar.md).

## Examples of signature decisions

| Formula | Static result | Reason |
| --- | --- | --- |
| min(width) | length, when width is length | one is the minimum variadic arity |
| max(width, height) | length, when both are length | all operands share one arithmetic kind |
| min(width, size_index) | formula_dimension | length and count cannot be mixed |
| within(width, height, eps_geo) | boolean, when both are length | a named tolerance class |
| within(width, height, size_index) | formula_dimension | a size ordinal is not a tolerance |
| arc_length(sweep, radius) | length, for angle and length | explicit angle/radius function |
| sweep * radius | formula_dimension, naming arc_length | the product has no stored kind |
| area_value / width | length | directed area/length quotient |
| width / area_value | formula_dimension | reversing a quotient changes its dimension |
| point_at(edge_value, edge_position) | point, for edge and ratio | reads existing geometry |

A successful selector signature does not construct geometry or resolve a reference. Its runtime
range and geometry checks belong to later stages. Boolean/point/edge operands cannot silently
become arithmetic values; an if condition must be Boolean, and both branches must have the same
arithmetic kind. Both branches' names are inspected even when only one will later execute.

## Complete signature matrix

The tracked static_signature_contract.py under docs/tasks/artifacts/formula_structure loads the
actual reference definitions and table-loading code from run_formula_language_census.sh. Expected
populations are authored independently: all eight kinds, five arithmetic kinds, four negatable
kinds, eight reserved names, positional products/quotients, and all 22 closed built-in/selector names.
Expected signatures are compared with the published populations in both directions; no extra or
missing reference row can disappear behind an example that happens not to call it.

The 4032 actual parse/infer cases cover:

- Every unary/square kind and all 64 ordered kind pairs for each binary operator.
- Every fixed function/selector signature at its arity over all positional kind combinations,
  zero/short/long arity refusals, and min/max at one, two and three arguments.
- Both variadic functions at the 255-argument syntax boundary and first excess, for all five
  arithmetic kinds. A wider call is a structural refusal, independently of its signature.
- All 512 condition/branch kind combinations, unresolved names in either branch and conditional arities.
- All eight reserved candidates against all 64 within operand pairs, plus numeric, derived and
  ordinary-name tolerance refusals and wrong arities.
- All six envelope call names, preserving their named precedence before inspecting an unbound
  argument, plus an undeclared ordinary call.

Environment entries expose only kind. Access to value, state, origin or geometry payload fails;
numerical evaluation, binding/storage and geometry callbacks also fail if invoked. Parsing still
uses the existing literal-input conversion where a control contains a literal; this is not a claim
that input normalization does no numerical work.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_signature_contract.py --mutations
bash docs/tasks/artifacts/formula_structure/run_formula_structure_probes.sh
```

Twelve in-memory variants of the actual reference predicates must fail the matrix assertions:
variadic base arity, tolerance role, count negation, square result, addition consistency, directed
quotient, variadic homogeneity, generic arithmetic membership, Boolean condition, matching branches
envelope precedence and an environmental value read through get. The producer on disk remains unchanged. These are actual failed body
assertions, not merely nonzero script exits.

## Repairs and remaining obligations

D112 repaired within's reference guard, which admitted all reserved names, including the three size
inputs. It now admits precisely the five tolerance names. D113 repaired min/max's undocumented
minimum of two operands; their one-kind variadic rows require one. This clarifies existing rules
and changes no authored grammar or canonical identity bytes.

The signature matrix is complete for its stated finite populations. The namespace/header review
below is separate; atomic whole-recipe preflight remains G1-SLICE.5b.1b.2. Full static obligation
closure belongs .1c. Product declarations, signatures and whole static graph remain .5b.2–.4.
Numerical execution, operations, geometry and two-platform final acceptance retain their .5c–.5g
owners. No product evaluation or API/MCP release claim follows.

## Names and single-statement static checking

The reference now separates three stages: syntax_statement parses a header and its operands;
static_statement checks declaration metadata, names and kinds; statement executes only after that
static stage succeeds. These are reference maintenance interfaces, not new sc-core APIs. The
product syntax parser still exposes unevaluated syntax and has not acquired static acceptance.

| Statement or declarations | Static result |
| --- | --- |
| let width:length=body_width, with body_width declared length but unknown | accepted without reading its state/value |
| let body_width:length=1 cm, with body_width already from measurement | formula_ambiguous_name, preserving measurement and recipe origins |
| a second let width:length=1 cm, after a prior recipe binding | formula_rebinding |
| let eps_num:length=1 cm | formula_rebinding; reserved context names cannot be authored bindings |
| assert closure:eps_geo=width==height, with both declared length | accepted without fetching the tolerance value |
| assert closure:eps_fmt=width==height outside export context | statically accepted; missing valid context value is a runtime check |
| assert closure:size_index=width==height | formula_parse; size_index is outside the closed TOLERANCE grammar role |
| assert closure:eps_num=flag==flag, with flag Boolean | formula_dimension; closure comparisons require arithmetic operands |
| let width:length=if(is_base_size, known_width, missing_width) | formula_unbound_name even if the missing branch would not execute |

Declarations are consumed as ordered pairs before a dictionary can discard a duplicate. The
reference checks machine spelling, declared kind/origin membership, reserved binding and duplicate
names; duplicate reports retain both source origins, including two declarations from the same
origin. A let collision with an earlier recipe binding is rebinding; one with an input origin is
ambiguity. Neither shadows silently. An assertion label names a check and introduces no value
binding; it does not make a later expression name visible.

The nine origin labels are measurement, ease, parameter, profile, material, geometry, recipe, size
and tolerance. All eight reserved names have statically known kinds, even where a context has not
provided their values. The five tolerance annotations are eps_num, eps_geo, eps_fmt, eps_imp and
eps_phys. A non-class annotation such as eps_chord is outside that grammar role; a valid class
with no supplied value has the separate runtime formula_tolerance_unbound rule.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
```

The 1139 independent cases cover all nine-origin/eight-kind metadata combinations, all81 ordered
origin collision pairs, every reserved name against each origin and all six let kinds, spelling,
headers, kind mismatches, forward/self names, either conditional branch and both assertion operands.
Every arithmetic assertion kind pair is checked at all five tolerance classes. Environment entries
expose only kind/origin; any value, state, availability or geometry read fails. Evaluation, storage
and geometry callbacks fail if called, including through the runtime adapter for a static refusal.
Parsing retains its existing canonical literal-input conversion; this is not a claim that parsing
performs no numerical work.

Thirteen compiled in-memory changes to actual guards must fail body assertions. They cover spelling,
origin validation, pair collisions, reserved/input/recipe rebinding, let kind mismatches, assertion
class/kind roles, metadata value reads and execution before static acceptance. Source on disk remains
unchanged during those controls. Existing syntax-only fixtures now use syntax_statement directly,
so syntactically valid reserved-name bindings still have their original canonical bytes while later
static validation refuses them. No Rust syntax or serializer behavior changed.

This proves metadata namespace and single-statement checking in the reference. It does not validate
canonical input adapters, physical geometry or typed product diagnostic payloads; those retain
.5e.1, G2 and .5b.2–.4 owners. Whole-recipe no-execution proof remains .1b.2. D121 (irrational-result
provenance at T1) and D122 (origin-specific missing-value diagnostic routing) are scheduled .5e.3
and .5e.1 reference repairs before their product execution evidence. Neither reference behavior is
claimed correct by these static controls.
