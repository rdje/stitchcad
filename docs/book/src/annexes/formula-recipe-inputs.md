# Annex: complete formula recipe inputs and identity

> **Status:** G1-SLICE.5a.3f.1b implements immutable normalized statements and complete recipes in
> sc-core. It converts input literals and preserves syntax metadata. Statement/recipe canonical
> serialization is implemented at .1c. [Ordered name scopes](formula-name-scopes.md) and
> [whole recipe kind proofs](formula-checked-recipes.md) are available; numerical binding and evaluation remain later work.

The [statement parser](formula-statements.md) records what you wrote. Input normalization then
converts every literal to the [canonical internal units](formula-literals.md), without calculating
operators or deciding what names mean. You can use it before a change review or later validation;
a successful conversion does not certify a garment dimension or a closure check.

## Normalize one statement

```rust
use sc_core::recipe::{FormulaStatement, FormulaNormalizedStatementKind};

let source = String::from("let width: length = 2.5 cm");
let syntax = FormulaStatement::parse(&source)?;
let normalized = syntax.normalize_literals()?;
drop(syntax);
assert_eq!(normalized.name(), "width");
if let FormulaNormalizedStatementKind::Let { declared_kind, expression } = normalized.kind() {
    assert_eq!(declared_kind.token(), "length");
    assert_eq!(expression.canonical_form().as_str(), "length:25000");
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

FormulaNormalizedStatement owns its normalized expression arenas and borrows the original source.
It outlives the parsed statement allocation, while source must remain alive. Its private fields
prevent replacing a name, annotation or arena. Clone retains source borrowing and all metadata.
The returned kind/operand views cannot outlive the normalized owner.

FormulaNormalizedStatementKind has Let with declared_kind/expression and Assert with tolerance/
left/right. All six declared kinds and five symbolic tolerance names are preserved exactly. The
kind annotation is not inferred from the expression, and a tolerance name supplies no numeric value.
For example, a count expression under a boolean declaration remains inspectable here; later static
validation owes the dimensional refusal. Wide literal children retain their u128 magnitudes rather
than being narrowed to the i64 numeric binding form.

## Normalize a complete ordered recipe

```rust
use sc_core::recipe::{FormulaRecipe, FormulaNormalizedStatementKind};

let source = "let width: length = 25 mm\nassert closure: eps_geo = width == 2.5 cm";
let syntax = FormulaRecipe::parse(source)?;
let normalized = syntax.normalize_literals()?;
drop(syntax);
assert_eq!(normalized.statements().len(), 2);
assert_eq!(normalized.statements()[1].name(), "closure");
if let FormulaNormalizedStatementKind::Assert { tolerance, left, right } =
    normalized.statements()[1].kind()
{
    assert_eq!(tolerance.token(), "eps_geo");
    assert_eq!(left.canonical_form().as_str(), "width");
    assert_eq!(right.canonical_form().as_str(), "length:25000");
}
assert!(FormulaRecipe::parse(" \n\t")?.normalize_literals()?.statements().is_empty());
# Ok::<(), Box<dyn std::error::Error>>(())
```

FormulaNormalizedRecipe owns an immutable slice of normalized statements in authored order.
Empty recipes remain empty. Duplicate or forward declarations are retained, with no sorting,
merging or implicit name binding. Conversion visits each statement in order and assertion operands
left then right. It reuses the existing expression conversion, including every call argument and
all conditional branches. An invalid literal in an untaken branch therefore refuses input conversion;
this does not change the separate rule that numerical evaluation later executes only a taken branch.

No partially normalized statement or recipe escapes on failure. Already-built temporary arenas are
dropped; the original parsed syntax is unchanged and remains available for inspection or correction.
The original256-node-per-expression,4096-statement and16-if-depth bounds remain in force. There is
no new aggregate node cap, recursive traversal or implicit numerical rounding/binding step.

## Original source locations remain available

| Surface | Meaning after normalization |
| --- | --- |
| Statement span | Complete original statement, excluding outer whitespace |
| Name span/name | Exact original identifier extent and borrowed text |
| Annotation span | Original declared kind or tolerance token |
| Expression/node spans | Original complete-source byte locations, including grouping |
| Literal number/unit | Original decimal spelling and closed unit token |
| Literal kind/magnitude | Converted canonical input, retaining the existing width/domain rules |

Source spelling and positions are distinct from canonical expression identity. A cloned normalized
recipe still borrows the original names and literal text. Explicit inspection exposes customer data.
Debug of recipes/statements/role views/arenas exposes shape and positions, omitting customer names,
raw numbers and literal magnitudes.

## Conversion refusals keep operand and statement context

FormulaStatementLiteralError exposes expression_part, literal_error, span and diagnostic_code.
The part is Binding, AssertionLeft or AssertionRight. The nested FormulaLiteralError retains its
exact rule, source span, rational-width limit token/bound and actual domain witness.
FormulaRecipeLiteralError adds statement_index and statement_error. Its index is always a known
1-based ordinal because this operation consumes an already accepted syntax recipe; the parser's
global non-ASCII refusal remains a different error with no identified statement index.

```rust
use sc_core::recipe::{FormulaRecipe, FormulaStatementExpression};

let syntax = FormulaRecipe::parse(
    "let width: length = 25 mm\nassert closure: eps_geo = width == (1000.000001 m)"
)?;
let result = syntax.normalize_literals();
assert!(result.is_err());
let error = result.expect_err("one micrometre exceeds the scalar 1 km domain");
assert_eq!(error.statement_index(), 2);
assert_eq!(error.statement_error().expression_part(), FormulaStatementExpression::AssertionRight);
assert_eq!(error.diagnostic_code(), "formula_domain");
assert_eq!(syntax.statements().len(), 2);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The scalar length domain is1 km; the10 m piece bounding box needs geometry context and is not an
input-literal limit. A width refusal occurs before rounding, while scalar length refusal checks the
rounded input. The [literal contract](formula-literals.md) supplies exact boundary examples.

Standard Error source chaining is recipe → statement operand → original literal error. The recipe
error boxes its nested cause to keep the returned error compact; this allocation occurs only on
refusal. Recipe errors are Clone; standalone/literal errors retain Copy. Display/Debug omit customer
names and original source spelling. Numeric bound/measured witnesses remain deliberately available
as structured diagnostic data. Human localization and the full command argument envelope belong to .6.

## Own canonical statement and recipe identity

After all input literals convert successfully, canonical_form produces owned identity bytes.
Use this when comparing authored inputs across source formatting or unit aliases. It retains the
statement name, annotation and every operand in order, without evaluating their numeric result.

```rust
use sc_core::recipe::FormulaStatement;

let identity = {
    let source = String::from("let width: length = 2.5 cm");
    FormulaStatement::parse(&source)?.normalize_literals()?.canonical_form()
};
assert_eq!(identity.as_str(), "(bind width length length:25000)");
let alias = FormulaStatement::parse(" let width : length = ((25 mm)) ")?
    .normalize_literals()?.canonical_form();
assert_eq!(identity, alias);
# Ok::<(), Box<dyn std::error::Error>>(())
```

FormulaCanonicalStatement owns a private String. FormulaCanonicalRecipe does the same for the
complete authored sequence. Both outlive the source and every syntax/normalized arena. Their public
as_str returns explicit customer-bearing bytes; into_string transfers the owned text. Editing a
transferred clone cannot change the original identity. Clone and Eq compare complete exact bytes;
Debug reports byte_count and omits customer names and literal values.

```rust
use sc_core::recipe::FormulaRecipe;

let identity = {
    let source = String::from("let n: count = 1\nassert check: eps_num = n == 1");
    FormulaRecipe::parse(&source)?.normalize_literals()?.canonical_form()
};
assert_eq!(identity.as_str(),
    "(recipe (bind n count count:1) (assert check eps_num n count:1))");
assert_eq!(FormulaRecipe::parse(" \n\t")?.normalize_literals()?.canonical_form().as_str(), "(recipe)");
let mut editable = identity.clone().into_string();
editable.clear();
assert!(!identity.as_str().is_empty());
# Ok::<(), Box<dyn std::error::Error>>(())
```

A recipe always has its wrapper, even with one statement. The serializer uses the exact
[grammar §4.1 templates](../spec/formula-language/grammar.md#41-statement-and-whole-recipe-bytes):
one ASCII space between parts, no outer padding or terminal newline. Both assertion operands remain
flat children, while comparisons inside an operand keep their expression nodes. Empty and
whitespace-only recipes share (recipe). Reordering statements, changing an annotation or swapping
operands changes identity. Duplicate/forward declarations stay present; no sorting or merging occurs.

Expression, statement and recipe identities are distinct Rust types. Their raw text can overlap:
the ordinary expression bind(width, length, 25 mm) has the same bytes as the binding statement
let width: length = 25 mm. The expression recipe(bind(n, count, 1)) has the same bytes as a
one-statement recipe. Cross-type equality is unavailable. Persistence must frame the known type
in each field/digest domain; these bytes supply no universal reader, project version or hash namespace.

Serialization iterates statements and reuses the existing iterative expression serializer. It
introduces no new scalar conversion or structural limits, and preserves raw turns, wide literal
children, unknown names/calls, unevaluated branches and incompatible declared/input kinds. A syntax
or input-conversion refusal has no successfully normalized owner from which to produce whole identity.
Name/type/binding validation, tolerance resolution and numerical execution remain separate steps.

Ten public identity contracts check sixteen authored statements/nine complete sources, all55
independently authored expression rows through all three operand roles (165 controls), and100
independent Decimal/Fraction rows through those roles (300 controls). The actual four normative
book/decision examples, alias equality/order distinctions, two actual cross-type text collisions,
ownership/privacy/extraction and large sources are covered. On64KiB stack, the full4096-statement,
two256-node/16-if-level operands per assertion serialize to exact independently composed bytes;
100000-character names,50000 grouping levels and deepest/widest calls also retain exact identity.
These are flat-shape/byte contracts, not a browser, performance or production certification.

Five compile-fail docs check private construction and all three cross-domain equality pairings;
two runnable API docs check source-independent usage. Twenty-one actual serializer faults compile
and fail public assertions, covering headers, operands, envelope/order/count/spacing, empty identity,
terminal newline, opaque Debug and exact extraction. The direct entry plans eight inputs before
effects: implementation/current test/four imported TSVs/book grammar/byte decision. Ordinary mode
declares twenty-two diagnostic files and one mutable source; classifier-only declares zero files
or mutable sources and invokes no native child. Both modes preserve prepared local stores.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py --classifier-only
```

A failed-body thread panic must name a current assertion macro path/line/byte column. Custom messages
pass; nonthread mentions, passing names and labelled extraction panics refuse. Two independent
complete captures/eighteen physical refusals and twenty-four mode omissions preserve all21 original
cases/full-target arguments/restoration; an independent legacy-regex fault distinguishes nonthread
noise. Copied classifier-only mode accepts linked readonly input/unused log aliases without source/
output writes. Every actual failed body reaches a current macro, including small-stack workers;
independently authored failed-test sets remain exact. Thirty-one frozen inputs/eighteen unchanged
Rust files/six compile-time inputs/twenty-two local single-link logs and the current full ten-test
executable restore exactly. Dispatch captures and real native behavior are verified separately.
Existing fixtures retain their independent recursive-reference checks; no independent whole-recipe
parser or numerical evaluation proof is claimed. [Coupled review .3f.2](#coupled-input-and-identity-review)
is complete below.

```bash
cargo test -p sc-core --test formula_canonical_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py
```

## Verification scope

Eight public contracts check sixteen independently authored statement/header/operand-byte rows,
nine whole sources, original spans/source borrowing/privacy/Clone and exact later-statement failures.
One hundred independent Decimal/Fraction literal rows pass through all three operand roles, supplying
300 whole-recipe input controls. Both width and rounded scalar refusals retain their typed families.
Earliest refusal/source chaining, unknown calls, duplicate/forward names, raw turns/wide magnitudes,
incompatible annotations and unevaluated operators retain the stated input-only scope.

On a64KiB thread stack,4096 assertions each hold two256-node operands with16 if levels. Parsing,
conversion, syntax drop, normalized Clone and drop preserve all2097152 nodes. This checks the declared
flat shape and separate bounds for that maximum; it is not a memory-performance or browser certificate.
Six compile-fail docs check private fields/source/view lifetimes, and two runnable API docs check usage.

Seventeen actual production faults compile and fail public assertions: name/header metadata,
declared kind/tolerance, operand coverage/error role, order/count/ordinal, spans, privacy and both
error causes. The direct entry plans seven inputs before effects: three implementations, current
public test and its three imported TSVs. Ordinary mode declares eighteen logs and three mutable
sources; classifier-only declares zero files or mutable sources and invokes no native child. Both
modes retain local store/directory preparation.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py --classifier-only
```

A qualifying failed-body thread panic must name a current assertion macro path/line/byte column.
Custom messages pass; passing names, labelled extraction panics and nonthread location mentions
refuse. Two independent complete captures, sixteen physical refusals and actual input/file/mutable/
mode/stores/restoration/classifier faults calibrate the plans, including each of the three mutable
declarations. A copied classifier-only entry accepts linked read-only input and ignores unused log
aliases without source/output writes. Seventeen original full-target faults reach current macros
in every failed body, including the small-stack worker; independently authored failed-test sets and
source bindings remain exact. Twenty-seven frozen inputs/seventeen unchanged Rust files/three
compile-time TSVs/eighteen local single-link logs and the current full eight-test executable restore
exactly. Dispatch capture and real native behavior are verified separately.
Authored fixtures are checked by the independent recursive reference, with semantic execution trapped.
Test-side header/operand byte composition is inspection, not a product statement/recipe serializer.

```bash
cargo test -p sc-core --test formula_normalized_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py
```

## Coupled input and identity review

G1-SLICE.5a.3f.2 checks the complete path from authored statements to normalized inputs and owned
identity. The [worked skirt examples](../spec/formula-language/examples.md) supply 17 bindings and
four assertions:21 statements with 25 expression operands. Independently authored complete statement
bytes match the actual published names/kinds/operands and the complete ordered recipe. The recursive
reference checks syntax with inference/evaluation trapped, plus complete ordered token coverage;
authored statement chunks do not supply an independent whole-recipe parser.

The 13 refusal examples also have a current stage. The missing conditional branch, text value and
unsupported cube refuse during syntax parsing. The other 10 still produce input identity: their
kind/name/division/unknown/tolerance/envelope checks belong to later static validation and evaluation.
Accepting their syntax and inputs does not accept their use in a garment. A bad literal in an
untaken branch or a call argument still aborts whole normalization, retaining the original span,
known statement ordinal 22 and operand role after the 21 valid worked statements.

| Obligation | Current evidence | Remaining implementation owner |
| --- | --- | --- |
| Whole let/assert syntax, order and fixed bounds | Statement/recipe contracts and coupled maximum/first excess tests | Syntax milestone G1-SLICE.5a.4 complete |
| All literal inputs, including untaken branches | Literal/normalized contracts and later whole-input refusal tests | Numeric binding G1-SLICE.5c |
| Names, annotations and original global metadata | 21 worked statements, source borrowing and Clone checks | Namespace/type acceptance G1-SLICE.5b |
| Exact typed identity, empty/order/alias behavior | Owned identity contracts, actual worked recipe and typed collision controls | Typed project fields/digests G1-SLICE.7 |
| Source/arena-independent identity, privacy and extraction | Ownership contracts and actual statement Debug controls | Command diagnostic envelope G1-SLICE.6 |
| All 13 worked refusals | Three syntax refusals; ten semantic checks explicitly deferred | Static validation/evaluation G1-SLICE.5b/.5e |
| Published values/assertion verdicts, lazy execution and replay | Independent reference only; inputs/identity compute no values | Product evaluator/DAG G1-SLICE.5e |
| Geometry selectors and construction | Syntax/identity retain ordered selector calls | G1-SLICE.5f and G2-2D |
| Storage, API/MCP and production acceptance | Distinct future workflow/approval requirements | G1-SLICE.7/.9 and G7-RELEASE |

Five new public controls check the actual worked/refusal populations, metadata and complete bytes,
whole later literal failures, alias/order invariants and combined limits of 4096 statements, 256 nodes and 16 nested conditionals
with first excesses. The bounded maximum serializes on a 64 KiB stack. Existing tests additionally
exercise all 4096 assertions with both 256-node operands; neither case is a performance/browser proof.
### Coupled input runner modes and evidence

Seven original production faults target metadata, missing later inputs, flattened refusal index,
recipe order, assertion operand identity, combined bound and normalized Debug privacy. Each compiles
and fails at a current guarded public assertion macro. The direct entry declares eight inputs before
activation or reads: all four implementations, current test source, complete worked-statement TSV,
the actual example chapter and refusal-stage TSV. Ordinary mode declares all seven numbered fault
logs plus restored.log and all four mutable implementations. Classifier-only declares no diagnostic
files, mutable source or native child; caller-provided valid local stores and unrelated values survive.

Complete mode captures verify original case tuples, source order/bindings, exact Cargo arguments,
filters, diagnostic bytes, per-case restoration and final byte checks before the full rebuild.
Twenty-eight actual omissions and the legacy nonthread predicate fail independently. Physical
controls refuse aliased endpoint logs, each hard-linked mutable source and missing late inputs
before reads or effects. An actual copied classifier-only entry accepts an input with three hard
links and two unused log aliases without source/output writes or child dispatch; all inputs, the
alias victim and environment remain exact. These captures establish dispatch; real compilation
supplies the native evidence separately.

Failed-body classification requires a thread panic at a current source/path/line/UTF-8 byte-column
assertion macro. Custom macro messages qualify; compiler-note locations, compiler/expect noise
and passing test names do not. D197 closes only after expression, reduction and input families
verify their original controls and current artifacts. D198 removes an obsolete statement-availability
phrase from the expression inventory; its expression fixtures and renderer remain unchanged.
Current inventory/reference and all four original interpreter renderer faults pass after repair.

The original seven input Cargo filters are retained, including their original substring semantics;
each currently selects exactly one public test. The four filtered tests per fault were not executed
and receive no coverage credit. Independent review inspects every actual failed body, including
small-stack worker evidence, against current quoted-thread macro locations. Eight local single-link
logs, seventy-five frozen inputs, sixty-three unchanged production Rust sources, all four mutable
implementations and the unchanged five-test source restore exactly. The full target passes after
restoration, and a newly Cargo-selected executable independently runs all five current tests.
The structural suite watches authored/reference populations and original actual anchors; this
proof supplies no numerical execution or browser/performance authority.

```bash
cargo test -p sc-core --test formula_recipe_input_review
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py
```

This closes the scoped normalization/identity review. The full syntax milestone .5a.4 is complete;
static kind proofs are implemented; numerical implementation remains .5c–.5g, as mapped below.
The formula contract §9's product numeric/cross-platform and
refusal requirements remain open; this review grants no execution, geometry or release approval.

Numerical binding/evaluation, typed project hashes,
storage/recovery, geometry, command/API/MCP control and production approval retain their task owners.
This input API supplies none of those later results or approvals.

## Syntax milestone and the route to execution

The syntax milestone G1-SLICE.5a.4 closes syntax, literal inputs and canonical identity. A recipe
can now be inspected and compared reproducibly. It still cannot compute garment dimensions.
The source you author uses -x and x ^ 2; (- child) and (^2 child) are canonical serializer output,
with child standing for the operand. Display glyphs remain presentation, outside machine parsing.

The proof map below names public test families under crates/sc-core/tests/. Their independently
authored fixtures/reference checks verify these scoped obligations. Prior exclusive fault runners
supply actual compiled assertion failures; the milestone reruns the seven coupled input faults
and confirms exact source restoration. It does not claim to rerun every earlier fault family.

| Requirement | Public contract family | Reviewed scope |
| --- | --- | --- |
| ASCII, identifier spelling, three keywords, longest operators | formula_lex_contract | Borrowed tokens, full-source preflight and exact lexical rules |
| Decimal syntax, closed units and one-space separator | formula_expression_contract | Original spelling/span; display and exponent notation refused |
| Precedence, associativity, square payload, comparisons | formula_expression_contract | Explicit ordered trees; non-chaining and unsupported exponent |
| Ordered calls and all three conditional children | formula_expression_contract | Syntax only; function names/types remain unchecked |
| Source borrowing, privacy, clone/drop and flat limits | formula_lex_contract / formula_expression_contract | Located refusals, 256 nodes and 16 conditional levels |
| Exact input conversion, reduction, width and scalar domains | formula_literal_contract | Independent Fraction/Decimal rows and pathological spellings |
| Every literal position, aliases, raw turns and atomic failure | formula_normalized_contract | Unevaluated whole arena with original metadata |
| Every expression role, operator/order and exact owned bytes | formula_canonical_contract | Typed identity, explicit extraction, no simplification |
| Six binding annotations, five assertion classes and operands | formula_statement_contract | Complete headers/global spans; no binding or tolerance resolution |
| Empty/ordered statement boundaries and 4096 bound | formula_recipe_contract | Full-source consumption; later errors and combined maxima |
| Every statement input and contextual normalization refusal | formula_normalized_recipe_contract | Both assertion operands; known ordinal and nested Error chain |
| Owned statement/recipe identity and typed text collisions | formula_canonical_recipe_contract | Empty wrapper, ordered metadata/operands and domain distinction |
| Actual worked/refusal population and all first excesses | formula_recipe_input_review | 21 statements/25 operands; 3 syntax and 10 deferred semantic refusals |

The machine/display/canonical distinctions, span availability and all four language limits remain
specified by the [grammar](../spec/formula-language/grammar.md) and
[contract](../spec/formula-language.md). Rational width128 applies to literal conversion now;
exact result width and numeric binding still require execution implementation. Privacy checks cover
Debug/error shape; explicitly requesting canonical bytes reveals customer formula content.
The full milestone runs native fmt/clippy/tests, the three-library WASM build, every diagnostic
probe suite and the doctrine gate. WASM compilation is not a real-browser execution certificate.

### Remaining implementation owners

| Stage | Owner | Required result before closure |
| --- | --- | --- |
| Names and static kinds | G1-SLICE.5b | All origins/reserved names/signatures; validate both branches before computing any value |
| Exact arithmetic and bindings | G1-SLICE.5c | Reduced rational width, per-result domains, explicit quantization and once-rounded signed storage |
| Irrational functions | G1-SLICE.5d | True-value nearest-quantum result, principal branches/poles and algorithm-independent proof |
| Execution and replay | G1-SLICE.5e | Taken-only computation, uncertainty/tolerances, atomic results, declaration order and dependency validation |
| Construction operations | G1-SLICE.5f | Complete typed v1 operation list, prior geometry dependencies and selectors through G2/G3 contracts |
| Full formula acceptance | G1-SLICE.5g | Product examples/refusals/fixture agreement and observed two-platform byte reproducibility |

Each stage has owned children in docs/tasks/G1-SLICE-recipes.md. Static obligation review,
scoped checks and [whole recipe proofs](formula-checked-recipes.md) are implemented; coupled whole
review .5b.4b is complete; arithmetic .5c follows.
Unknown inputs retain declared kinds but no invented numeric value. Factory artifact blocking
remains G4's policy; size-axis integration still waits for D70. Command atomicity/localization (.6),
typed project fields/digests and cycle loading (.7), API/MCP (.9), browser execution (.12), geometry
(G2/G3) and human production approval (G7) remain separately required. Closing syntax closes none
of these later obligations.
