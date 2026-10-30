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

## Owned original expression document

`FormulaExpressionDocument::new(source)` owns an original expression String and its existing typed
canonical expression identity. It reuses the actual expression parser,literal converter and
serializer. It supplies no fake let,name,header or statement ordinal. This is the source-ownership
prerequisite at .5e.2.5.derived.1;explicit source owners follow below. Canonical input joins follow below;namespace/kind
and complete dependency admission remain .derived.3.2.

| Public API | Meaning |
| --- | --- |
| `FormulaExpressionDocument` | Immutable original expression source and typed canonical syntax |
| `FormulaExpressionDocumentError` | Original expression Syntax or Literal refusal |

source() returns original whitespace/text;canonical() borrows the typed identity. Exact-source
equality remains separate from normalized identity:2.5 cm and25 mm share one identity but retain
distinct authored documents. Cloned replacement documents do not mutate the original. Debug
shows source/canonical byte counts,not customer names or text.

normalized() derives immutable source-borrowed arenas through the same parser/converter. It keeps
original local spans and cannot outlive the document. It stores no self-referential arena and
never reconstructs source positions from identity bytes. Repeated views preserve source/identity.

```rust
use sc_core::recipe::FormulaExpressionDocument;
let document = FormulaExpressionDocument::new(" (2.5 cm + eps_num) ".into())?;
let view = document.normalized()?;
assert_eq!(&view.canonical_form(), document.canonical());
let span = view.root().span();
assert_eq!(document.source().get(span.start()..span.end()), Some("(2.5 cm + eps_num)"));
# Ok::<(), Box<dyn std::error::Error>>(())
```

An empty source is not an expression;its original syntax refusal is retained. A let header is not
an expression either. Original parser failures precede literal normalization. Every literal,
including an untaken branch's,retains its existing width/domain check and source location. Errors
retain the original typed cause through Syntax/Literal and the standard Error source chain.

The document performs no name/kind checking or operator evaluation. Missing names,division by
zero,negative roots or incompatible kinds can therefore remain syntactically inspectable until
actual checking/execution. It grants no semantic entity id,canonical-input join,namespace proof,
dependency/cycle admission,current Design,computed value or readiness. Canonical Derived formula
ids remain references requiring real source integration. The existing Formula(id) amount adapter
continues to use actual DesignParameter metadata;this document changes no such identity contract.

Four focused native contracts compare actual source/span/alias/replacement behavior and original
syntax/literal causes,including complete branch literals and syntax-only numerical failures.
Three documentation tests include actual private-field E0616 and borrowed-view E0515 refusals plus
the positive example. Existing parser/literal/kind/numeric obligations retain their own checks;
these document controls do not independently recertify their algorithms or Derived execution.

## Explicit formula source owners

`FormulaSource::new(id, expression)` retains an explicitly assigned EntityId and the actual owned
expression document. Ids come from the existing entity-creation boundary;they are never generated
from expression text,canonical bytes or list position. Equal bodies can belong to distinct entities.
A source has no invented variable name,output annotation,let header or stored result.

`FormulaSources::new(sources, limits)` takes the complete ordered vector without cloning original
or canonical buffers. It publishes an immutable registry only after complete content admission and
unique-id checking. sources() borrows authored order;source(id) returns that exact original owner
through a sorted index. Missing ids return None;labels,canonical aliases and neighboring ids supply
no fallback. Private fields prevent rebinding through either shared view. Debug omits source text.

| Public API | Meaning |
| --- | --- |
| `FormulaSource` | Explicit source entity id and actual original expression document |
| `FormulaSources` | Complete immutable ordered inventory and exact-id lookup |
| `FormulaSourceLimits` | Explicit source-count,original-byte and canonical-byte allowances |
| `FormulaSourceResource` | Sources, OriginalBytes or CanonicalBytes population |
| `FormulaSourcesError` | Complete quota/overflow,duplicate identity or allocation refusal |

The three limits are independent. Original bytes include whitespace;canonical bytes count the
separately retained typed identities. Every record counts,including an unused source or a repeated
body. Count also bounds one index record per source. Zero allowances admit an empty registry.
Construction checks count,checked byte sums,original-byte allowance and canonical-byte allowance
before fallibly reserving that single index. Its in-place sort changes no source order or body.
DuplicateIdentity retains the actual id,first authored position and earliest authored duplicate,
even when numeric id order differs. Identical duplicate bodies still refuse. No accepted prefix
escapes any refusal;Allocation retains the original TryReserveError in the standard error chain.

```rust
use sc_core::{ontology::EntityId, recipe::{FormulaExpressionDocument, FormulaSource,
    FormulaSourceLimits, FormulaSources}};
let expression = FormulaExpressionDocument::new(" 2.5 cm + eps_num ".into())?;
let limits = FormulaSourceLimits::new(1, expression.source().len(),
    expression.canonical().as_str().len());
let id = EntityId::from_bits(9001); // explicit fixture identity, not a production allocator
let sources = FormulaSources::new(vec![FormulaSource::new(id, expression)], limits)?;
assert_eq!(sources.source(id).unwrap().expression().source(), " 2.5 cm + eps_num ");
assert!(sources.source(EntityId::from_bits(9002)).is_none());
# Ok::<(), Box<dyn std::error::Error>>(())
```

These are retained-content/index allowances after caller-owned document creation. They do not
bound that earlier allocation,parser scratch,allocator overhead,numeric kernels,CPU or graph work.
Admission policy is not stored canonical content. No canonical source-state/value cache is added.

This inventory alone proves no global Design identity disjointness,namespace/kind validity,
canonical Derived join,acyclic dependencies,currentness or evaluated result. For example,
missing+1 mm/0 stays inspectable source;actual static checking still refuses its missing name.
Canonical Derived { formula } needs a real source join before execution. Existing quantity
Formula(id) continues to resolve actual DesignParameter metadata and its current canonical target
([sewing amounts](../spec/design-inputs.md#actual-sewing-amount-declarations));source9001 cannot
impersonate parameter5001. No extra rounding boundary is introduced for an owned expression.

Four focused native contracts cover original buffers/spans/order,exact-id versus canonical alias,
separate complete quota boundaries,empty drafts,source-only static refusals and an independent
prefix-scan comparison over all256 four-record sequences of four ids. Three documentation guards
reject default limits,private-registry mutation and id rebinding. Existing parsing/kind/numeric
algorithms and full dependency execution retain their separate proof obligations.

## Join actual canonical Derived sources

`FormulaSourceInputs::new(&inputs, &sources)` checks the complete actual CanonicalInputs and
FormulaSources before lending any accepted scope. It allocates no workspace,clones no state,reads
no authored value and parses/evaluates no expression. Both owners stay borrowed and immutable.
This is source ownership admission;namespace,result-kind,dependency/cycle and execution proofs
remain required at .5e.2.5.derived.3.2 and its later execution owners.

| Public API | Meaning |
| --- | --- |
| `FormulaSourceInputs` | Complete immutable canonical/source admission over actual owners |
| `FormulaDerivedInput` | Actual Derived target/declaration and its exact expression owner |
| `FormulaSourceInputError` | Original owners,target family/kind/position and refusal |
| `FormulaSourceInputRefusal` | IdentityCollision source position or MissingSource formula id |

The complete identity pass comes first. Every owned canonical declaration id must be disjoint
from every source entity id,including unused entities and non-Derived declarations. IdentityCollision
keeps the original source index plus canonical target/position. This checks these two inventories;
other Design/metadata/host identities still need the complete containing ownership census.

Only then does admission visit every actual Derived state in dedicated-Length order followed by
general-scalar order. Its formula field must select an actual expression owner. MissingSource
retains that original id and the real canonical target/kind/position,even after valid earlier joins.
No accepted prefix escapes. The separate canonical source/provenance field is never interpreted
as a formula id. Evidence,assumption,preference and observation ids remain references;they are not
mistaken for additional owned canonical declarations. Known zero,false,Preference and Unknown
states need no formula source and remain unchanged.

derived_inputs() lends all and only Derived targets in original family order,also in reverse.
Each FormulaDerivedInput exposes position(),target() and source();length()/scalar() borrow the
actual dedicated/general canonical record. General Length remains general Length. Several targets
can share one source without losing distinct ids,positions or declared kinds. A shared source still
needs actual output-kind checks for each consumer;source existence supplies no such proof.

```rust
use sc_core::{ontology::EntityId, recipe::*, value::*};
let formula = EntityId::from_bits(9001);
let declaration = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(10), source: EntityId::from_bits(20),
    state: LengthState::Derived { formula },
})?;
let inputs = CanonicalInputs::new(vec![declaration], vec![])?;
let expression = FormulaExpressionDocument::new("1 mm".into())?;
let limits = FormulaSourceLimits::new(1, expression.source().len(),
    expression.canonical().as_str().len());
let sources = FormulaSources::new(vec![FormulaSource::new(formula, expression)], limits)?;
let joined = FormulaSourceInputs::new(&inputs, &sources).expect("actual complete joins");
let derived = joined.derived_inputs().next().expect("actual Derived target");
assert_eq!(derived.target(), CanonicalInputTarget::Length(EntityId::from_bits(10)));
assert_eq!(derived.source().id(), formula);
assert!(derived.length().unwrap().authored_value().is_err()); // Still requires actual evaluation.
# Ok::<(), Box<dyn std::error::Error>>(())
```

Errors retain inputs()/sources() as actual borrowed contexts as well as target()/position()/refusal().
They cannot outlive those owners;handle a local join error while its context exists rather than
erasing it into a static error box. Debug/Display omit expression text and authored values.
No current Design/revision,evidence truth,resource envelope,rounding or computed result is created.
Quantity Formula(id) keeps its existing parameter-backed meaning. A source body such as
missing+1 mm/0 remains inspectable until real static checking and execution;joining it does not
make its missing name,output kind or operation valid.

Four native contracts check actual canonical/source pointers,both representation families and all
six scalar kinds,shared-source consumers,original late failures,complete ownership precedence,
non-Derived states/provenance and empty drafts. Five existing canonical-input contracts retain
actual-owner/family/kind/read boundaries. Four documentation tests include the positive example,
private-field E0616 and E0515 refusals for escaping either actual owner. These scoped checks
certify source joins;complete graph/evaluation/platform verification remains pending.

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
