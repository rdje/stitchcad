# Built-in and selector signatures

> **Implemented:** closed signature metadata in sc-core recipe, G1-SLICE.5b.3b. This API reads
> kinds and symbolic roles. It does not accept a formula, evaluate a value or invoke geometry.
> Grammar, canonical spelling and the existing if special form remain unchanged.

After [operator signatures](formula-operator-signatures.md), the next static question is whether
the arguments fit a published function or selector. Unknown measurements still have declared
kinds, and a tolerance class remains recognizable even when its value provider is absent.
The normative rows are [grammar §§6–7](../spec/formula-language/grammar.md); the API implements
their closed kind rules independently of their later numeric and geometry rules.

## Inspect the closed vocabulary

FormulaBuiltin::ALL contains 22 names in normative order. The grouped sin/cos/tan and x/y rows
expand into separate enum variants. Sqrt and atan2 each have two signatures but one registry entry.
Exact token recognition neither trims nor normalizes a name, and adds no keyword reservations.

```rust
use sc_core::recipe::{FormulaBuiltin, FormulaBuiltinCategory, FormulaBuiltinArity};

let function = FormulaBuiltin::from_token("hypot").unwrap();
assert_eq!(function, FormulaBuiltin::Hypot);
assert_eq!(function.category(), FormulaBuiltinCategory::Function);
assert_eq!(function.arity(), FormulaBuiltinArity::Fixed(2));
assert_eq!(FormulaBuiltin::from_token(" Hypot"), None);
assert_eq!(FormulaBuiltin::from_token("unlisted_call"), None);
assert_eq!(FormulaBuiltin::Min.arity(), FormulaBuiltinArity::OneOrMore);
```

The categories describe the existing syntax and semantics:

| Category | Names | Meaning |
| --- | --- | --- |
| Function | sqrt, hypot, abs, min, max, clamp, round_to, sin, cos, tan, atan, atan2, arc_length | Numeric functions with separate runtime domains and rounding |
| Conditional | if | Existing special form; check both branches, evaluate only the chosen branch |
| ToleranceComparison | within | Same-kind arithmetic comparison with a named class |
| Selector | x, y, dist, dir, len, param_at, point_at | Read existing geometry; create no entity |

If remains special syntax rather than an ordinary extensible call. Loop, repeat, while, fn and
macro are ordinary scalar spellings with no new capability. The lookup returns None for them,
reserved input names and envelope-owned constructs such as nurbs or solve. The later expression
checker must apply the envelope's named refusal before generic unknown-call handling; this
metadata lookup alone emits no diagnostic or origin-search claim.

## Query a signature

FormulaBuiltinOperand::Value carries one of the eight operand kinds. Result_kind returns an
Option of FormulaKind: Some supplies the declared result kind; None means no signature accepts
the supplied arity, kinds or symbolic role. It supplies no source location or expression proof.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaBuiltinOperand as O, FormulaKind as K};

assert_eq!(F::Hypot.result_kind(&[O::Value(K::Length), O::Value(K::Length)]),
           Some(K::Length));
assert_eq!(F::Hypot.result_kind(&[O::Value(K::Length), O::Value(K::Angle)]), None);
assert_eq!(F::ArcLength.result_kind(&[O::Value(K::Angle), O::Value(K::Length)]),
           Some(K::Length));
assert_eq!(F::ArcLength.result_kind(&[O::Value(K::Length), O::Value(K::Angle)]), None);
assert_eq!(F::ParamAt.result_kind(&[O::Value(K::Edge), O::Value(K::Length)]),
           Some(K::Ratio));
assert_eq!(F::PointAt.result_kind(&[O::Value(K::Edge), O::Value(K::Ratio)]),
           Some(K::Point));
```

The arithmetic type variable T binds consistently to length, angle, area, ratio or count.
Boolean, point and edge cannot fill T. This includes abs(count), whose kind is still count;
negative counts remain a separate numeric-domain refusal. Min and max start at one operand,
and every operand must share its arithmetic kind. Clamp and round_to retain that same rule at
their fixed arities. Atan2 accepts two lengths or two ratios, never one of each; their positional
y, x meaning is preserved by the later expression and value layers.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaBuiltinOperand as O, FormulaKind as K};

assert_eq!(F::Min.result_kind(&[O::Value(K::Length)]), Some(K::Length));
assert_eq!(F::Max.result_kind(&[]), None);
assert_eq!(F::Min.result_kind(&[O::Value(K::Length), O::Value(K::Count)]), None);
assert_eq!(F::If.result_kind(&[O::Value(K::Boolean), O::Value(K::Length),
                              O::Value(K::Length)]), Some(K::Length));
assert_eq!(F::If.result_kind(&[O::Value(K::Boolean), O::Value(K::Length),
                              O::Value(K::Angle)]), None);
```

Checking the supplied branch kinds does not prove that either expression resolves its names.
The accepted-expression layer must check every branch and argument before whole-recipe acceptance.

## Keep the tolerance role symbolic

FormulaBuiltinOperand::Tolerance retains one of the five existing FormulaToleranceName classes.
Its ordinary kind is length, so it can also be read as a length outside within's third role.
Within requires that third operand to carry the actual symbolic role, not merely the length kind.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaBuiltinOperand as O,
                     FormulaKind as K, FormulaToleranceName as T};

let width = O::Value(K::Length);
let eps_geo = O::Tolerance(T::Geometric);
assert_eq!(eps_geo.kind(), K::Length);
assert_eq!(F::Within.result_kind(&[width, width, eps_geo]), Some(K::Boolean));
assert_eq!(F::Within.result_kind(&[width, width, O::Value(K::Length)]), None);
assert_eq!(F::Within.result_kind(&[width, width, O::Value(K::Count)]), None);
assert_eq!(F::Abs.result_kind(&[eps_geo]), Some(K::Length));
```

An ordinary length literal, a length-valued variable or an expression such as eps_geo + eps_geo
does not name a class. Size_index, size_count and is_base_size are size inputs, not classes.
The descriptor can be constructed for metadata queries; constructing one grants no proof about
an authored expression. The future checker must derive Tolerance from the actual resolved
reserved-name node, including transparent grouping, and keep derived lengths as Value.
No provider, tolerance value, rounding operation or contribution source is read here.

## Arity and structural bounds are different checks

FormulaBuiltinArity::Fixed requires exactly its count; OneOrMore requires at least one.
Only min/max return OneOrMore. Signature arity does not relax the parser's 255-argument bound.
A metadata slice with 256 homogeneous lengths has a min signature, but a source call with 256
arguments still refuses at the structural boundary. Zero-argument calls refuse syntax; a direct
metadata query on an empty slice returns None.

## Verification and remaining proof

Four public contracts independently enumerate all eight kinds and all five symbolic classes:
every 13-member tuple at arities zero through four, for every registry name, gives 680702 cases.
Wider min/max samples cover 255, 256 and 4096 operands. The complete actual normative rows are
compared in both directions, and all 22 tokens match the unchanged canonical serializer.
Twenty-one actual compiled faults must fail test-body assertions and restore product source exactly;
compiler errors, expect-only panics and test-name noise cannot count as proof.

Run cargo test -p sc-core --test formula_builtin_signature_contract for the public contracts.
The tracked producer docs/tasks/artifacts/formula_structure/builtin_signature_mutations.py runs
exclusive product faults; its classifier and exact anchors are watched by structural probes.

Accepted normalized expressions, source-bearing dimension refusals and every name dependency
remain G1-SLICE.5b.3c; coupled review is .3d, and atomic ordered recipe acceptance is .4.
Numeric domains, rounding and execution are .5c–.5e; real geometry selectors are .5f/G2.
No computed garment, API/MCP release or independent production approval follows from these signatures.
