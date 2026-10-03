# Ordered formula name scopes

> **Status:** implemented metadata foundation, G1-SLICE.5b.2d.2. FormulaNameCursor and
> FormulaStatementNameScope enforce authored declaration order and retain actual binding sources.
> Expression signatures/types and complete static recipe acceptance remain .5b.3/.4; execution
> remains .5c–.5g. This chapter builds on [initial declarations and exact reads](formula-declarations.md).

A formula statement sees the initial namespace and declarations above it. A current or future let
cannot supply its own RHS, and an assertion label never declares a scalar value. The cursor derives
that scope from an actual [normalized recipe](formula-recipe-inputs.md), preserving its owner,
statement order and original global spans. Callers cannot supply an ordinal, inject a recipe
binding or obtain a final initial namespace from the cursor.

## Visit one actual statement at a time

FormulaNameCursor::new consumes a checked FormulaNamespace and borrows one immutable
FormulaNormalizedRecipe. The initial namespace contains authored sources and all eight fixed
reserved names. current() returns the current FormulaStatementNameScope, or None at the end.
The scope exposes statement_index(), the actual one-based ordinal, and statement(), the original
normalized statement with its name, spans, annotation and expression operands.

resolve() accepts a validated MachineToken and returns an original initial or earlier-let
FormulaDeclaration. Its exact-name behavior and missing-name arguments are the same as the
[initial read API](formula-declarations.md#exact-declared-name-reads). Unknown canonical records and
absent export/profile/size providers retain their declared kinds. The cursor reads no numerical
value, canonical state, provider availability or physical geometry.

```rust
use sc_core::{name::MachineToken, recipe::{
    FormulaKind, FormulaNameCursor, FormulaNamespace, FormulaRecipe,
}};
let source = "let width:length=25 cm let hem:length=width";
let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
let width = MachineToken::new("width").unwrap();
let first = cursor.current().unwrap().unwrap();
assert_eq!(first.statement_index(), 1);
assert!(first.resolve(&width).is_err()); // current let is absent from its RHS scope
assert!(cursor.advance_metadata().unwrap());
let second = cursor.current().unwrap().unwrap();
assert_eq!(second.statement_index(), 2);
assert_eq!(second.statement().name(), "hem");
assert_eq!(second.resolve(&width).unwrap().kind(), FormulaKind::Length);
assert!(cursor.advance_metadata().unwrap());
assert!(cursor.current().unwrap().is_none());
assert!(!cursor.advance_metadata().unwrap());
```

advance_metadata() records only the current let's actual authored annotation and location, then
moves one statement forward. It returns true for an advance and false at the fused end. An assert
moves the cursor but adds no binding, even if its label repeats a value name or reserved spelling.
Assertion positions count toward subsequent let ordinals. Empty recipes and the fixed maximum
of 4,096 statements have the same end behavior.

This is metadata staging. A normalized RHS such as missing or a dimensionally invalid operator is
still unchecked here. Recording its annotation supplies no proof that the expression has that kind
or can execute. The product type checker and whole-recipe validator must check every operand before
using metadata advance; they remain the next owners. No accepted expression, recipe graph, numeric
binding or partial geometry is returned by this API.

## Binding refusals retain real sources

current() checks a let header before exposing its scope, and advance_metadata() applies that same
check before insertion. A refusal leaves the cursor at the same actual statement with the same
prior declarations. Repeating either call repeats the refusal; a caller cannot skip it, silently
replace an earlier entry or reorder the recipe to make a forward name visible.

FormulaNamespaceError now represents these three binding cases:

| Case | Token | Binding sources |
| --- | --- | --- |
| ReservedBinding | formula_rebinding | fixed reserved metadata, actual attempted declaration |
| AmbiguousName | formula_ambiguous_name | earlier authored input, actual attempted let |
| RecipeRebinding | formula_rebinding | earlier and attempted lets in this same actual recipe |

The initial namespace also uses AmbiguousName for colliding authored pairs, including equal origins.
An assertion label introduces no binding collision. name() and binding_sources() explicitly expose
the refusal's spelling and ordered source pair; default Debug omits authored payload and Display
contains only the stable internal token. The command layer localizes it using these typed arguments.

Recipe sources include both actual one-based statement indices, authored kinds, whole-statement
spans and binding-name spans. Reserved metadata and initial inputs have no recipe ordinal; none is
invented. Canonical record identities/borrows and geometry references remain intact. This follows
[contract5.2.1](../spec/formula-language.md#521-binding-refusal-sources) and the delegated
[D131 reference evidence](formula-static-validation.md#reserved-name-diagnostic-sources).

```rust
use sc_core::recipe::{FormulaDeclarationSource, FormulaNameCursor,
    FormulaNamespace, FormulaNamespaceError, FormulaRecipe};
let source = "let width:length=25 cm assert gap:eps_fmt=width==width let width:angle=90 deg";
let recipe = FormulaRecipe::parse(source).unwrap().normalize_literals().unwrap();
let mut cursor = FormulaNameCursor::new(FormulaNamespace::new([]).unwrap(), &recipe);
assert!(cursor.advance_metadata().unwrap());
assert!(cursor.advance_metadata().unwrap());
let error = cursor.current().unwrap_err();
assert!(matches!(error, FormulaNamespaceError::RecipeRebinding { .. }));
assert_eq!(error.token(), "formula_rebinding");
let [prior, attempt] = error.binding_sources();
assert!(matches!(prior.source(), FormulaDeclarationSource::Recipe { statement_index: 1, .. }));
assert!(matches!(attempt.source(), FormulaDeclarationSource::Recipe { statement_index: 3, .. }));
assert!(cursor.advance_metadata().is_err());
assert_eq!(cursor.current().unwrap_err().name(), "width");
```

The absent export tolerance in that assertion has no effect on metadata staging; its value is a
later runtime obligation. Likewise a reserved-name let refuses against the fixed metadata source
without fetching its provider. Header refusals precede later RHS name/type inspection.

## Ownership and verification

Cursor fields are private. A scope borrows its cursor and actual statement owner, so it cannot
survive an advance or escape either allocation. Successful declaration metadata retains its
original canonical/recipe source lifetime; missing-name diagnostics retain the exact query lifetime.
Default cursor/scope Debug exposes only the position/ordinal. No source text or authored name is
included by default.

Eight public contracts cover actual owner pointers, initial/prior/self/forward visibility, six
annotations, all reserved contexts and48 reserved attempts, assertion gaps/labels, input/geometry
collision identities, both repeated-let indices/spans, unchanged failed advances, fused empty/max
ends and formatting. Both4,096-assertion and4,096-let recipes traverse and drop on a64KiB stack.
Five compile-fail examples protect private cursor fields, final namespace escape, recipe/scope
lifetimes and mutation while a borrowed scope remains live.

```bash
cargo test -p sc-core --test formula_ordered_names_contract
python3 -I -B docs/tasks/artifacts/formula_structure/ordered_name_mutations.py
```

Run mutations alone: eighteen temporary compiled faults in the actual cursor/diagnostic code must
fail public body assertions, then restore both original source files exactly. Controls cover
predeclared future names, skipped/incorrect positions, ignored/misclassified headers, lost or
reversed sources, changed tokens, failed-advance state, assertion advancement, wrong owner views
and formatting leaks. The standing structural suite watches fault anchors and refuses compiler,
unwrap/expect or test-name noise. Complete expression/static graph and runtime acceptance retain
their separate proof owners.
