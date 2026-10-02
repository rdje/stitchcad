# Annex: complete formula recipe inputs and identity

> **Status:** G1-SLICE.5a.3f.1b implements immutable normalized statements and complete recipes in
> sc-core. It converts input literals and preserves syntax metadata. Statement/recipe canonical
> serialization is implemented at .1c; name/type checking, binding and evaluation remain G1-SLICE.5 work.

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
two runnable API docs check source-independent usage. Twenty-one actual production serializer faults
must compile and fail public assertions, covering headers, operands, envelope/order/count/spacing,
empty identity, terminal newline, opaque Debug and exact extraction. The exclusive runner restores
its source exactly; its classifier refuses compiler/expect-only/passing-name noise. Existing authored
fixtures remain checked by the actual independent recursive reference; no independent whole-recipe
parser or numerical evaluation proof is claimed. Coupled review .3f.2 follows this implementation.

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
error causes. The exclusive runner restores all three sources exactly. Its failed-body classifier
refuses passing-name, compiler and expect-only noise; the structural suite watches actual anchors.
Authored fixtures are checked by the independent recursive reference, with semantic execution trapped.
Test-side header/operand byte composition is inspection, not a product statement/recipe serializer.

```bash
cargo test -p sc-core --test formula_normalized_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py
```

Numerical binding/evaluation, typed project hashes,
storage/recovery, geometry, command/API/MCP control and production approval retain their task owners.
This input API supplies none of those later results or approvals.
