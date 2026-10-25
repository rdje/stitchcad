# Canonical formula rational values

The recipe API now constructs immutable exact rational values with independent sign and full
128-bit numerator and denominator magnitudes. Construction reduces fractions without rounding.
FormulaRational construction and its unscaled products/quotients allocate no workspace. Sums use a private bounded
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
Whole-expression numeric execution and evaluator integration remain later slices.
The 128-bit result limit remains unchanged.

## Completed numeric values and exact domains

FormulaNumericValue pairs an immutable quantity kind with its exact rational internal-unit value.
Its constructor admits only length, angle, area, ratio and count. Boolean, point and edge refuse as
nonnumeric kinds. The quantity layer preserves fractions; it applies no binding/storage rounding.

| API | Available behavior |
| --- | --- |
| FormulaNumericValue | Private admitted kind and exact internal-unit rational |
| FormulaNumericError | Typed nonnumeric-kind, scalar-domain or rational refusal |
| NonNumericKind | formula_dimension with the actual refused kind |
| ScalarDomain | formula_domain with actual operation, kind, exact value, minimum and optional maximum |
| BinaryDimension | formula_dimension with the actual operator and left/right kinds |
| arc_length_hint | Identifies the absent angle-times-length product |
| Rational | Retains the underlying FormulaRationalError and diagnostic family |
| kind | Inspect the admitted quantity kind |
| rational | Inspect the exact canonical internal-unit value |
| from_literal | Import a normalized literal's once-rounded integer and kind |
| checked_add / checked_sub | Same-kind exact sum/difference with completed-result domains |
| checked_mul / checked_div | Permitted ordered signatures and exact ppm scaling before width/domain checks |

Length and area use inclusive signed limits of 10⁹ micrometres and 10¹⁸ square micrometres.
The check compares the exact fraction with the bound: limit + 1/10 refuses even if binding would
round it back to the limit. Negative lengths/areas are admitted inside the same symmetric limits.
Count admits nonnegative fractions and refuses every negative fraction. An absent maximum in a
Count refusal means no scalar upper bound; signed i64 storage is checked later at binding.
Angle and ratio retain their full admitted rational range at this boundary. Angles preserve signed
complete turns. Ratios are measured in parts per million, so exactly 1.0 is 1,000,000 internal units.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let fraction = FormulaRational::new(1, 3, false)?;
let count = FormulaNumericValue::new(FormulaKind::Count, fraction)?;
assert_eq!(count.rational(), fraction); // no early integer rounding

let excess = FormulaRational::new(10_000_000_001, 10, false)?;
assert!(FormulaNumericValue::new(FormulaKind::Length, excess).is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

The normalized-literal bridge performs no new conversion. For example, a normalized 0.0000004 ratio
is already zero ppm; importing it cannot recover a hidden fraction. A normalized 720 deg input
remains 720,000,000 microdegrees. The error contains actual kind/value/bounds; expression
and statement context belongs to later evaluator integration. Scalar refusals retain the actual
constructor/operator token. Unary operators, comparison, built-ins and binding remain pending features.

### Exact quantity operators

The four methods reuse the normative kind signatures. Addition/subtraction require the same kind.
Products can derive area from two lengths; ordered quotients can derive length from area/length or
ratio from equal dimensions. An absent pair refuses with its actual operator and operand kinds.
Angle times length includes the arc_length hint, even for zero values. A valid zero divisor refuses
before a zero numerator shortcut; no zero quotient is fabricated.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let length = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::new(1, 3, false)?)?;
let half = FormulaNumericValue::new(FormulaKind::Ratio, FormulaRational::new(500_000, 1, false)?)?;
assert_eq!(length.checked_mul(half)?.rational(), FormulaRational::new(1, 6, false)?);
let quotient = length.checked_div(length)?;
assert_eq!(quotient.kind(), FormulaKind::Ratio);
assert_eq!(quotient.rational(), FormulaRational::from_signed_integer(1_000_000));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The quantity layer scales ratios exactly as ppm. A product of two 500,000 ppm values is 250,000 ppm;
Count 3 divided by a 2,000,000 ppm ratio is exact Count 3/2. No implicit rounding occurs.
Scaling and arithmetic form one completed operator result: intermediate scaling products do not
trigger the reduced128-bit limit. A private workspace bounds three rational factors to384 bits,
reduces the complete result, then checks the returned rational and scalar domain. For example, a
maximum128-bit ppm numerator times 2 ppm is admitted when the scaled reduced result fits; the same
numerator times Count 2 correctly reports129 bits. A tiny denominator can also exceed128 bits
because ppm scaling belongs to the actual result, and must refuse with its exact measured width.

Every completed result is admitted independently. Length at its limit plus 1/10 micrometre refuses
immediately, so a later subtraction cannot rescue it. Count 1 minus Count 2 refuses a negative
result. Error bounds and measured values remain exact; arithmetic and scalar failures expose no
partially computed value. Binding and whole-recipe replay remain later work.

Check this feature with the affected native target:

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_numeric_contract
```

The focused constructor/input cases cover signed exact limits, fractional Count, wider temporaries,
refused geometry/Boolean kinds, normalized rounding and raw multi-turn angles. Broad verification
remains deferred to integration/pre-push.

## Focused check

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_rational_contract
```

The focused public target covers construction and all four exact operations, including cancellation
before width checks, typed numerator/denominator witnesses and zero-divisor priority. Native execution
passes four tests, including 34 rows derived independently with Python Fraction arithmetic. The
primitive is complete; quantity-aware operations follow. Broad/platform work remains .5c.4/pre-push.

The fixture oracle imports no production arithmetic. Reproduce its expected rows with:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/rational_value_oracle.py
```

Its output is retained in crates/sc-core/tests/fixtures/formula_rationals.tsv and consumed by the
native target above. The rows cover full magnitudes, signs, canonical zero, wide cancellation,
zero division and fully reduced numerator/denominator width refusals. This is a focused comparison;
it does not claim exhaustive arithmetic or platform verification.
