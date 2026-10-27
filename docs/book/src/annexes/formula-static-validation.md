# Static formula validation evidence

> **Status:** reference static checks and coupled public expression/current-statement/whole-recipe review are verified.
> Product libraries implement [syntax, inputs and canonical identity](formula-recipe-inputs.md),
> [sourced names](formula-name-scopes.md), bounded expression/current-statement checks and
> [whole recipe kind proofs](formula-checked-recipes.md). Syntax success alone grants no static proof;
> a complete kind proof grants no numerical result, computed garment or production approval.

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

The complete4032-case matrix and its14 reference faults are retained in the
[reference signature evidence](formula-static-signatures.md#complete-signature-matrix).

### Reference source locations

Both loaders preserve real lines and virtual fault filenames; the
[complete location controls](formula-static-signatures.md#reference-source-locations) remain available.

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
and [complete static graph proofs](formula-checked-recipes.md) are available. Exact call payloads are checked by166 cases/12
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

Complete97-case/75-payload/12-fault local geometry-reference receipts remain in
`docs/history/stitchcad-devnotes-part491.md`.

D140 is repaired in this local reference adapter. These controls do not validate arbitrary provider
metadata, persisted graphs, whole-recipe runtime rollback, product operation identity or physical
geometry. Product selectors/operation-argument execution remains .5f.3b/G2; complete product static
recipe acceptance is implemented below. The grammar and stable diagnostic token set are unchanged.

## Binding-header diagnostic arguments

[Contract5.2.5](../spec/formula-language.md#525-binding-header-dimension-arguments) retains exact
binding_annotation/binding_kind payloads and lexical/child priority. Raw invalid annotations stay
at syntax admission; [current-statement proof](formula-name-scopes.md#check-the-actual-current-statement)
retains actual product context. Historical264 payload/fault controls and examples are preserved whole
in `docs/history/stitchcad-devnotes-part521.md`, retrieved with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part521.md`.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/header_dimension_contract.py --mutations
```

## Names and single-statement static checking

The reference separates syntax admission, metadata/static checking and execution. Names bind once,
origins never shadow, reserved names keep known kinds without values, and both conditional branches
must resolve. Assertion labels introduce no value binding. Current product [ordered name scopes](formula-name-scopes.md),
[whole recipe proofs](formula-checked-recipes.md) and [canonical input adapters](formula-runtime-validation.md)
retain their separate scopes; static acceptance supplies no runtime/geometry result.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_namespace_contract.py --mutations
```

Historical1139 metadata/payload cases, example table and thirteen compiled fault controls are
preserved whole in `docs/history/stitchcad-devnotes-part522.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part522.md`.
Runtime source/provenance and missing-context validation remain separately documented in the
runtime annex; physical geometry and full execution/integration retain their task owners.

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

These authored names identify test examples; neither adds a reserved grammar token.

| Example name | Role |
| --- | --- |
| `collision` | Arbitrary measurement/parameter name used to demonstrate an ordered binding collision. |
| `eps_chord` | Intentionally invalid assertion-class spelling; syntax refuses it as formula_parse. |

## Whole recipe before execution

A recipe must pass static checking in its entirety before its first statement computes a value.
Whole-source syntax/structure, literal inputs and ordered static inference are separate phases.
Each completes for the entire source before the next starts. For example,
`let bad:length=missing` followed by `assert late:eps_chord=1 mm==1 mm` refuses the later
syntax error, without looking up missing. A late invalid annotation, input literal or structural
limit likewise prevents earlier static checking. Syntax/structure also precedes literal conversion:
an earlier `let first:count=340282366920938463463374607431768211456` cannot hide a later
invalid assertion class, open parenthesis or structural bound. Both literal magnitude and structure
can raise formula_domain, so the token alone does not prove phase agreement. Source order remains
within each phase; whole-source lexical validation and raw annotation priority remain intact.

The reference parses each identified statement once into raw number/unit leaves, preserving original
spans. A separate iterative traversal normalizes every literal, including untaken branches, into
checked operand tuples. Static checks reuse those normalized owners. After every input succeeds,
it checks names/kinds in source order and publishes prior
let metadata. It returns a complete tuple plan after all checks; no accepted prefix escapes.
Detached static_statement still validates only its own statement. This reference tuple plan has
no canonical product identity and grants no numerical/geometry execution authority.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py --mutations
```

The [pure reference guards](formula-recipe-inputs.md#pure-input-review-and-phase-reference-guards)
cover imports, borrowed calls and the in-memory mutation mode.

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

Complete196-case/consumer/fault receipts remain in `docs/history/stitchcad-devnotes-part491.md`.

The copied-book instrument plans the complete current book and every selected copy, log and
existing deletion candidate before it activates local stores. An unknown fixture file or directory,
a symlink, a hardlinked output, or an effective input that a copy would replace causes refusal
before creation, deletion or delegation. Empty selected parts directories are protected too.
Pure contracts() validates only its ten reference inputs and never activates stores; borrowed
consumer_contracts(verbose=False) produces only the two late-error copies, and measurement_contract()
produces only its ceiling copy. Each copy is rechecked immediately before replacement and before use.

The real shell replay retains the prepared store environment and the caller's effective
FORMULA_CONTRACT, FORMULA_GRAMMAR, FORMULA_EXAMPLES, FORMULA_FIXTURE and FORMULA_PARTS_DIR selectors.
Its FORMULA_BOOK remains this repository's book, as before. Source inspection and child dispatch use
the same selector snapshot. These locality controls leave the recipe grammar and196 cases unchanged;
intercepted child output is dispatch evidence. The separate real replay supplies numerical evidence.

Recipe size is measured from the accepted worked plan:21 statements. The13 separate refusal
candidates are not appended to that recipe. D123 formerly combined them into34, falsely refusing
a copied book with a25-statement ceiling. That independent control now passes, while restoring
the old aggregate formula in memory causes a named assertion failure. The normative ceiling
remains4096; the smaller value is used only to falsify the measurement in a copied book.

D119's reference ordering and D123's measurement defects are repaired at .5b.1b.2. Product sourced namespaces, kind signatures, scoped checks and immutable whole recipe proofs are implemented. Reference numeric provenance
is [verified separately](formula-runtime-validation.md#approximation-contributions-and-admissible-classes); reference origin/context routing is verified at .5e.1a. Static checking cannot settle
those execution obligations.

## Complete static review and remaining contracts

The static review checks the21 worked statements and13 refusal examples against authored outcomes.
Its100 controls distinguish syntax/static refusals from runtime-only zero division, missing material/
factory values and false assertions. No numeric or geometry callback runs in these controls.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_review_contract.py --mutations
```

Current product APIs provide sourced namespaces, ordered scopes, expression signatures, whole recipe
proofs and coupled static acceptance. [Whole recipe proof](formula-checked-recipes.md) retains current
scope. D154's reference refusal ordinal repair remains OPEN under .5b.4c; the director's existing
arithmetic/product exception does not close it. Persisted-cycle/replay repair remains .5e/.7; physical
geometry and computed cross-platform verification remain G2/.5g.

D124's ruling keeps the current grammar: loop/repeat/while/fn/macro are ordinary scalar names, unknown
calls refuse before argument semantics, malformed definitions parse-refuse, and unsupported powers/
envelope calls keep their declared tokens. ADR-0003 retains that ruling. D125's false-assertion repair
and its exact runtime diagnostics remain in [assertion controls](formula-runtime-validation.md#reference-runtime-assertion-controls).
Original historical matrices, fault controls and proof commentary are preserved whole in
`docs/history/stitchcad-devnotes-part518.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part518.md`.

### Implementation sequence after the review

[Sourced/ordered scopes](formula-name-scopes.md), expression checks and
[whole recipe proofs](formula-checked-recipes.md) are implemented as separate metadata/static stages.
They retain actual sources, both-branch dependencies and diagnostics without executing values.
Persisted corrupt-cycle diagnostics remain .5e.4/.7. Whole historical sequence/proof commentary is
preserved in `docs/history/stitchcad-devnotes-part520.md`, retrieved with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part520.md`.

### Coupled public static review

The maintained review compares actual product/reference expression, statement and complete recipe
interfaces using independently authored source expectations. Canonical owner pointers and complete
graph occurrences come from the product; source-derived ordinals/spans/roles remain independent.
Historical exact counts, fault sets, producer plans, restoration and phase-order receipts remain in
`docs/history/stitchcad-devnotes-part517.md`, retrievable through history_archive.py read.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_coupled_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/first_phase_contract.py --mutations
```

Run source writers exclusively and consume restoration before another build/probe. Dispatch captures
and actual native/reference behavior certify separate scopes. D154's reference ordinal repair remains
open with .5b.4c; the existing director's arithmetic exception leaves that static-lane closure open.
Static proofs grant no numerical result, physical geometry, release readiness or human approval.
The [standalone producer guard](../build-and-checks.md#direct-python-producers) retains current
input/file/store plans, actual driver assertions and unconditional source/artifact restoration.

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
