# Canonical formula rational values

The recipe API now constructs immutable exact rational values with independent sign and full
128-bit numerator and denominator magnitudes. Construction reduces fractions without rounding.
It uses no allocation or added dependency.

| API | Available behavior |
| --- | --- |
| FormulaRational | Private, reduced numerator/positive denominator with a separate sign |
| FormulaRationalError | Typed construction refusal; no fabricated statement or expression context |
| ZeroDenominator | Refuses every zero denominator, including 0/0; diagnostic family formula_division |
| new | Constructs from numerator magnitude, denominator and negative flag |
| from_signed_integer | Exact signed integer conversion, including i128 MIN |
| numerator_magnitude | Inspect the reduced unsigned numerator |
| denominator | Inspect the reduced positive denominator |
| is_negative | Inspect the independent sign; zero is positive |
| is_zero | Inspect canonical zero, whose components are 0/1 |

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

This is the canonical value representation for the [numeric contract](../spec/formula-language.md#42-exact-arithmetic-and-the-two-places-a-value-rounds).
It applies no quantity kind, scalar domain, signed storage conversion or source binding.
Arithmetic, wider intermediate reduction, measured width refusals and evaluator integration
remain the next implementation slices. The 128-bit result limit remains unchanged.

## Focused check

```bash
python3 -I -B scripts/local_environment.py -- cargo test -p sc-core --test formula_rational_contract
```

The focused public contract checks reduction, independent signs, canonical zero, zero-denominator
refusal and full-width endpoint construction. Native execution passes one test. Broad arithmetic,
oracle and platform verification remain owned by .5c.1.2/.3 and later integration milestones.
