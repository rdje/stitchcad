# Formula declaration metadata

> **Status:** implemented product vocabulary at G1-SLICE.5b.2a in sc-core::recipe. Immutable sourced
> declarations and namespace resolution are the next .5b.2 children; a recipe still has no product
> static acceptance or execution. [Reference static review](formula-static-validation.md) is complete
> for its stated instrument populations and has a separate proof scope.

The checker needs to know what a name means before fetching its value. An unknown measurement
can still be length. A size context can be absent while size_index still has the kind count.
These types describe that metadata; they contain no numbers, input states or value providers.
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

A reserved spelling is a valid reference token and may not be rebound. This metadata slice does
not yet create or validate a namespace, inspect declaration pairs, reject collisions or resolve
forward references. It also supplies no context values, geometry, policy decisions or evaluator.
Those obligations remain .5b.2b–.2d, .5b.3/.4 and .5c–.5g.

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
