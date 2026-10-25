# Canonical formula rational values

The recipe API now constructs immutable exact rational values with independent sign and full
128-bit numerator and denominator magnitudes. Construction reduces fractions without rounding.
It uses no allocation or added dependency.

| API | Available behavior |
| --- | --- |
| FormulaRational | Private, reduced numerator/positive denominator with a separate sign |
| FormulaRationalError | Typed value refusal; no fabricated statement or expression context |
| ZeroDenominator | Refuses every zero denominator, including 0/0; diagnostic family formula_division |
| ZeroDivisor | Refuses a zero quotient divisor, including 0/0; diagnostic family formula_division |
| RationalWidth | Refuses an excessive fully reduced component with exact measured_bits |
| measured_bits | Bit count of the excessive numerator or denominator after cross-cancellation |
| new | Constructs from numerator magnitude, denominator and negative flag |
| from_signed_integer | Exact signed integer conversion, including i128 MIN |
| numerator_magnitude | Inspect the reduced unsigned numerator |
| denominator | Inspect the reduced positive denominator |
| is_negative | Inspect the independent sign; zero is positive |
| is_zero | Inspect canonical zero, whose components are 0/1 |
| checked_mul | Exact product with cross-cancellation before reduced-width checks |
| checked_div | Exact quotient with zero-divisor validation before zero handling |
| rational_bit_bound | Returns 128 for a width refusal |
| limit_token | Returns max_rational_bits for a width refusal |

## Construct and inspect an exact fraction

```rust
use sc_core::recipe::FormulaRational;

let value = FormulaRational::new(6, 8, true)?;
assert_eq!(value.numerator_magnitude(), 3);
assert_eq!(value.denominator(), 4);
assert!(value.is_negative()); // exactly -3/4

let zero = FormulaRational::new(0, u128::MAX, true)?;
assert_eq!(zero, FormulaRational::ZERO);
assert!(!zero.is_negative());
# Ok::<(), Box<dyn std::error::Error>>(())
```

Equivalent fractions compare equal after reduction. A default value is canonical zero.
The constructor validates the denominator before canonicalizing zero, so 0/0 refuses.
No component is narrowed through a signed integer: `u128::MAX` is admitted as a numerator
magnitude with either sign, or as a positive denominator. Signed integer conversion retains
the exact magnitude of `i128::MIN`.

```rust
use sc_core::recipe::{FormulaRational, FormulaRationalError};

assert_eq!(
    FormulaRational::new(1, 0, false),
    Err(FormulaRationalError::ZeroDenominator),
);
```

## Exact products and quotients

```rust
use sc_core::recipe::FormulaRational;

let left = FormulaRational::new(u128::MAX, 2, false)?;
let right = FormulaRational::new(2, u128::MAX, true)?;
assert_eq!(left.checked_mul(right)?, FormulaRational::from_signed_integer(-1));
assert_eq!(left.checked_div(left)?, FormulaRational::ONE);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Both input fractions are already reduced. The methods remove cross factors before forming the
result components, so a wide raw product that reduces into the admitted range remains valid.
No intermediate rounding occurs. Zero products and admitted zero quotients return canonical zero;
every zero divisor refuses, including when the numerator is also zero.

A fully reduced component wider than 128 bits returns RationalWidth in the formula_domain family.
The error identifies numerator or denominator, exact bit count, bound 128 and max_rational_bits token.
For example, multiplying the maximum numerator by 2 reports 129 numerator bits; multiplying two maximum
numerators reports 256. Bounded 64-bit partial products determine that width without allocation or
an added dependency. No excessive value is returned.

This is the value kernel for the [numeric contract](../spec/formula-language.md#42-exact-arithmetic-and-the-two-places-a-value-rounds).
It applies no quantity kind, scalar domain, signed storage conversion or source binding.
Sums/differences, quantity-aware operator wiring and evaluator integration remain later slices.
The 128-bit result limit remains unchanged.

## Focused check

```bash
python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test formula_rational_contract
```

The focused public target covers construction and exact products/quotients, including cancellation
before width checks, typed numerator/denominator witnesses and zero-divisor priority. Native execution
passes two tests. Wider arithmetic, independent oracle and platform work remain owned by .5c.1.2.2/.3.
