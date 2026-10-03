# Typed wanted-kind signatures

> **Implemented:** immutable diagnostic metadata in sc-core recipe, G1-SLICE.5b.3c.1.
> The catalogs describe admissible operand kinds and symbolic roles. Source-bearing expression
> errors and accepted expression owners are the next slice; whole-recipe acceptance remains .4.

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

The next checker must pair these wanted rows with every actual operand kind and real node/source
context. Named envelope refusal still takes precedence over ordinary unknown calls. Lookup errors
must report the catalog or declaration domains actually searched; a standalone expression has
no invented recipe ordinal. Those error/owner interfaces are .5b.3c.2, actual statement context
is .3, and atomic whole-recipe graph acceptance is .4. Numerical execution and release approval
remain separate work.
