# Complete formula recipe proofs

> **Implemented:** sc-core recipe, G1-SLICE.5b.4a. A normalized recipe can return an immutable
> proof that every statement passes names and kinds in its actual prior-binding scope.
> Coupled whole-factory review remains .5b.4b. Numerical execution, construction operations,
> persistence and physical approval retain their later owners.

Use this interface after complete [syntax and literal input checks](formula-recipe-inputs.md).
FormulaNormalizedRecipe::check_kinds consumes a checked FormulaNamespace and borrows the actual
normalized recipe. It returns FormulaCheckedRecipe only after every statement succeeds. Initial
namespace construction can refuse before this call; those admission errors have no recipe ordinal.

## Accept a complete ordered recipe

```rust
use sc_core::recipe::{FormulaNamespace, FormulaRecipe};
let recipe = FormulaRecipe::parse("let n:count=1\nassert check:eps_num=n==n")
    .unwrap().normalize_literals().unwrap();
let proof = recipe.check_kinds(FormulaNamespace::new([]).unwrap()).unwrap();
assert!(std::ptr::eq(proof.recipe(), &recipe));
assert_eq!(proof.statements().len(), 2);
assert_eq!(proof.dependencies().count(), 3);
assert_eq!(proof.canonical_recipe(), recipe.canonical_form());
```

The validator privately stages a cursor and statement proofs. It checks a header, then the
statement's scoped operands and annotation/comparison, then advances the successful annotation.
A failed let cannot supply a later statement. Self and future names remain unavailable; assertion
labels supply no scalar. A header collision refuses before its RHS is inspected. Existing
[call and child priorities](formula-name-scopes.md#check-the-actual-current-statement) remain intact,
including checks in both conditional branches.

statements() exposes the complete immutable ordered slice of FormulaCheckedStatement proofs.
Each retains its original normalized statement and genuine one-based position. The vector cannot
be replaced or mutated through this API. An empty recipe produces an empty complete proof.
Clone the initial namespace before the call if another check needs the same immutable metadata;
the private staging cursor never modifies that caller's reusable clone.

## Read ordered dependency occurrences

FormulaRecipeNameDependency has statement_index(), role(), span() and declaration() getters.
The consumer ordinal includes assertions. FormulaRecipeDependencyRole is a closed enum:

| Role | Original source |
| --- | --- |
| Binding | A name occurrence in a let's RHS |
| AssertionTolerance | The assertion's original symbolic tolerance header |
| AssertionLeft | A name occurrence in its left operand |
| AssertionRight | A name occurrence in its right operand |

The iterator visits statements in authored order. Within an assertion it yields the tolerance
header first, then left operand occurrences, then right operand occurrences. The header is genuine
metadata with its annotation span and fixed reserved declaration; it is not a fabricated expression
node. Repetitions and untaken-branch occurrences remain distinct. The iterator streams the existing
operand proofs, without allocating a duplicate whole edge vector.

The supplier's declaration retains its real initial input, reserved, geometry or prior-let source.
A prior-let source ordinal is strictly earlier than its consumer; this source language cannot create
a forward dependency cycle. An initial or reserved source has no recipe ordinal. Complete recipe
acceptance gives every prior annotation a successful check; a [local statement proof](formula-name-scopes.md)
alone cannot make that guarantee.

Unknown canonical inputs remain valid metadata, with no state read or numerical fallback:

```rust
use sc_core::{name::MachineToken, ontology::EntityId, recipe::*, value::*};
let width = MachineToken::new("width").unwrap();
let record = LengthDeclaration::new(LengthDeclarationDefinition {
    id: EntityId::from_bits(1), source: EntityId::from_bits(2),
    state: LengthState::Unknown { observation: EntityId::from_bits(3) },
}).unwrap();
let input = FormulaDeclaration::length_input(&width, FormulaInputOrigin::Measurement,
    EntityId::from_bits(4), &record);
let initial = FormulaNamespace::new([
    FormulaInitialDeclaration::try_from(input).unwrap(),
]).unwrap();
let recipe = FormulaRecipe::parse(
    "let doubled:length=if(is_base_size,width+width,width)\nassert check:eps_geo=doubled==width"
).unwrap().normalize_literals().unwrap();
let proof = recipe.check_kinds(initial).unwrap();
let uses: Vec<_> = proof.dependencies().collect();
assert_eq!(uses.iter().map(|d| d.declaration().name()).collect::<Vec<_>>(),
    ["is_base_size", "width", "width", "width", "eps_geo", "doubled", "width"]);
assert!(matches!(uses.get(1).unwrap().declaration().source(),
    FormulaDeclarationSource::LengthInput { declaration, .. }
    if std::ptr::eq(declaration, &record)));
assert!(matches!(uses.get(5).unwrap().declaration().source(),
    FormulaDeclarationSource::Recipe { statement_index: 1, .. }));
```

canonical_recipe() returns the authored normalized recipe's exact owned identity bytes.
Those bytes do not include the initial namespace's supplier identities. Equal recipe bytes with
different measurement records can therefore resolve different dependency sources. Inspect those
sources separately; canonical recipe bytes alone do not identify a resolved computation or release.
No graph fingerprint, storage identity or numeric dependency result is granted by this interface.

## Keep the actual first refusal

FormulaRecipeCheckError is privately constructed and retains recipe(), statement(),
statement_index(), span() and token(). FormulaRecipeCheckRefusal distinguishes Namespace (the
actual header collision) from Statement (the actual nested expression/annotation/comparison error).
Its read-only payload preserves existing typed arguments. The error exposes no successful prefix
or final namespace. A late refusal still retains the complete normalized recipe, including its suffix.

```rust
use sc_core::recipe::*;
let recipe = FormulaRecipe::parse(
    "let first:length=1 mm\nassert prior:eps_geo=first==first\nlet bad:length=1.0\nlet later:count=1"
).unwrap().normalize_literals().unwrap();
let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
assert!(result.is_err());
let error = result.unwrap_err();
assert_eq!(error.statement_index(), 3);
assert_eq!(error.token(), "formula_dimension");
assert!(std::ptr::eq(error.recipe(), &recipe));
assert_eq!(error.span(), error.statement().annotation_span());
assert_eq!(error.canonical_recipe(), recipe.canonical_form());
assert!(matches!(error.refusal(), FormulaRecipeCheckRefusal::Statement(nested)
    if matches!(nested.refusal(), FormulaStatementCheckRefusal::BindingDimension(args)
        if args.expression_kind() == FormulaKind::Ratio && args.wanted_kind() == FormulaKind::Length)));
```

Header collisions use the original binding name span. Other failures retain the existing actual
statement error span: annotation for a let mismatch, nested node for an expression refusal, and
whole statement for the assertion comparison. No comparison separator span or combined AST is
invented. canonical_statement() and canonical_recipe() are explicit identity requests from the
actual owner, even after rejection; these requests do not certify acceptance.

Reserved-name refusals keep formula_rebinding and real binding sources, including metadata with
no ordinal. An initial-name collision uses formula_ambiguous_name. Earlier-let rebinding retains
both genuine recipe positions. D131's diagnostic repair changes no grammar:

```rust
use sc_core::recipe::*;
let recipe = FormulaRecipe::parse(
    "let first:count=1\nassert prior:eps_num=1==1\nlet eps_num:length=missing"
).unwrap().normalize_literals().unwrap();
let result = recipe.check_kinds(FormulaNamespace::new([]).unwrap());
assert!(result.is_err());
let error = result.unwrap_err();
assert_eq!(error.token(), "formula_rebinding");
assert_eq!(error.statement_index(), 3);
assert!(matches!(error.refusal(), FormulaRecipeCheckRefusal::Namespace(collision)
    if matches!(collision.binding_sources()[0].source(), FormulaDeclarationSource::Reserved(_))
    && matches!(collision.binding_sources()[1].source(),
        FormulaDeclarationSource::Recipe { statement_index: 3, .. })));
```

Proofs and errors borrow their normalized owner and any canonical source records. Keep those
owners alive while inspecting them; they cannot escape a local recipe or local input record.
Default Debug omits authored source; error Display contains only the stable diagnostic token.
Explicit source/name/canonical getters deliberately expose authored content to the caller.

## Verification and remaining authority

```bash
cargo test -p sc-core --test formula_checked_recipe_contract -- --nocapture
python3 -I -B docs/tasks/artifacts/formula_structure/recipe_owner_contract.py
python3 -I -B docs/tasks/artifacts/formula_structure/checked_recipe_mutations.py
```

The public contracts cover96 binding pairs across prefix positions and640 assertion kind/class/
position cases, late/untaken errors, real collisions, repeated dependencies and unknown records.
Limits4096 statements,256-node operands,16 conditionals and4000 grouping pairs pass on a64KiB stack.
Five actual documentation compiler guards require only E0451/E0515 for privacy and recipe/record
borrows. The four Rust examples in this chapter compile and execute against Cargo's current library.
Actual compiled whole-proof faults require running public assertion failures, with source and
compiled artifact restored; compile/link/expect-only noise never counts. Run mutations exclusively.

A statically valid false assertion, zero divisor, negative square root, absent size/export context
or too-wide stored result still has no execution verdict. The proof reads no values, providers or
geometry and cannot approve those cases. Coupled whole-factory review is .5b.4b; exact arithmetic
and binding are .5c, irrational algorithms .5d, execution/replay .5e and operations .5f. Persistence,
physical geometry, export readiness and human approval retain their separate roadmap owners.
