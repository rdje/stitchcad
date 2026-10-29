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

The reference point provider reads x/y formula sources; the edge provider reads len. These are
operation arguments, not formula constructors. All sources parse, then all kinds resolve in order,
then every argument must be length before any value is computed. Child errors retain priority;
provider mismatches retain every resolved kind, actual x/y/len position and required length.
Untaken branches still have their names checked. Failure leaves result/cache/contributions unchanged.
The payload contract remains [§5.2.4](../spec/formula-language.md#524-geometry-argument-refusals).

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/geometry_argument_contract.py --mutations
```

The complete historical examples,97 cases/75 payloads/12 faults and D140 local-adapter scope remain
whole in `docs/history/stitchcad-devnotes-part529.md`; use
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part529.md`.
These controls do not prove arbitrary provider metadata, whole runtime rollback, product operation
identity or physical geometry. Product selectors/operation execution remains .5f.3b/G2; current
static recipe acceptance is documented below. Grammar and stable diagnostic tokens are unchanged.

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

[Contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources) defines reserved_name and
recipe_name payloads. Reserved sources retain role/kind/origin/context, without values or invented
ordinals. Actual prior/current recipe bindings retain their known spans and ordinals; detached
metadata retains only known locations. Product source locators and ordered scopes keep their own
validation. Localized rendering remains a product responsibility.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/reserved_diagnostic_review.py --mutations
```

Historical3624 payload cases, D131 rationale and nineteen compiled fault controls are preserved whole
in `docs/history/stitchcad-devnotes-part523.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part523.md`.
Independent approval remains unclaimed; static diagnostics supply no physical/runtime result.

## Ambiguous-name diagnostic sources

formula_ambiguous_name retains both actual ordered sources, including equal origins and known
metadata positions. Input positions never become invented recipe ordinals. A let collision precedes
RHS name lookup and retains actual let spans/ordinal when known. Product canonical source locators
keep their separate validation; no value/state/geometry read follows diagnostic construction.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/ambiguity_payload_contract.py --mutations
```

Historical4192 payload cases, examples and sixteen compiled fault controls are preserved whole in
`docs/history/stitchcad-devnotes-part524.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part524.md`.

| Example name | Role |
| --- | --- |
| `collision` | Authored measurement/parameter example name, never a reserved token |
| `eps_chord` | Invalid assertion-class example spelling; syntax refuses formula_parse |

## Whole recipe before execution

Whole source syntax/structure, all literal inputs and ordered name/kind inference finish in that
order before execution. A later syntax/input refusal prevents earlier inference; no accepted plan
prefix escapes. Static validity does not guarantee numeric availability or a computable result.
The reference's tuple plan is instrumentation; current [product whole recipe proofs](formula-checked-recipes.md)
retain actual source/identity/dependency metadata separately. D154's remaining reference ordinal
repair retains its live owner below.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py --mutations
python3 -I -B docs/tasks/artifacts/formula_structure/static_recipe_contract.py --mutations
```

Copied-book controls plan all selected effective inputs/outputs before stores activate, refuse unsafe
replacement/deletion candidates and recheck copies before replacement/use. Pure reference contracts
remain store-free; real replay retains the prepared environment and exact caller selectors. These
controls do not change the grammar or numerical evidence scope. The worked recipe has21 statements;
its13 separate refusals never count toward that plan. The normative statement ceiling remains4096.
Historical D119/D123/D148 examples, phase/locality controls and source receipts are retained whole in
`docs/history/stitchcad-devnotes-part526.md`; retrieve with
`python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part526.md`.
Reference provenance/runtime routing and physical geometry retain their separately owned proofs.

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

The status guard checks current scope statements and proof routes independently of the retired table.
It supplies no runtime/geometry proof and leaves D154 open:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/static_status_contract.py --mutations
```

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

Current coupled calibration compares26,723 shared cases through independent expectations,
reference code and actual public Rust APIs. Its complete fault population is9 local plus19
appended whole-recipe faults:28 Rust faults,with6 reference and3 copied-book controls. All pass
their original mismatch/body refusals,then restore source and current compiler artifacts.
The two computed-result faults now target the shared operand feeding kind stack and node
certificate;one copied-book fault targets the current implemented-status sentence. Only those
three data adapters change. All other program AST/API/plans/CLI/classifiers/reference data,
7 other local tuples and19 standing whole-factory tuples remain exact. The initial local-only
census undercount is corrected by verifying the full composition and runtime28-fault result.
Whole producer dispatch/guard verification also passes;its simulated children remain separate
from this real native/reference proof. Full pre-push/platform verification remains separately owned.

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
