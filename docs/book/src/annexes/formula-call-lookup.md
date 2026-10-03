# Source-bearing call lookup

> **Implemented:** exact call lookup and immutable refusals in sc-core recipe,
> G1-SLICE.5b.3c.2a. This supplies callee metadata before argument checking. Accepted expressions,
> statement integration, whole-recipe validation and execution remain separate work.

A data name and a callee have different lookup domains. A measurement named waist can be read
as data; it cannot be invoked as waist(...). The closed built-in catalog alone supplies ordinary
functions and selectors. The six envelope aliases refuse first, with their envelope token and
actual requested construct ([contract §5.2.2](../spec/formula-language.md#522-call-lookup-sources)).
The grammar, its three keywords and its function population are unchanged.

## Resolve an exact ordinary callee

FormulaBuiltin::resolve_call takes a validated MachineToken. It searches the envelope aliases
before the [built-in catalog](formula-builtin-signatures.md), with no namespace, argument,
value, state or geometry provider. Success returns existing signature metadata. It proves no
argument kind, numeric domain or accepted expression.

```rust
use sc_core::name::MachineToken;
use sc_core::recipe::FormulaBuiltin;

let name = MachineToken::new("hypot").unwrap();
let builtin = FormulaBuiltin::resolve_call(&name).unwrap();
assert_eq!(builtin, FormulaBuiltin::Hypot);
assert_eq!(builtin.signatures().len(), 1);
```

All 21 ordinary call names are admitted. If remains the existing keyword special form and
cannot enter a MachineToken. Its metadata remains in the catalog for static conditional checking.
A built-in spelling may also be a data declaration, but that declaration supplies no callable.
Reserved data names such as eps_num and size_count also supply no callable.

## Retain an unknown callee and its actual sources

FormulaCallRefusal has private fields and borrows the exact query. An unbound callee retains
FormulaCallRefusalKind::Unbound, the formula_unbound_name token, scope formula_call and the
ordered sources Envelope, BuiltinCatalog. These are FormulaCallLookupSource values, separate
from the nine FormulaOrigin values used by [data-name reads](formula-declarations.md).

```rust
use sc_core::name::MachineToken;
use sc_core::recipe::{FormulaBuiltin, FormulaCallLookupSource as S};

let name = MachineToken::new("waist").unwrap();
let error = FormulaBuiltin::resolve_call(&name).unwrap_err();
assert_eq!(error.name(), "waist");
assert_eq!(error.lookup_scope(), "formula_call");
assert_eq!(error.token(), "formula_unbound_name");
assert_eq!(error.origins_searched(), [S::Envelope, S::BuiltinCatalog]);
assert!(error.alternatives().is_empty());
```

An unknown callee has no claimed replacement. The query remains borrowed; a refusal cannot
outlive its MachineToken. Debug omits the authored name and Display emits only the internal
token. A command adapter obtains structured arguments through explicit accessors and localizes
the token for presentation. No statement index, span or canonical expression is invented by
this name-only API. An enclosing checker retains context only when it actually possesses it.

## Refuse an envelope request before inspecting arguments

| Callee aliases | Refusal kind / token | Typed alternatives |
| --- | --- | --- |
| nurbs, spline, bspline | Nurbs / env_nurbs | LineSegment, CircularArc, CubicBezier |
| solve, constraint, fixpoint | SketchConstraints / env_sketch_constraints | OrderedConstructionRecipe |

The name accessor retains the alias actually requested, rather than replacing spline with nurbs
or inventing a constraint such as parallel. Its scope is a formula call. No entity or geometric
constraint parameters have been accepted. The searched source is Envelope alone: lookup stops
there before the built-in catalog or any argument is read.

```rust
use sc_core::name::MachineToken;
use sc_core::recipe::{FormulaBuiltin, FormulaCallAlternative as A, FormulaCallLookupSource as S};

let name = MachineToken::new("spline").unwrap();
let error = FormulaBuiltin::resolve_call(&name).unwrap_err();
assert_eq!(error.name(), "spline");
assert_eq!(error.token(), "env_nurbs");
assert_eq!(error.origins_searched(), [S::Envelope]);
assert_eq!(error.alternatives(), [A::LineSegment, A::CircularArc, A::CubicBezier]);
```

The [envelope contract](../spec/feature-matrix.md#10-the-diagnostic-contract) and
[units curve set](../spec/units-and-tolerances.md#4-curve-representation) declare the alternatives.
Their machine tags are line_segment, circular_arc, cubic_bezier and ordered_construction_recipe.
These tags describe diagnostics; invoking any of them as a callee still refuses as unbound.

The reference diagnostic payload contains name, lookup_scope and origins_searched. Env_nurbs
also carries curve_kind and supported_curve_set; env_sketch_constraints carries constraint_kind
and recipe_alternative. Each requested kind is the actual callee spelling in formula_call scope.
The reference's static and runtime entry points share this callee check before argument access.
Thus spline(missing_argument) reports env_nurbs, while loop(missing_argument) reports
formula_unbound_name naming loop. An argument name/kind error cannot replace either callee diagnostic. Syntax and input-normalization
refusals retain their earlier phase; this lookup grants no syntax or normalized-input proof.

## Verification and remaining boundary

Five public contracts independently enumerate the ordinary calls, envelope aliases, unknown and
reserved names, source order, alternatives, tags, privacy and borrowed query identity. Two negative
Rustdoc examples verify private construction and refusal lifetimes. Actual compiled source faults
must fail body assertions and restore every source byte.

The independent reference producer runs 166 actual cases, with exact complete payloads and
argument, namespace, value and execution traps. Twelve actual compiled counterfactuals prove
name/source/scope/alternative retention and callee-before-argument precedence at static and runtime
entry points. These controls address D136/D137; the separate dimension payload repair remains
G1-SLICE.5b.3c.2b.1. None supplies accepted expression, whole-graph, value or geometry proof.

The durable engineering decision is `docs/decisions/decision_call-lookup.md`; its verification
claims this metadata scope, with independent production approval unclaimed.
