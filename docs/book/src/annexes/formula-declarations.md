# Formula declaration metadata

> **Status:** implemented vocabulary and immutable sourced declarations at G1-SLICE.5b.2a/.2b in sc-core::recipe.
> Checked initial namespaces are available at .2c.2; ordered name reads/bindings remain .2d. A recipe has no product
> static acceptance or execution. [Reference static review](formula-static-validation.md) is complete
> for its stated instrument populations and has a separate proof scope.
> [Reserved-name diagnostic sources](formula-static-validation.md#reserved-name-diagnostic-sources)
> are specified and independently exercised in the reference before product namespace implementation.

The checker needs to know what a name means before fetching its value. An unknown measurement
can still be length. A size context can be absent while size_index still has the kind count.
The vocabulary describes metadata without numbers, input states or value providers. Sourced
declarations carry identities or borrow existing canonical records; their metadata queries read
no state or numerical value.
The normative populations are [contract §§2–3](../spec/formula-language.md#2-values-and-kinds).

## Kinds and origins

FormulaKind has exactly eight variants: Length, Angle, Area, Ratio, Count, Boolean, Point and Edge.
Each has an exact lowercase token. FormulaKind::ALL enumerates the complete table in its authored
order; from_token recognizes an exact token without trimming, aliases or case repair.

The six scalar kinds convert to/from the existing FormulaBindingKind statement annotations.
Point and Edge have no binding_kind: they are references to geometry that operations own.
These conversions do not infer an expression's kind or validate any operation's signature.

```rust
use sc_core::recipe::{FormulaBindingKind, FormulaKind};
assert_eq!(FormulaKind::from(FormulaBindingKind::Angle), FormulaKind::Angle);
assert_eq!(FormulaKind::Point.binding_kind(), None);
assert_eq!(FormulaKind::from_token("Length"), None);
```

FormulaOrigin has exactly nine variants: Measurement, Ease, Parameter, Profile, Material, Geometry,
Recipe, Size and Tolerance. Its ALL/token/from_token APIs mirror the kind vocabulary. An origin
identifies the semantic source domain. It does not certify a source record, a value or evidence.

## Reserved names and their contexts

FormulaReservedName represents the existing five FormulaToleranceName values plus SizeIndex,
SizeCount and IsBaseSize. Its eight-name population agrees with MachineToken's reserved input
classification. Only the three grammar words let/assert/if remain reserved identifiers; loop,
repeat, while, fn and macro remain ordinary names under the approved current grammar.

| Name | Kind | Origin | Required context | Tolerance role |
| --- | --- | --- | --- | --- |
| eps_num | length | tolerance | Always | Numerical |
| eps_geo | length | tolerance | Always | Geometric |
| eps_fmt | length | tolerance | Export | Format |
| eps_imp | length | tolerance | Export | Importer |
| eps_phys | length | tolerance | Profile | Physical |
| size_index | count | size | Size | none |
| size_count | count | size | Size | none |
| is_base_size | boolean | size | Size | none |

FormulaReservedContext describes the required value provider. It is distinct from FormulaOrigin:
eps_phys has tolerance origin and requires a profile to supply its value. Always describes the
language-defined classes; the metadata API does not fetch even those classes' numerical values.
Optional contexts stay declared when absent. Runtime missing-value diagnostics remain later work.

```rust
use sc_core::recipe::{FormulaKind, FormulaOrigin, FormulaReservedContext, FormulaReservedName};
let name = FormulaReservedName::from_token("eps_phys").unwrap();
assert_eq!(name.kind(), FormulaKind::Length);
assert_eq!(name.origin(), FormulaOrigin::Tolerance);
assert_eq!(name.required_context(), FormulaReservedContext::Profile);
assert_eq!(FormulaReservedName::SizeIndex.tolerance_name(), None);
```

A reserved spelling is a valid reference token and may not be rebound. Initial namespace collisions
are checked below; ordered reads/forward-reference and whole-recipe checks remain .5b.2d and .3/.4. Context values,
numerical execution, geometry and policy decisions remain later obligations.

## Immutable sourced declarations

FormulaDeclaration borrows a validated MachineToken's exact name or the actual normalized recipe
source. Its private construction and Copy/Clone views retain those lifetimes. Default Debug for
both the declaration and its source shows only kind/origin, omitting customer names, identities,
canonical states and numeric values. Explicit source inspection remains available.

FormulaInputOrigin closes the five external domains: Measurement, Ease, Parameter, Profile and
Material. The input constructor carries both the metadata record identity and its separate
canonical declaration identity, plus the declared scalar annotation. FormulaScalarInputOrigin
restricts this generic path to Parameter, Profile and Material. Measurement and Ease must use the
canonical length adapter; they cannot claim angle/area/ratio/count/Boolean input kinds. Other scalar
metadata remains a claim:
canonical registry adapters must validate target identity, kind and provenance. It does not store
an authored or computed number, certify a source record, or fetch a value.

The length_input adapter borrows an actual immutable LengthDeclaration and forces the kind length.
Known, assumed, unknown, preference and derived records all declare that same kind. The canonical
record retains its sole value/state/source ownership; the declaration does not clone or cache it.

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::{
    FormulaBindingKind, FormulaDeclaration, FormulaScalarInputOrigin, FormulaKind,
}};
let name = MachineToken::new("desired_sweep").unwrap();
let declared = FormulaDeclaration::input(&name, FormulaScalarInputOrigin::Parameter,
    EntityId::from_bits(11), EntityId::from_bits(12), FormulaBindingKind::Angle);
assert_eq!(declared.kind(), FormulaKind::Angle);
assert_eq!(declared.name(), "desired_sweep");
```

Point and edge constructors retain the exact PointRef/EdgeRef creator and local tag, forcing their
geometry kind and origin. They neither resolve coordinates/curves nor certify prior-operation order.
The reserved constructor takes a FormulaReservedName and derives its fixed name/kind/origin without
requiring its optional context. Source locators are exposed through FormulaDeclarationSource.

The recipe constructor inspects an actual one-based position in a FormulaNormalizedRecipe.
Only a let contributes a declaration. Zero, absent positions and assertion labels return None;
an assertion label cannot become a scalar input. Returned metadata preserves the actual ordinal,
name, authored kind annotation and original whole-statement/name spans. The RHS is not inferred,
executed or bound, and namespace rules still have to refuse reserved-name let bindings.

```rust
use sc_core::recipe::{FormulaDeclaration, FormulaKind, FormulaRecipe};
let syntax = FormulaRecipe::parse("let width:length=missing assert check:eps_num=width==width").unwrap();
let recipe = syntax.normalize_literals().unwrap();
let declared = FormulaDeclaration::recipe(&recipe, 1).unwrap();
assert_eq!(declared.kind(), FormulaKind::Length);
assert!(FormulaDeclaration::recipe(&recipe, 2).is_none());
```

Here missing remains unvalidated syntax. A sourced declaration is metadata for the future checker,
so this example grants no static acceptance or authority to compute a missing value.

## Verification boundary

The public formula_semantic_contract tests independently enumerate all eight kinds, six binding
conversions, nine origins and eight reserved names with their kind/origin/context/tolerance roles.
They compare every row of the actual normative tables in both directions and reject repaired
spellings, unknown names and extra excluded-form reservations. No value context is accepted by
these metadata methods.

```bash
cargo test -p sc-core --test formula_semantic_contract
python3 -I -B docs/tasks/artifacts/formula_structure/semantic_mutations.py
```

Run the mutation command alone: it compiles ten temporary changes to the actual implementation
and requires assertion failures in test bodies, then restores the exact source bytes in finally.
The standing structural runner checks every fault anchor and refuses compiler/expect/name noise.
These controls verify metadata behavior; they do not establish full static or runtime acceptance.

The seven public formula_declaration_contract tests cover five canonical length origins/three general scalar domains/six
annotations, every canonical LengthState, exact geometry refs, all eight reserved names and actual
recipe ordinals/spans. Boundary4096 and absent4097 are checked; assertion labels and zero ordinals
declare nothing. Five compile-fail contracts enforce private fields and name/record/recipe lifetimes.

```bash
cargo test -p sc-core --test formula_declaration_contract
python3 -I -B docs/tasks/artifacts/formula_structure/declaration_mutations.py
```

Run declaration mutations alone. Nineteen actual compiled source/kind/origin/identity/ordinal/span/
privacy faults must fail test-body assertions; widening the scalar-domain boundary must also fail
the negative construction contract because the forbidden measurement call now compiles. Source
bytes are restored exactly. The structural runner
watches anchors and failure classification. Metadata tests and code inspection establish the stated
locator contract; numeric reads, adapters, namespace acceptance and physical geometry remain separate.

## Checked initial namespace

FormulaInitialDeclaration admits existing authored input and geometry declarations through
`TryFrom<FormulaDeclaration>`. Input, LengthInput, Point and Edge sources enter this boundary;
Recipe and Reserved return the original rejected declaration. A future let cannot become an
initial input, and an authored list cannot inject the language's own reserved metadata. Names
already borrow validated MachineToken values, so malformed spellings and the three grammar
keywords cannot enter this path. The declaration's canonical registry/operation-order validity
still requires its separately owned checks.

FormulaNamespace::new consumes these initial declarations in their authored order and includes
all eight reserved metadata entries. A collision refuses before replacing the earlier entry;
no partial namespace escapes. Two declarations from the same origin also collide, even with
identical metadata or values. Absent export, profile or size contexts do not hide the fixed kind
of their reserved names. Construction fetches no numeric value, state, provider or geometry.

This unknown measurement retains its actual canonical record while entering the namespace:

```rust
use sc_core::{
    name::MachineToken, ontology::EntityId,
    recipe::{FormulaDeclaration, FormulaDeclarationSource, FormulaInitialDeclaration,
        FormulaInputOrigin, FormulaKind, FormulaNamespace},
    value::{LengthDeclaration, LengthDeclarationDefinition, LengthState},
};
let name = MachineToken::new("body_width").unwrap();
let record = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(101), source: EntityId::from_bits(102),
    state: LengthState::Unknown { observation: EntityId::from_bits(103) },
}).unwrap();
let declaration = FormulaDeclaration::length_input(
    &name, FormulaInputOrigin::Measurement, EntityId::from_bits(104), &record);
let initial = FormulaInitialDeclaration::try_from(declaration).unwrap();
let namespace = FormulaNamespace::new([initial]).unwrap();
let found = namespace.declarations().find(|d| d.name() == "body_width").unwrap();
assert_eq!(found.kind(), FormulaKind::Length);
assert!(matches!(found.source(), FormulaDeclarationSource::LengthInput { declaration, .. }
    if std::ptr::eq(declaration, &record)));
assert_eq!(namespace.declarations().len(), 9); // one input plus eight reserved names
```

The declarations iterator is exact-size and ordered lexically by name for deterministic metadata
inspection. That order does not change recipe statement order. Namespace Clone retains the same
borrowed records and references; it copies no canonical value/state. Private fields and borrowed
lifetimes prevent detached names/records or direct namespace mutation. Default namespace Debug
shows only the declaration count; declaration/initial/error Debug omits authored names and IDs.

## Source-preserving collision errors

FormulaNamespaceError has two cases. ReservedBinding carries the fixed FormulaReservedName and
actual attempted declaration; its token is formula_rebinding. AmbiguousName carries the first and
second authored declarations; its token is formula_ambiguous_name. name() explicitly exposes the
colliding spelling, and binding_sources() returns the earlier/reserved and attempted declarations
in that order. The existing source views retain both semantic origins, canonical identities,
record borrows or exact point/edge references. Initial sources and reserved metadata have no
recipe statement ordinal; this API fabricates none. [Contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources)
and [reference argument evidence](formula-static-validation.md#reserved-name-diagnostic-sources)
define the source distinction.

The ambiguous source pair is boxed once on failure to keep the returned error compact. Error
Clone retains the same canonical borrows; it copies no authored numeric value or state.

A profile parameter attempting a reserved format-tolerance name refuses without a format context:

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::{
    FormulaBindingKind, FormulaDeclaration, FormulaInitialDeclaration, FormulaNamespace,
    FormulaNamespaceError, FormulaReservedContext, FormulaReservedName, FormulaScalarInputOrigin,
    FormulaToleranceName,
}};
let name = MachineToken::new("eps_fmt").unwrap();
let attempt = FormulaDeclaration::input(&name, FormulaScalarInputOrigin::Profile,
    EntityId::from_bits(201), EntityId::from_bits(202), FormulaBindingKind::Count);
let error = FormulaNamespace::new([
    FormulaInitialDeclaration::try_from(attempt).unwrap()
]).unwrap_err();
assert_eq!(error.token(), "formula_rebinding");
assert_eq!(error.name(), "eps_fmt");
assert!(matches!(error, FormulaNamespaceError::ReservedBinding { reserved, .. }
    if reserved == FormulaReservedName::Tolerance(FormulaToleranceName::Format)
        && reserved.required_context() == FormulaReservedContext::Export));
assert_eq!(error.binding_sources()[1].kind(), sc_core::recipe::FormulaKind::Count);
```

Default error Display exposes only the internal stable token. Localized user messages belong to
the command layer and use the explicit typed arguments. A successful initial namespace certifies
source admission and initial-name uniqueness. Exact initial reads are available below; prior recipe bindings remain
.5b.2d.2; expression kind/signature checking and complete static dependency graphs remain .3/.4.
Numeric state/context validation, evaluation and geometric/registry correctness retain their owners.

Ten public contracts and three negative private/lifetime doctests cover closed reserved metadata, rejected recipe/reserved admission, three
scalar domains/six kinds, five canonical length domains/all five states, exact geometry refs,
all36 ordered origin collisions/equal origins, reserved attempts across every scalar annotation
and point/edge source, first-error ordering, deterministic inspection, lifetimes and opaque
formatting. Seventeen actual compiled admission/context/collision/source/order/privacy faults
fail public body assertions and restore the production source exactly; the watched structural
runner also refuses invalid fault classification and stale anchors.


## Exact declared-name reads

FormulaNamespace::resolve accepts a validated MachineToken and returns the exact existing
FormulaDeclaration. Lookup uses the authored spelling directly. The returned declaration retains
its original name and canonical source borrows; it is independent of the query token and namespace
allocation. Reading metadata neither copies canonical state nor fetches a numerical value.

An unknown measurement still has a known declared kind. The unknown body_width namespace in the
[initial namespace example](#checked-initial-namespace) resolves to the same borrowed record:

```rust
# use sc_core::{name::MachineToken, ontology::EntityId, recipe::{FormulaDeclaration,
# FormulaInitialDeclaration, FormulaInputOrigin, FormulaKind, FormulaNamespace},
# value::{LengthDeclaration, LengthDeclarationDefinition, LengthState}};
# let name = MachineToken::new("body_width").unwrap();
# let record = LengthDeclaration::new(LengthDeclarationDefinition { id: EntityId::from_bits(1),
# source: EntityId::from_bits(2), state: LengthState::Unknown { observation: EntityId::from_bits(3) } }).unwrap();
# let namespace = FormulaNamespace::new([FormulaInitialDeclaration::try_from(
# FormulaDeclaration::length_input(&name, FormulaInputOrigin::Measurement, EntityId::from_bits(4), &record)).unwrap()]).unwrap();
let query = MachineToken::new("body_width").unwrap();
let declared = namespace.resolve(&query).unwrap();
assert_eq!(declared.kind(), FormulaKind::Length);
assert_eq!(declared.name(), "body_width");
```

Reserved names likewise retain their kinds when their runtime providers are absent: eps_fmt is a
length with export context, eps_phys is a length with profile context, and is_base_size is Boolean
with size context. Static resolution supplies no provider-availability or canonical registry proof.
A runtime read of an unknown fact remains formula_unknown; a missing valid tolerance provider
remains formula_tolerance_unbound. Those runtime adapters and execution checks remain separately
owned. This API resolves the initial namespace before statement one; prior recipe visibility,
forward/self-reference refusal and expression type checking remain .5b.2d.2–.4.

An absent declaration returns opaque FormulaUnboundName. name() borrows the exact query and
origins_searched() exposes all nine flat namespace domains in contract order: measurement, ease,
parameter, profile, material, geometry, recipe, size and tolerance. Recipe has no entries in the
initial namespace. No spelling is repaired, alias accepted, value substituted or recipe reordered.
The error carries no guessed statement index or canonical expression; later whole-recipe composition
adds only context it actually owns. Default Debug omits the name; Display is the stable internal
token, which the command layer must localize with the explicit arguments.

```rust
use sc_core::{name::MachineToken, recipe::{FormulaNamespace, FormulaOrigin}};
let namespace = FormulaNamespace::new([]).unwrap();
let query = MachineToken::new("missing_width").unwrap();
let error = namespace.resolve(&query).unwrap_err();
assert_eq!(error.token(), "formula_unbound_name");
assert_eq!(error.name(), "missing_width");
assert_eq!(error.origins_searched(), &FormulaOrigin::ALL);
```

Seven public read contracts verify all scalar annotations/domains, five canonical length states/
domains, exact point/edge references, all eight reserved sources, distinct names with equal kinds,
missing/near spellings, immutable failures, independent borrow lifetimes and formatting privacy.
Two compile-fail contracts protect the diagnostic's private construction and query lifetime.
Nine actual compiled lookup/source/value-read/fallback/domain/token/name/privacy faults must fail
public body assertions and restore exact source bytes. Run the mutation command alone:

```bash
cargo test -p sc-core --test formula_name_read_contract
python3 -I -B docs/tasks/artifacts/formula_structure/name_read_mutations.py
```
