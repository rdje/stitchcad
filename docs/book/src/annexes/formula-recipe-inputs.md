# Annex: complete formula recipe inputs and identity

> **Status:** G1-SLICE.5a.3f.1b implements immutable normalized statements and complete recipes in
> sc-core. It converts input literals and preserves syntax metadata. Statement/recipe canonical
> serialization is implemented at .1c. [Ordered name scopes](formula-name-scopes.md) and
> [whole recipe kind proofs](formula-checked-recipes.md) and numeric binding conversion are available;
> whole recipe execution remains pending.

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

Historical identity acceptance covers10 native contracts,21 compiled serializer faults, typed-domain
collisions, opaque Debug and exact extraction, including maximum flat shapes. Original complete
protocol/measurements are retained byte-for-byte in docs/history/stitchcad-devnotes-part500.md.
The public archive reader retrieves that record without Git or recursive unpacking:

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part500.md
cargo test -p sc-core --test formula_canonical_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/canonical_recipe_mutations.py
```

These receipts establish syntax/identity behavior, without numerical, browser or production authority.
Current source/provider/output planning and failed-body classification remain required by the linked
entry's local-environment profile; classifier-only mode starts no native child or mutable-source run.

## Owned authored recipe document

`FormulaRecipeDocument::new(source)` owns exact source text and its typed canonical identity after
complete parsing/literal conversion. Empty drafts accept. source() borrows original whitespace/text;
canonical() borrows identity bytes. Document equality retains exact spelling: 2.5 cm and25 mm can
have equal canonical identities while differently authored documents remain distinct.

normalized() reuses the actual parser/converter to return source-borrowed arenas with original spans;
it stores no self-reference and generates no spans from canonical bytes. Views cannot outlive the
document. Repeated calls preserve state; typed syntax/literal refusals propagate without fallback.
Private fields require a new validated replacement. Debug exposes byte counts, not customer text.

```rust
use sc_core::recipe::FormulaRecipeDocument;
let document = FormulaRecipeDocument::new("let width: length = 2.5 cm".into())?;
let view = document.normalized()?;
assert_eq!(&view.canonical_form(), document.canonical());
assert_eq!(view.statements()[0].name(), "width");
# Ok::<(), Box<dyn std::error::Error>>(())
```

This owns syntax, without proving names/kinds, executing operators or certifying Design readiness.
Duplicate/unbound names and incompatible kinds remain inspectable until static checking.

| API token | Meaning here |
| --- | --- |
| `FormulaRecipeDocument` | Immutable original text and typed syntax identity |
| `FormulaRecipeDocumentError` | Original Syntax or Literal refusal with nested source context |

## Verification scope

Eight input contracts check16 statement rows/nine sources and300 independent literal-role controls,
including later-statement errors, exact width/rounded-domain refusals and all original source metadata.
Maximum flat shapes, source/view privacy, Clone/drop and opaque diagnostics are covered. Seventeen
compiled production faults calibrate the original native bodies. Complete retained measurements,
plans, refusal/body controls and restoration receipts are in the same part500 record above; current
commands remain:

```bash
cargo test -p sc-core --test formula_normalized_recipe_contract
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py --classifier-only
python3 -I -B docs/tasks/artifacts/formula_structure/normalized_recipe_mutations.py
```

The input contracts retain unknown calls, duplicate/forward names, raw turns, wide literals,
incompatible annotations and unevaluated branches. They do not certify whole semantic execution.

## Coupled input and identity review

The review covers the [worked skirt](../spec/formula-language/examples.md)'s21 statements/25 operands
and13 refusals (3syntax,10beyond inputs). Later literal errors retain original spans. Input identity
proves no execution; current static/numeric foundations and remaining owners are mapped below.

### Coupled input runner modes and evidence

Five public controls and seven original compiled faults cover authored populations, metadata, exact
bytes, input refusal, combined limits and Debug. The complete original protocol, dispatch/physical
controls, current-assertion evidence and byte-exact restoration receipt are retained in
docs/history/stitchcad-devnotes-part504.md. Retrieval and current entry commands:

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part504.md
cargo test -p sc-core --test formula_recipe_input_review
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_mutations.py
```

Classifier-only mode declares no mutable source, diagnostic files or native child. Ordinary mode
retains the original seven filtered native commands; unselected tests receive no coverage credit.
These historical input proofs establish no performance, browser, geometry or release certification.

## Syntax milestone and the route to execution

Syntax/identity remain inspection, not garment execution. Source uses -x and x ^ 2; (- child) and
(^2 child) are canonical output. Display glyphs stay outside machine parsing.

The completed milestone's full test-family matrix, limits and proof scope are retained byte-for-byte
in docs/history/stitchcad-devnotes-part505.md. Current detailed contracts remain in the
[syntax](formula-syntax.md), [literal](formula-literals.md), [statement](formula-statements.md) and
[static-proof](formula-checked-recipes.md) annexes; the stage table below retains current remaining work.

```bash
python3 -I -B scripts/history_archive.py --root . read docs/history/stitchcad-devnotes-part505.md
```

### Implementation and remaining owners

| Stage | Owner | Current boundary |
| --- | --- | --- |
| Names/static kinds | G1-SLICE.5b | Whole-recipe proofs and coupled static review implemented |
| Exact arithmetic/bindings | G1-SLICE.5c | Scoped APIs implemented; coupled/platform verification remains tracked |
| Irrational functions | G1-SLICE.5d | Root/arc/sin/cos/inverse APIs available; tangent and complete coupled proof pending |
| Execution/replay | G1-SLICE.5e | Taken-only computation, uncertainty/tolerances and atomic results pending |
| Construction operations | G1-SLICE.5f | Typed operation/dependency bridge and G2/G3 selectors pending |
| Full formula acceptance | G1-SLICE.5g | Product examples/refusals and observed two-platform bytes pending |

Children in docs/tasks/G1-SLICE-recipes.md and its execution sibling own remaining work. Unknown
inputs gain no numeric fallback. Structural SizeSet inputs exist under D70; executed size mapping,
G4 policy, command atomicity/localization (.6), storage/digests/cycle loading (.7), API/MCP (.9),
browser execution (.12), G2/G3 geometry and G7 human approval remain separately required.

## Pure input-review and phase reference guards

These read-only references validate actual inputs before each provider/fixture read. Regular
same-volume files, including read-only hard links, are accepted; links, missing/outside paths and
nested Git boundaries refuse. Import exposes the library without activating stores or children;
caller environment and arguments remain unchanged. Diagnostic dispatch is separate from real CLI
behavior. Historical counts, fault sets and complete receipts remain in
`docs/history/stitchcad-devnotes-part508.md`, retrievable through history_archive.py read.

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_input_review_reference.py
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/whole_phase_contract.py --mutations
```

The in-memory mutation mode requires actual body assertion failures and exact unchanged source;
it writes no source/diagnostics and starts no child. This review grants no execution approval.
