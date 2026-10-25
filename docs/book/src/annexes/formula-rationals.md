# Canonical formula rational values

The recipe API now constructs immutable exact rational values with independent sign and full
128-bit numerator and denominator magnitudes. Construction reduces fractions without rounding.
Construction and products/quotients allocate no workspace. Sums/differences use a private bounded
wide-integer workspace; the public value retains its fixed 128-bit components.

| API | Available behavior |
| --- | --- |
| FormulaRational | Private, reduced numerator/positive denominator with a separate sign |
| FormulaRationalError | Typed value refusal; no fabricated statement or expression context |
| ZeroDenominator | Refuses every zero denominator, including 0/0; diagnostic family formula_division |
| ZeroDivisor | Refuses a zero quotient divisor, including 0/0; diagnostic family formula_division |
| RationalWidth | Refuses an excessive fully reduced component with exact measured_bits |
| measured_bits | Bit count of the excessive numerator or denominator after full reduction |
| new | Constructs from numerator magnitude, denominator and negative flag |
| from_signed_integer | Exact signed integer conversion, including i128 MIN |
| numerator_magnitude | Inspect the reduced unsigned numerator |
| denominator | Inspect the reduced positive denominator |
| is_negative | Inspect the independent sign; zero is positive |
| is_zero | Inspect canonical zero, whose components are 0/1 |
| checked_mul | Exact product with cross-cancellation before reduced-width checks |
| checked_div | Exact quotient with zero-divisor validation before zero handling |
| checked_add | Exact sum, reducing wider intermediates before checking widths |
| checked_sub | Exact signed difference, preserving cancellation and canonical zero |
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
a wide-integer dependency on this path. No excessive value is returned.

## Exact sums and differences

```rust
use sc_core::recipe::FormulaRational;

let two_thirds = FormulaRational::new(2, 3, false)?;
let half = FormulaRational::new(1, 2, false)?;
assert_eq!(two_thirds.checked_sub(half)?, FormulaRational::new(1, 6, false)?);
assert_eq!(half.checked_sub(two_thirds)?, FormulaRational::new(1, 6, true)?);

let maximum = u128::MAX;
let wide = FormulaRational::new(maximum, maximum - 1, false)?;
assert_eq!(
    wide.checked_add(wide)?,
    FormulaRational::new(maximum, (maximum - 1) / 2, false)?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

The last sum has a 257-bit raw numerator but an admitted reduced result. Checking raw components
would incorrectly refuse it. The private num-bigint workspace admits at most 257 numerator bits
and 256 denominator bits from two admitted operands, then reduces by the greatest common divisor.
Only the reduced components cross the 128-bit value boundary. Opposite equal values produce positive
canonical zero. Subtraction changes the right operand's sign exactly, without signed narrowing.

A genuinely excessive reduced sum returns the same typed RationalWidth refusal: maximum integer
plus one reports 129 numerator bits; 1/maximum minus 1/(maximum-1) reports 256 denominator bits.
No intermediate rounding or partially computed value escapes. The pinned dependency and rationale
are recorded in the repository's edge/formula rational decision; platform verification remains pending.

This is the value kernel for the [numeric contract](../spec/formula-language.md#42-exact-arithmetic-and-the-two-places-a-value-rounds).
It applies no quantity kind, scalar domain, signed storage conversion or source binding.
Quantity-aware operator wiring and evaluator integration remain later slices.
The 128-bit result limit remains unchanged.

## Focused check

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_rational_contract
```

The focused public target covers construction and all four exact operations, including cancellation
before width checks, typed numerator/denominator witnesses and zero-divisor priority. Native execution
passes three tests. Independent oracle integration and platform work remain owned by .5c.1.3.
