# Formula declaration metadata

> **Status:** implemented vocabulary and immutable sourced declarations at G1-SLICE.5b.2a/.2b in sc-core::recipe.
> Namespace resolution is next at .2c/.2d; a recipe still has no product
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

A reserved spelling is a valid reference token and may not be rebound. Namespace collision,
forward-reference and whole-recipe checks remain .5b.2c/.2d and .5b.3/.4. Context values,
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
