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
| sweep / radius | formula_dimension | the quotient has no signature; arc_length advice would change its meaning |
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

Fourteen in-memory variants of the actual reference predicates must fail the matrix assertions:
quotient product advice, missing product advice, variadic base arity, tolerance role, count negation,
square result, addition consistency, directed quotient, variadic homogeneity, generic arithmetic
membership, Boolean condition, matching branches, envelope precedence and an environmental value
read through get. The producer on disk remains unchanged. These are actual failed body assertions,
not merely nonzero script exits.

### Reference source locations

The table-loaded reference is extracted from a Python heredoc inside the shell census. Its loader
preserves the original header line offset when compiling the normal reference, so tracebacks name
the real source line and show the matching source text. In-memory fault variants use an explicitly
virtual filename instead of claiming that their changed code is the unchanged source file.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/reference_locator_contract.py --mutations
```

D144's controls independently parse the original definitions/decorators and compare all86 compiled
code locations, check the actual geometry dimension traceback text and inspect a virtual fault
frame. Two actual compiled loader faults must fail body assertions: losing the header offset and
letting a fault impersonate the original file. Source bytes remain unchanged. This fixes diagnostic
locations in the test instrument; formula grammar, diagnostic arguments and execution results are
unaffected. Structural probes watch these controls alongside the existing signature evidence.

## Repairs and remaining obligations

D112 repaired within's reference guard, which admitted all reserved names, including the three size
inputs. It now admits precisely the five tolerance names. D113 repaired min/max's undocumented
minimum of two operands; their one-kind variadic rows require one. This clarifies existing rules
and changes no authored grammar or canonical identity bytes.
D134 repairs product guidance incorrectly attached to refused angle/length quotients; the matrix
now asserts both presence and absence of that advice for every operator/kind pair. The two actual
faults restore the quotient mistake and suppress valid product advice, respectively.

The signature matrix is complete for its stated finite populations. The namespace/header review,
atomic whole-recipe preflight and full static obligation review below are also verified in the
reference. Product [sourced declarations and namespaces](formula-declarations.md),
[ordered metadata scopes](formula-name-scopes.md) and [operator signatures](formula-operator-signatures.md)
are available, as are [built-in and selector signatures](formula-builtin-signatures.md).
[Typed wanted-kind catalogs](formula-wanted-signatures.md) and
[source-bearing call lookup](formula-call-lookup.md) and
[initial-scope expression proofs](formula-wanted-signatures.md#bounded-product-expression-checking)
and [current-statement checks](formula-name-scopes.md#check-the-actual-current-statement)
are available; complete static graph remains .5b.4. Exact call payloads are separately checked by166 cases/12
actual compiled faults; the older recognition/kind matrix alone certifies no diagnostic payloads.
Reference expression dimension payloads are now verified by4023 cases/15 actual compiled faults,
including complete kinds/wanted rules and multiple-error selection; see the
[wanted-rule annex](formula-wanted-signatures.md#reference-expression-dimension-payloads).
[Header arguments](#binding-header-diagnostic-arguments) now retain their actual case context. Reference geometry argument checks are verified below at .5f.3a.
The complete static review covers its stated expression/header outcomes; local provider checking
adds no product accepted-expression, complete operation graph or physical geometry proof.
Numerical execution, operations, geometry and two-platform final acceptance retain their .5c–.5g
owners. No product evaluation or API/MCP release claim follows.

## Geometry provider argument checking

The reference point provider takes x and y formula sources; its edge provider takes a len source.
These are operation arguments, not formula constructors. All sources parse before any kind is
resolved, then all kinds resolve in that order. Each argument must be length before the first value
is computed. The scoped payload is defined in [contract5.2.4](../spec/formula-language.md#524-geometry-argument-refusals).

| Provider | Argument formulas | Result |
| --- | --- | --- |
| point | x:1 mm; y:2 mm | exact coordinates1000/2000µm |
| point | x:1.0; y:2 deg | formula_dimension; actual ratio/angle, wanted length/length |
| point | x:1 um / 0; y:1 deg | formula_dimension before the division executes |
| point | x:1.0; y:missing | formula_unbound_name; all kinds are unavailable |
| edge | len:1 deg | formula_dimension; actual angle, wanted length |
| point | x:hypot(3 um,4 um); y:7 um | x contribution hypot; y has no approximation source |

No failed static check changes the provider's result, lazy-cache flag or contribution metadata.
An invalid or unbound child preserves its own error; a provider mismatch reports every resolved
argument kind, its real x/y/len position, and required length. An untaken conditional branch still
has its names checked. Parsing retains literal-input normalization; execution traps cover expression
evaluation, value reads and geometry callbacks.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/geometry_argument_contract.py --mutations
```

The97 independent cases cover all64 ordered point kind pairs and all eight edge kinds;70 invalid
named combinations expose only kind and trap execution. Literal/computed refusals and child-error
priority include an early zero divisor, approximation calls, syntax errors and an untaken branch.
Seventy-five complete provider dimension payloads are checked. Successful point/edge cache replay
verifies exact values, source unions, asymmetric per-coordinate sources and contributions inherited
from named values. Twelve compiled actual source faults must fail body assertions: guard bypass,
omitted y, execution during kind checking, reversed kinds/order, fabricated kinds, missing argument,
wrong wanted kinds/scope, premature cache publication, lost coordinate/edge sources and swapped
coordinate sources. The reference on disk remains unchanged by these faults.

D140 is repaired in this local reference adapter. These controls do not validate arbitrary provider
metadata, persisted graphs, whole-recipe runtime rollback, product operation identity or physical
geometry. Product selectors/operation-argument integration remain .5f.3b/G2, and complete product
static recipe acceptance remains .5b.4. The grammar and stable diagnostic token set are unchanged.

## Binding-header diagnostic arguments

The reference distinguishes the two header cases in
[contract5.2.5](../spec/formula-language.md#525-binding-header-dimension-arguments).
For let saved:point=missing, formula_dimension has scope binding_annotation, raw_annotation point,
operation let, name saved, annotation_span (10,15), and all six wanted_kinds. The RHS is not parsed;
point is not presented as a bindable declared kind. Whole lexical errors still take precedence.

For let saved:length=1.0, the same token has scope binding_kind, declared_kind length,
expression_kind ratio, annotation_span (10,16), and wanted_kinds (length,). No value is evaluated.
An unresolved RHS or invalid child operation keeps its own error before header comparison.
Detached syntax/static/runtime-adapter refusals omit statement_index. Ordered whole preflight
rebases spans and includes its actual one-based ordinal; a late error publishes no partial plan.
The reference supplies no canonical expression. [Product current-statement proof](formula-name-scopes.md#check-the-actual-current-statement)
is available; raw invalid annotations stay at syntax admission.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/header_dimension_contract.py --mutations
```

The 264 cases check232 exact payloads, all six declared/eight expression kinds, raw spelling,
whitespace/local/global spans, late whole-recipe errors, lexical/child priority and runtime-domain
separation. Metadata permits no state/value read; parse and execution traps enforce phase boundaries.
Thirteen actual compiled body assertion faults must fail, with on-disk source unchanged.

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

## Ambiguous-name diagnostic sources

Two declarations named `collision`, first a measurement of kind length and then a parameter
of kind angle, refuse `formula_ambiguous_name`. Arguments retain `name=collision`,
`origins=(measurement, parameter)`, and both actual kind/origin sources. The ordered pair adapter
retains declaration positions1 and2; neither becomes a recipe ordinal. Two measurement declarations
also refuse and retain both equal origins and both distinct positions.

For an input named width followed by `let width:length=missing`, the collision precedes RHS
name lookup. The prior source retains its input metadata; the attempted source retains the actual
let name/statement spans and its whole-recipe ordinal when available. A detached dictionary has
no original declaration position or whole-recipe ordinal to invent. This is diagnostic metadata,
with no value/state/geometry query. Canonical product sources retain their actual record identities.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/ambiguity_payload_contract.py --mutations
```

The producer checks4192 exact argument payloads across all nine origins, eight input kinds and
six bindable annotations, including equal origins and interrupted positions. Sixteen actual faults
compiled in memory trigger body assertions; tracked source remains unchanged. Numerical execution
is trapped. The token and accepted grammar remain unchanged.

## Whole recipe before execution

A recipe must pass static checking in its entirety before its first statement computes a value.
Whole-source syntax/input validation precedes ordered static inference. For example,
`let bad:length=missing` followed by `assert late:eps_chord=1 mm==1 mm` refuses the later
syntax error, without looking up missing. A late invalid annotation, input literal or structural
limit likewise prevents earlier static checking. Within the input phase, original statement order
and local validation priorities remain intact; whole-source lexical validation comes first.

The reference parses each identified statement once, retaining its actual tuple AST and original
spans. Only after every input succeeds does it check names/kinds in source order and publish prior
let metadata. It returns a complete tuple plan after all checks; no accepted prefix escapes.
Detached static_statement still validates only its own statement. This reference tuple plan has
no canonical product identity and grants no numerical/geometry execution authority.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py --mutations
```

D148's584 independent controls combine earlier missing-name, kind, callee, input-collision and
reserved-name errors with later malformed syntax, invalid annotations, literal width and node/
conditional/4096-statement bounds. Traces verify all input finishes before static inference or
prior-binding publication, original operand tuple identity, one parse per statement and real
spans/ordinals. Eleven actual compiled faults trigger body assertions; source stays unchanged.
Numerical/provider/state access is trapped. Grammar and accepted limits remain unchanged.
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
| Complete typed diagnostic arguments | reference call/expression/provider/header payloads verified | .5b.2–.4 and command .6 |
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
is retained in ADR-0003. .5b.1c.2 closes the reference review; product namespace and local kind proofs are available,
while atomic graph acceptance remains .5b.4. This clarification changes diagnostic promises to match v1's
closed grammar and enables no excluded capability.

The reference runtime assertion repair closes D125 at G1-SLICE.5e.3a. A false assertion now raises
formula_assertion; its name, exact left/right values and kinds, and symbolic tolerance class/value
are retained. A statically valid false assertion still passes preflight, because preflight computes
no verdict. [Runtime assertion controls](formula-runtime-validation.md#reference-runtime-assertion-controls) describe the repair.
D121 reference provenance is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes). D122 reference origin/context routing is verified
[separately](formula-runtime-validation.md#missing-values-by-origin); product adapters remain .5e.1.
No static review result approves numerical determinism, a physical garment or a production release.

### Implementation sequence after the review

The product namespace foundation .5b.2 accepts immutable typed declarations from canonical inputs,
validates machine names and consumes declaration pairs before an index can discard collisions.
[Ordered metadata scopes](formula-name-scopes.md) retain actual prior let annotations and locations;
complete expression/recipe acceptance remains the next two stages.
Kinds and origins are separate from numeric availability. Reserved names have known kinds even
without an instance/export context. Geometry declarations refer to prior operation outputs;
they do not authorize construction. Input adapters must preserve existing source identities and
avoid a sc-core to sc-measure dependency cycle. D124 preserves the existing reserved-word set.

The product expression checker .5b.3c.2b.2 consumes bounded normalized syntax and the checked
initial namespace, preserving all ordered dependencies and checking both conditional branches.
Each refusal carries its actual operation, complete resolved kinds and wanted rules, with the
existing angle-times-length hint. Call/tolerance-role populations match the chapter in both
directions. Envelope dispatch applies before operand semantics after syntax/input succeeds.
No numerical, tolerance-value, storage, geometry or solver callback belongs in this stage.
[Current-statement/annotation integration](formula-name-scopes.md#check-the-actual-current-statement)
is implemented at .5b.3c.3b; whole acceptance remains .4.

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
