# Static formula validation evidence

> **Status:** complete reference-instrument static review at G1-SLICE.5b.1. Product libraries implement
> [syntax, inputs and canonical identity](formula-recipe-inputs.md) and [closed declaration metadata](formula-declarations.md). A valid syntax tree is not yet
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

The signature matrix is complete for its stated finite populations. The namespace/header review,
atomic whole-recipe preflight and full static obligation review below are also verified in the
reference. Product [sourced declarations](formula-declarations.md#immutable-sourced-declarations) are
available; namespace, signatures and static graph remain .5b.2c/.2d–.4.
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
.5e.1, G2 and .5b.2–.4 owners. Whole-recipe checking is described below. D121 reference provenance at T1 is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes). D122 reference origin/context routing is
[separately verified](formula-runtime-validation.md#missing-values-by-origin); these static controls
make no runtime correctness claim. Product input adapters remain .5e.1.

### Reserved-name diagnostic sources

formula_rebinding has two source-bearing cases, defined in [contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources).
The reserved_name case carries name, reserved_source and attempted_source. Reserved metadata
contains role, kind, origin and required_context. It carries no numerical value, availability or
recipe ordinal. Physical tolerance has origin tolerance and provider profile; those are distinct.

The reference identifies an initial attempt by its real ordered declaration_index, kind and
origin. This is a locator into caller-supplied metadata, not a canonical record identity claim.
An attempted recipe let retains its annotation, whole/name spans and an actual ordinal when
whole preflight supplies it. Detached checking retains local spans and omits the ordinal.
The recipe_name case retains prior_source and attempted_source; whole preflight additionally
supplies prior_statement_index and statement_index from the actual accepted earlier binding
and current statement. An intervening assertion counts toward order but never creates a binding.

For example, an initial measurement declaration named eps_phys refuses with a reserved source
of kind length, origin tolerance and required_context profile, plus the measurement attempt's
metadata position. No zero statement index fills an absent location. For a second let width
after an assertion, the actual indices are1 and3, with global spans for both bindings.
Detached input metadata claiming recipe origin cannot prove a prior source location; the reference
reports its known metadata only. Canonical product source locators and namespace validation retain
their separate .5b.2 scope. Debug/logging policy and localized rendering remain product obligations.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/reserved_diagnostic_review.py --mutations
```

The producer checks3624 actual argument cases: eight reserved metadata rows, nine-origin/eight-kind
declarations at three positions through direct and whole entry points; six annotation kinds through
detached/runtime-static and whole entries; real prior indices and assertion gaps. State/value and
execution reads are trapped. Nineteen compiled actual field/token/location faults fail body
assertions; source on disk stays unchanged. D131's delegated decision repairs its argument contract
without changing grammar or the stable refusal token; independent approval remains unclaimed.

## Whole recipe before execution

A recipe must pass static checking in its entirety before its first statement computes a value.
For example, with width declared as length:

```
let first: length = width / 0
let last: count = width
```

The last statement has a kind error. The reference refuses formula_dimension without executing
the first statement; its zero divisor therefore never reaches numerical evaluation. With the last
statement removed, the recipe is statically valid and its division refusal belongs to execution.
Static validity says that names and kinds fit; it does not promise that values can be computed.

The reference's preflight method accepts original recipe source and initial declaration pairs.
It locates top-level let/assert boundaries without treating newlines as statement terminators,
preserves the original source slices and literal spacing, and checks statements in authored order.
Each accepted let contributes only kind/origin metadata to the next statement's local namespace.
Forward/self references fail, both conditional branches are checked, and assertion labels introduce
no binding. Bare expressions remain available to the instrument's individual-expression tests;
the whole-recipe grammar admits only let/assert statements. Empty recipes are accepted.

The plan is returned as one tuple after every statement passes; a late error returns no accepted
prefix and never changes the caller's declarations. This reference plan is an instrument result,
not the future immutable typed product graph. Parsing still performs canonical literal-input
conversion. Preflight never reads input values, states, context availability or physical geometry,
and never calls numerical evaluation, binding storage or geometry resolution.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_recipe_contract.py --mutations
```

The196 independently authored whole-source controls cover original slices and headers, all nine
origins, all81 ordered origin collisions, reserved declarations, unknown context values, declaration
order and late refusals. Actual recipe lengths4095/4096 succeed and statement4097 fails for either
header. Expressions at255/256 nodes and conditionals at15/16 levels succeed; node257 and depth17
fail before execution. Metadata-only inputs and trapped callbacks make forbidden reads observable.

The book consumer now preflights all17 bindings and four assertions together before L2 numerical
replay. Two copied books combine an early zero divisor with a last-assertion kind/name error;
the actual consumer refuses before any statement callback. The unmodified shell entry point still
computes all published values and raises all13 expected refusal diagnostics. Compiled in-memory
guard faults also test whether boundary/order/limit guards or the consumer's preflight call can
be bypassed; the actual producer on disk stays unchanged during those controls. Fourteen actual
guard faults must fail body assertions, including early consumer evaluation and measurement faults.

Recipe size is measured from the accepted worked plan:21 statements. The13 separate refusal
candidates are not appended to that recipe. D123 formerly combined them into34, falsely refusing
a copied book with a25-statement ceiling. That independent control now passes, while restoring
the old aggregate formula in memory causes a named assertion failure. The normative ceiling
remains4096; the smaller value is used only to falsify the measurement in a copied book.

D119's reference ordering and D123's measurement defects are repaired at .5b.1b.2. Product static namespaces, immutable
dependency graphs and complete typed diagnostics remain .5b.2–.4. Reference numeric provenance
is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes); reference origin/context routing is verified at .5e.1a. Static checking cannot settle
those execution obligations.

## Complete static review and remaining contracts

The .5b.1c.1 review checks all21 worked statements and all13 refusal sources against independently
authored expected static outcomes. Three refusal examples are statically valid: division by zero,
unknown material shrinkage and an unavailable factory tolerance fail only when executed. A false
arithmetic assertion is also statically valid; its failure belongs to execution. All other listed
refusals fail syntax or static checking. No numerical or geometry callback runs in these controls.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations
```

The100 cases also check all six envelope call spellings before operand name/kind resolution,
including either conditional branch. Envelope dispatch begins after valid syntax: malformed calls
still fail parsing. Expected envelope populations and book statement/refusal populations are compared
in both directions; seven actual compiled faults and three loaded documentation faults must fail
body assertions. Source remains unchanged.

| Requirement | Reference evidence | Remaining product owner |
| --- | --- | --- |
| Eight kinds, six let kinds; no implicit conversion | signature4032 and namespace1139 controls | .5b.2/.3 |
| Nine origins, eight reserved names, collisions and spelling | namespace1139, recipe196 controls | .5b.2 |
| Every operator/function/selector signature and arity | closed signature matrix;22 names | .5b.3 |
| Tolerance-name roles; Boolean test; both branches | signature/namespace matrices, recipe preflight | .5b.3/.4 |
| Declaration order, headers, no accepted prefix on late error | whole-source196, actual consumer ordering | .5b.4 |
| Statement4096, expression256, conditional16 boundaries | recipe boundaries; earlier syntax/input controls | product syntax implemented; .5b.4 integration |
| All worked and refusal static outcomes | independently authored21/13 populations | .5b.4; runtime rows .5e |
| Envelope dispatch before operand semantics | six calls/either branch; actual guard fault | .5b.3 |
| Exact source and canonical identity | earlier product syntax/input/identity controls | .5b.4 semantic error context |
| Complete typed diagnostic arguments | reference tokens/messages only | .5b.2–.4 and command .6 |
| Persisted cycles and atomic runtime/replay behavior | outside these static instrument controls | .5e/.5f and storage .7 |
| Physical geometry and cross-platform computed values | outside these static instrument controls | G2 and .5g |

**D124 is resolved by the director's ruling2026-10-03: keep the current grammar.** Excluded
loop/function/macro capabilities introduce no additional source forms or keywords. Unknown calls
such as loop(width), repeat(2,width) and while(width>0 um) raise formula_unbound_name, before
argument semantics, including in an untaken conditional branch. Malformed fn/macro definition
shapes raise formula_parse. Declared scalar names loop, repeat, while, fn and macro remain valid.
Recognized non-square exponents retain formula_unsupported; envelope calls retain their tokens.

The recognition controls compare actual contract6 cells and grammar1.1 keyword populations with
independently authored expectations; ordinary names/let headers, all three reserved words and
unknown-call/parse precedence are exercised with execution and values trapped. The concrete ruling
is retained in ADR-0003. .5b.1c.2 closes the static reference review; production namespace/type/graph
implementation remains .5b.2–.4. This clarification changes diagnostic promises to match v1's
closed grammar and enables no excluded capability.

The reference runtime assertion repair closes D125 at G1-SLICE.5e.3a. A false assertion now raises
formula_assertion; its name, exact left/right values and kinds, and symbolic tolerance class/value
are retained. A statically valid false assertion still passes preflight, because preflight computes
no verdict. [Runtime assertion controls](formula-runtime-validation.md#reference-runtime-assertion-controls) describe the repair.
D121 reference provenance is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes). D122 reference origin/context routing is verified
[separately](formula-runtime-validation.md#missing-values-by-origin); product adapters remain .5e.1.
No static review result approves numerical determinism, a physical garment or a production release.

### Implementation sequence after the review

The product namespace slice .5b.2 will accept immutable typed declarations from canonical inputs,
validate machine names and consume declaration pairs before an index can discard collisions.
Kinds and origins are separate from numeric availability. Reserved names have known kinds even
without an instance/export context. Geometry declarations refer to prior operation outputs;
they do not authorize construction. Input adapters must preserve existing source identities and
avoid a sc-core to sc-measure dependency cycle. D124 preserves the existing reserved-word set.

The product type-checking slice .5b.3 will consume bounded normalized syntax and the checked
namespace, preserving all ordered operands and checking both conditional branches. Each refusal
must carry its actual operator/function, operand kinds and expected rule, with the existing
angle-times-length hint. Function/selector and tolerance-role populations must match the chapter
in both directions. Envelope dispatch applies before operand semantics after syntax succeeds.
No numerical, tolerance-value, storage, geometry or solver callback belongs in this stage.

The whole-validator slice .5b.4 will inspect every statement in declaration order and return an
immutable typed dependency graph only after complete success. Dependency edges include untaken
branches; each name resolves to an initial declaration or a prior statement. A late error returns
no accepted graph prefix. Diagnostics retain actual source spans, known statement indices and
canonical identity where available; invalid syntax/input must not acquire invented context.
Syntax and identity APIs keep their existing scopes. Persisted corrupt-cycle diagnostics remain
the loader/replay obligation at .5e.4/.7; ordinary forward/self names remain unbound-name refusals.

### Parameter quantization and curve length

D126 corrected grammar6.1's claim that the10m piece bounding box guarantees5µm parameter
placement. The supported circular arc with centre(0,0), radius5m, start0°, end270° counter-clockwise
contains all four circle extrema and fits a10m square. Its length is7.5πm, about23.562m;
half a10⁻⁶ ratio quantum gives about11.780972451µm displacement. The endpoint chord displacement
is also about11.780972451µm, exceeding the internal10µm T2 value.

The static_review_contract.py producer verifies the public curve/box/T2 declarations, actual
reference arc-length binding23561945µm, an independent standard-library chord calculation and
an exact rational lower bound. Using3<π<22/7 and sin(x)>x−x³/6 at these small positive angles
proves chord displacement>10µm without trusting the reference approximation or floating-point π.
This is a mathematical counterexample, not an executable product geometry test.

For an edge of actual length L, half-quantum parameter error can contribute up to L/(2×10⁶)
of arc-length displacement before geometry approximation is added. A10m straight edge gives5µm;
a bounding box is insufficient evidence of that length. Product selectors and their combined
quantization/geometry error budgets remain .5f.3/G2. The correction changes no units, ratio quantum,
structural domain or tolerance class.
