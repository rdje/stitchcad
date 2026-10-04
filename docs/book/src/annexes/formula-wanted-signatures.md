# Typed wanted-kind signatures

> **Implemented:** immutable diagnostic metadata in sc-core recipe, G1-SLICE.5b.3c.1.
> The catalogs describe admissible operand kinds and symbolic roles. Source-bearing expression
> errors and initial-scope expression proofs are implemented below; whole-recipe acceptance remains .4.

A dimension diagnostic needs both the actual operand kinds and the kinds the operation accepts
([contract §5.2](../spec/formula-language.md)). Saying only that a call failed cannot explain which
argument needs correction. These catalogs provide the typed wanted rules for every implemented
[operator](formula-operator-signatures.md) and [built-in or selector](formula-builtin-signatures.md).
They carry no input values, source spans, statement indices or runtime context.

## Inspect an admissible rule

Each operation's signatures method returns a static, immutable slice of FormulaKindSignature
rows. A row has ordered operand requirements, an arity and a result requirement. It is constructed
privately from the normative tables; callers can inspect or copy it but cannot replace its fields
or forge a positional result index.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaBuiltinArity as A,
                     FormulaOperandRequirement as R, FormulaResultRequirement as V,
                     FormulaKind as K};

let rows = F::Hypot.signatures();
assert_eq!(rows.len(), 1);
let row = rows.first().unwrap();
assert_eq!(row.operand_requirements(), [R::Exact(K::Length), R::Exact(K::Length)]);
assert_eq!(row.arity(), A::Fixed(2));
assert_eq!(row.result_requirement(), V::Exact(K::Length));
```

| Requirement | Meaning | Examples |
| --- | --- | --- |
| Exact(kind) | That kind, with no implicit conversion | hypot takes two lengths; atan takes a ratio |
| Arithmetic | One shared T: length, angle, area, ratio or count | addition, clamp and matching if branches |
| Negatable | N: length, angle, area or ratio | unary minus excludes count |
| ToleranceName | One of the five exact reserved class symbols | within's third operand |

Arithmetic is a shared type variable throughout a row. It does not mean that each argument may
independently choose any arithmetic kind. Negatable is the single unary operand's allowed set.
A tolerance symbol retains its ordinary length kind in an Exact or Arithmetic position while
also retaining the separate class role for ToleranceName.

## Keep alternatives and operand order

Sqrt exposes area-to-length and ratio-to-ratio alternatives. Atan2 exposes length/length and
ratio/ratio; the two component kinds cannot be mixed. Square has three alternatives: length to
area, ratio to ratio, count to count. A binary product catalog explicitly includes both permitted
orders, while a quotient catalog preserves direction.

```rust
use sc_core::recipe::{FormulaBinaryOperator as B, FormulaBuiltinOperand as O, FormulaKind as K};

assert_eq!(B::Multiply.signatures().len(), 17);
assert_eq!(B::Divide.signatures().len(), 14);
let find = |args: &[O]| B::Divide.signatures().iter().find_map(|row| row.result_kind(args));
assert_eq!(find(&[O::Value(K::Area), O::Value(K::Length)]), Some(K::Length));
assert_eq!(find(&[O::Value(K::Length), O::Value(K::Area)]), None);
```

The result requirement is either Exact(kind) or Operand(index). An index is zero-based and refers
to an actual positional operand in that same row. Generic arithmetic results preserve operand
zero's kind. If preserves operand one's kind—the first branch—after requiring both branches to
share T and its condition to be Boolean. This is kind metadata, not a choice of which branch runs.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaOperandRequirement as R,
                     FormulaResultRequirement as V, FormulaKind as K};

let row = F::If.signatures().first().unwrap();
assert_eq!(row.operand_requirements(), [R::Exact(K::Boolean), R::Arithmetic, R::Arithmetic]);
assert_eq!(row.result_requirement(), V::Operand(1));
```

## Match kinds and class roles

Result_kind matches one row against FormulaBuiltinOperand descriptors. A matching row returns
the result kind; an arity, kind, consistency or role mismatch returns None. It reads no numeric
state or provider, performs no rounding and invokes no geometry. A catalog match cannot prove
that a source expression resolves its names or that its runtime domains hold.

```rust
use sc_core::recipe::{FormulaBuiltin as F, FormulaBuiltinOperand as O,
                     FormulaKind as K, FormulaToleranceName as T};

let row = F::Within.signatures().first().unwrap();
let width = O::Value(K::Length);
assert_eq!(row.result_kind(&[width, width, O::Tolerance(T::Geometric)]), Some(K::Boolean));
assert_eq!(row.result_kind(&[width, width, O::Value(K::Length)]), None);
assert_eq!(row.result_kind(&[width, O::Value(K::Count), O::Tolerance(T::Geometric)]), None);
```

Min and max each expose one OneOrMore row. Its sole Arithmetic requirement repeats for every
argument, from a minimum of one. As with [arity metadata](formula-builtin-signatures.md#arity-and-structural-bounds-are-different-checks), this does not relax the syntax's 255-argument bound.

## Verification and remaining proof

The catalogs are checked against independently authored kind matrices and the existing metadata
implementations: all 656 unary/binary kind cases and all 680702 built-in kind/class tuples at
arities zero through four. Matching alternatives must be unique. Complete typed built-in rows
are compared with all 24 normative rows, including symbolic tolerance roles, shared variables,
variadic requirements and exact result positions. Operator descriptors retain their closed
arities, 17 product alternatives and 14 directed quotients; wider variadic samples remain checked.

The two public integration suites are formula_operator_signature_contract and
formula_builtin_signature_contract. Nineteen actual compiled catalog faults must produce test-body
assertion failures and restore source exactly. The tracked wanted_signature_mutations.py producer
under docs/tasks/artifacts/formula_structure is exclusive; standing structural probes watch its
anchors and reject compiler, expect-only or test-name noise as substitute evidence.

The [bounded product checker](#bounded-product-expression-checking) pairs these wanted rows with
every actual operand kind and its normalized owner. Named envelope refusal precedes ordinary
unknown calls; errors report the domains actually searched and invent no recipe ordinal. Current
statement context remains .5b.3c.3, atomic whole-recipe acceptance .4. Numerical execution and
release approval remain separate work.

## Reference expression dimension payloads

D138's actual book reference now retains the operation, every resolved operand kind, each direct
reserved tolerance-name role and the complete wanted-signature alternatives
([contract §5.2.3](../spec/formula-language.md#523-expression-dimension-arguments-and-error-selection)).
Rows retain ordered operands, a variadic flag and a result. The alias T is shared across all of
its positions; N is the four negatable kinds. Product catalogs provide their corresponding typed
requirements and result positions; source-bearing product expression errors are verified below.

For 1 mm + 1.0, operation is +, operand_kinds is length, ratio, and the wanted rule is T,T to T.
For sqrt(1 mm), the actual kind is length and both area-to-length and ratio-to-ratio alternatives
are retained. For within(1 mm,1 mm,size_count), the three actual kinds are length,length,count,
the third symbolic tolerance role is absent and the wanted rule is T,T,tolerance to boolean.
A computed length such as eps_num+eps_num retains no class role, while ((eps_num)) retains eps_num.
These arguments carry no numeric values or guessed source/recipe indices.

Known operations check children left-to-right before their complete signature. Both if branches
are checked; a bad condition cannot justify inventing an unbound branch's kind. Therefore
if(1,missing,1 mm) reports formula_unbound_name for missing, while if(1,1 mm,1 mm) reports
formula_dimension with count,length,length and the wanted boolean,T,T to T row. An unknown or
envelope callee still refuses before its arguments; syntax/input limits retain their earlier phase.

The independent payload producer covers4023 actual cases:16 unary,640 binary,2264 ordinary-call,
30 arity,512 conditional,512 reserved-role,15 additional symbolic-role,20 wide-boundary and14
multiple-error priority controls. It inspects3814 complete dimension refusals with exact schema,
operand order/roles and complete alternative populations, with state/value/execution trapped.
Fifteen actual compiled faults prove retained fields, row direction/variadics/duplicates, symbolic
roles, child order and absence of value reads. The older4032-case kind matrix and14 fault controls
remain green after changed anchors were repaired; it alone certifies no payload schema.

[Header arguments](formula-static-validation.md#binding-header-diagnostic-arguments) now preserve
raw annotations or both actual kinds without invented context. D140's reference geometry argument repair
is [verified separately](formula-static-validation.md#geometry-provider-argument-checking): point x/y
and edge len require Length before value evaluation, with scoped actual/wanted payloads and retained
contributions. This expression payload proof and that local reference adapter do not certify product
operation identity, whole operation graphs or physical geometry; those remain .5f.3b/G2.
Statement/recipe acceptance, numerical execution and generated geometry keep
their separate owners. The grammar, token set and syntax identities are unchanged.

## Bounded product expression checking

**Implemented at G1-SLICE.5b.3c.2b.2:** FormulaNormalizedExpression::check_kinds consumes the
actual normalized owner and a checked initial FormulaNamespace. It certifies every expression name
and kind without reading a value, input state, tolerance provider or geometry. Syntax and literal
normalization retain their earlier phase. FormulaCheckedExpression borrows that exact expression
and actual declaration sources, retaining the root kind and every ordered name occurrence,
including untaken branches. Repeated names remain separate uses with their original node spans.
The namespace allocation can be dropped; borrowed canonical records and authored names stay alive.

```rust
use sc_core::recipe::{FormulaExpression, FormulaNamespace, FormulaKind};
let expression = FormulaExpression::parse("if(is_base_size,1 mm,2 mm)").unwrap()
    .normalize_literals().unwrap();
let namespace = FormulaNamespace::new([]).unwrap();
let proof = expression.check_kinds(&namespace).unwrap();
assert_eq!(proof.kind(), FormulaKind::Length);
assert_eq!(proof.dependencies()[0].declaration().name(), "is_base_size");
assert_eq!(proof.canonical_expression().as_str(),
    "(if is_base_size length:1000 length:2000)");
```

Known kinds do not require available values. Is_base_size is statically Boolean even without an
Instance. Unknown measurement records remain checkable through their canonical length locators;
geometry declarations retain the actual PointRef/EdgeRef without resolving coordinates or curves.
A statically valid expression such as 1 mm / 0, sqrt(-1.0) or tan(90 deg) can still fail at runtime.
No check computes a branch condition or a numeric-domain verdict.

FormulaExpressionCheckError borrows the actual failing normalized-node span and the same
expression owner. Grouping remains part of that span; the checker invents no narrower callee
span or recipe ordinal. Canonical_expression uses the existing whole-expression canonical factory
only on explicit inspection. UnboundName and Call variants preserve their existing distinct
search/source payloads. Dimension carries FormulaDimensionRefusal with a typed operation,
every resolved immediate kind and direct tolerance role, all wanted rows and arc_length_hint.

```rust
use sc_core::recipe::{FormulaExpression, FormulaNamespace,
                     FormulaExpressionCheckRefusal, FormulaKind};
let expression = FormulaExpression::parse("if(1,1 mm,2 deg)").unwrap()
    .normalize_literals().unwrap();
let error = expression.check_kinds(&FormulaNamespace::new([]).unwrap()).unwrap_err();
assert_eq!(error.token(), "formula_dimension");
if let FormulaExpressionCheckRefusal::Dimension(args) = error.refusal() {
    assert_eq!(args.operation().token(), "if");
    assert_eq!(args.operands().iter().map(|arg| arg.kind()).collect::<Vec<_>>(),
        [FormulaKind::Count, FormulaKind::Length, FormulaKind::Angle]);
    assert_eq!(args.wanted_signatures().len(), 1);
}
```

Callees resolve before arguments. Known children check left-to-right before their complete
immediate signature; if checks condition, then branch and else branch. Thus if(1,missing,1 mm)
reports the missing name before a bad-condition dimension tuple can be completed. A nested child's
own refusal wins over its parent's later children. Spline(missing) retains env_nurbs; an undeclared
callee such as loop(missing) retains formula_unbound_name for loop. Syntax/input errors retain their
earlier phase. Within checks all operands before enforcing its class role. Grouping keeps a direct
tolerance name symbolic; arithmetic and calls returning Length do not acquire that role.

```rust
use sc_core::recipe::{FormulaExpression, FormulaNamespace, FormulaKind};
let namespace = FormulaNamespace::new([]).unwrap();
for source in ["within(1 mm,2 mm,(eps_geo))", "within(1 mm,2 mm,eps_phys)"] {
    let expression = FormulaExpression::parse(source).unwrap().normalize_literals().unwrap();
    assert_eq!(expression.check_kinds(&namespace).unwrap().kind(), FormulaKind::Boolean);
}
let expression = FormulaExpression::parse("within(1 mm,2 mm,eps_geo + 0 mm)").unwrap()
    .normalize_literals().unwrap();
assert_eq!(expression.check_kinds(&namespace).unwrap_err().token(), "formula_dimension");
```

Errors abort construction; no accepted dependency prefix escapes. Explicit heap work stacks stay
within the existing256-node/16-conditional bounds, independent of grouping or unary source depth.
Private fields and lifetime contracts prevent forged proofs and detached expression/error/source
owners. Debug omits customer source/names/magnitudes; Display emits only the internal token, which
the command layer must localize. Explicit payload/identity access is available for diagnostics.
Inspect the borrowed error while its normalized owner is alive. Returning it as a generic
`Box<dyn Error>` with a static lifetime would detach that owner and is refused by Rust; a later
command adapter must explicitly copy the available structured arguments into its owned diagnostic.

Seven public contracts exercise656 operator tuples and52156 call kind/direct-class tuples at
arities1–3, plus fourth-argument, syntax, priority, dependency, runtime-domain and structural-bound
controls. The independently authored rows match the closed call population in both directions.
Sixteen actual compiled checker faults must fail body assertions and restore exact source:

```bash
cargo test -p sc-core --test formula_checked_expression_contract
python3 -I -B docs/tasks/artifacts/formula_structure/checked_expression_mutations.py
```

Run mutations alone, without overlapping builds/probes/gates. The standing structural suite watches
fault anchors and refuses compiler/expect/test-name noise. These proofs cover expression checking
against an initial namespace. Current-statement annotations, ordered prior-binding integration,
whole-recipe acceptance, canonical registry/operation-order validation, numerical execution and
physical geometry retain .5b.3c.3/.4, .5e/.5f and G2 owners. The grammar and token set are unchanged.
