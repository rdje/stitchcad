# Formula operator kind signatures

> **Status:** implemented metadata, G1-SLICE.5b.3a. Unary and binary kind signatures match
> [grammar5/5.1](../spec/formula-language/grammar.md#5-operators). Built-in/selector signatures,
> complete expression checking and [whole recipe kind proofs](formula-checked-recipes.md) are
> implemented separately; numerical execution remains .5c–.5g.

A declared kind can be checked before its value is known. These APIs accept only FormulaKind
metadata and return a result kind or None when the operator has no signature for those operands.
They consult no source value, state, provider or geometry. They produce no accepted expression,
recipe graph or numeric binding; the separate contextual expression checker owns expression proofs.

## Unary operators

FormulaUnaryOperator has two variants. Negate accepts length, angle, area and ratio, retaining that
kind. Count, Boolean, point and edge are refused. Square accepts length and returns area; ratio and
count squares retain their kinds. An angle square, Boolean square or geometry square is refused.
The exponent remains fixed syntax, never a second operand or a general exponent facility.

token() returns the existing canonical node symbols: - and ^2. These are machine identity metadata;
the display renderer retains the localized presentation rules. result_kind() checks only the kind:
negative count values and other numeric domains are separate runtime obligations.

```rust
use sc_core::recipe::{FormulaKind as K, FormulaUnaryOperator as U};
assert_eq!(U::Negate.result_kind(K::Angle), Some(K::Angle));
assert_eq!(U::Negate.result_kind(K::Count), None);
assert_eq!(U::Square.result_kind(K::Length), Some(K::Area));
assert_eq!(U::Square.result_kind(K::Ratio), Some(K::Ratio));
assert_eq!(U::Square.result_kind(K::Angle), None);
```

## Binary operators and operand order

The existing FormulaBinaryOperator now exposes token() and result_kind(left, right). Addition and
subtraction accept matching arithmetic kinds: length, angle, area, ratio or count. All six comparison
operators require that same relation and return Boolean. Boolean operands are outside the arithmetic
relation even for equality; point and edge references are also outside it.

Multiplication follows the closed product table in both orders. Length times length returns area;
a ratio or count scales the permitted arithmetic quantity. Count times ratio returns ratio, and
count times count returns count. An angle times length is refused in either order.

Division follows the authored table direction. Area divided by length returns length, while length
divided by area is refused. Count divided by ratio returns count; ratio divided by count returns
ratio; count divided by count returns ratio. A permitted division signature says nothing about the
divisor's value: zero still requires formula_division during execution. No implicit kind conversion
or division rounding is added here.

```rust
use sc_core::recipe::{FormulaBinaryOperator as B, FormulaKind as K};
assert_eq!(B::Multiply.result_kind(K::Length, K::Length), Some(K::Area));
assert_eq!(B::Multiply.result_kind(K::Ratio, K::Length), Some(K::Length));
assert_eq!(B::Divide.result_kind(K::Area, K::Length), Some(K::Length));
assert_eq!(B::Divide.result_kind(K::Length, K::Area), None);
assert_eq!(B::Divide.result_kind(K::Count, K::Ratio), Some(K::Count));
assert_eq!(B::Divide.result_kind(K::Ratio, K::Count), Some(K::Ratio));
assert_eq!(B::Equal.result_kind(K::Boolean, K::Boolean), None);
```

arc_length_hint(left, right) is true exactly for an angle-times-length multiplication in either
order. It identifies the normative guidance to use arc_length, which explicitly includes π and
rounding. It is false for division and every other kind pair. The contextual checker uses this
metadata when it constructs the typed formula_dimension refusal; this method itself renders no
user diagnostic and fetches no operands.

```rust
use sc_core::recipe::{FormulaBinaryOperator as B, FormulaKind as K};
assert_eq!(B::Multiply.result_kind(K::Angle, K::Length), None);
assert!(B::Multiply.arc_length_hint(K::Angle, K::Length));
assert!(B::Multiply.arc_length_hint(K::Length, K::Angle));
assert!(!B::Divide.arc_length_hint(K::Angle, K::Length));
```

## Verification boundary

Four public contracts enumerate all16 unary and640 ordered binary kind cases over the eight-kind
population. There are seven permitted unary cases and71 permitted binary cases, including17 ordered
multiplication pairs and14 directed quotient pairs. Exactly two cases carry the arc_length hint.
The tests compare the complete actual normative product/operator rows in both directions, and all
twelve metadata symbols agree with the existing independent canonical serializer.

```bash
cargo test -p sc-core --test formula_operator_signature_contract
python3 -I -B docs/tasks/artifacts/formula_structure/operator_signature_mutations.py
```

Run mutations alone: fourteen compiled faults must fail public body assertions, then restore the
exact source. They change tokens, count negation, square/product/comparison results, Boolean or
mismatched arithmetic, product commutativity, quotient direction/count rules and hint population.
The standing structural runner watches actual anchors and refuses compiler/unwrap/test-name noise.

The pure signature metadata establishes operand-kind rules. Name lookup, source locations and
[ordered metadata scopes](formula-name-scopes.md) keep their own contracts. Function/selector roles,
conditional checking, contextual dimensional errors and whole-recipe dependency proofs are implemented
by their separate signature and kind-checking APIs.
Numeric division/domain checks, quantization, geometry, execution and physical/release proof retain
their separate owners.
