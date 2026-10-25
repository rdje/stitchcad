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
| negated | Reverse sign exactly while preserving positive canonical zero |
| Ord / PartialOrd | Exact total ordering with bounded cross-products |
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
| UnaryDimension | formula_dimension with the actual unary operator and operand kind |
| BuiltinInputKind | formula_dimension with built-in, argument and actual kind; signatures retain wanted alternatives |
| NegativeSquareRoot | formula_domain with the actual negative area/ratio kind and exact value |
| BuiltinArity | formula_parse with the empty min/max built-in and actual count |
| BuiltinDimension | formula_dimension with built-in, zero-based argument, wanted and actual kinds |
| InvalidClamp | formula_domain with shared kind and exact low/high bounds |
| arc_length_hint | Identifies the absent angle-times-length product |
| Rational | Retains the underlying FormulaRationalError and diagnostic family |
| kind | Inspect the admitted quantity kind |
| rational | Inspect the exact canonical internal-unit value |
| from_literal | Import a normalized literal's once-rounded integer and kind |
| checked_add / checked_sub | Same-kind exact sum/difference with completed-result domains |
| checked_mul / checked_div | Permitted ordered signatures and exact ppm scaling before width/domain checks |
| checked_neg | Exact sign reversal for declared negatable kinds; Count refuses |
| checked_square | Exact length/ratio/Count square with scaling and result limits |
| checked_compare | One of the six exact same-kind comparisons, returning bool |
| abs | Exact total absolute value, retaining kind and canonical zero |
| checked_min / checked_max | One or more homogeneous numeric operands; returns an existing operand |
| checked_clamp | Explicit inclusive exact bounds of the same kind |
| checked_round_to | Authored same-kind step, nearest exact multiple with ties away from zero |
| checked_sqrt | Correctly rounded area-to-length or ratio-to-ratio square root |
| checked_hypot | Correctly rounded signed-length hypotenuse with no intermediate Area |
| checked_arc_length | Correctly rounded signed Angle/Length arc with full turns and certified pi |

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
constructor/operator token. Whole-statement binding/evaluator integration remain pending features.

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

### Unary operators and exact comparisons

checked_neg admits length, area, angle and ratio. Count refuses with UnaryDimension even when zero;
its nonnegative domain does not make unary negation a permitted signature. Negative zero remains
canonical positive zero. A formula angle preserves sign and complete turns after negation.
checked_square admits only length, ratio and Count. Length produces area; ratio keeps exact ppm
scaling, and fractional Count remains exact. Other unary kinds refuse before numeric execution.

```rust
use sc_core::recipe::{FormulaBinaryOperator, FormulaKind, FormulaNumericValue, FormulaRational};

let count = FormulaNumericValue::new(FormulaKind::Count, FormulaRational::new(3, 2, false)?)?;
assert_eq!(count.checked_square()?.rational(), FormulaRational::new(9, 4, false)?);
let larger = FormulaNumericValue::new(FormulaKind::Count, FormulaRational::new(2, 1, false)?)?;
assert!(count.checked_compare(FormulaBinaryOperator::Less, larger)?);
assert!(count.checked_neg().is_err());
# Ok::<(), Box<dyn std::error::Error>>(())
```

checked_compare accepts Equal, NotEqual, Less, LessEqual, Greater and GreaterEqual from the existing
operator enum. Operands must have the same numeric kind; a different kind or noncomparison
operator returns BinaryDimension. The returned bool is an exact primitive comparison, with no
source evaluation, binding, tolerance or irrational-result provenance inferred here. A complete
turn differs from angle zero. Comparison does not round fractions or subtract into a width-limited
intermediate result. FormulaRational implements total Ord/PartialOrd using at most256-bit exact
cross-products, so even near-equal full-width fractions stay ordered. Ordering allocates bounded
private workspace; sign reversal and zero canonicalization need no allocation.

### Exact abs, min, max and clamp

abs preserves kind and exact magnitude without rounding, even for negative magnitudes above the
signed integer range. Count is already nonnegative; zero remains positive. Length/area limits are
symmetric, so this total operation cannot create an out-of-domain result from an admitted value.

Min/max take a slice containing one or more numeric values. A single value returns itself;
multiple values use exact ordering and must all have the first value's kind. An empty slice returns
BuiltinArity in formula_parse. The first mismatched argument returns BuiltinDimension with its
zero-based index, wanted kind and actual kind. These value APIs do not parse or evaluate a call.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let half = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::new(1, 2, false)?)?;
let third = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::new(1, 3, false)?)?;
assert_eq!(FormulaNumericValue::checked_min(&[half, third])?, third);
assert_eq!(FormulaNumericValue::checked_max(&[half])?, half);
assert_eq!(half.checked_clamp(third, third)?, third);
# Ok::<(), Box<dyn std::error::Error>>(())
```

checked_clamp checks both bounds against the principal value's kind. The principal argument is
index zero, low is one and high is two. Exact low above high returns InvalidClamp with both supplied
bounds. Inclusive equal bounds are valid. A value below low returns low; one above high returns
high; otherwise the original value returns. This is explicit authored clamping. It cannot rescue
an invalid completed operand: every supplied value has already passed its own scalar-domain check.
Min/max/clamp return admitted operands, preserving fractions without intermediate rounding.

### Explicit authored round_to

checked_round_to takes a step of the same numeric kind and returns the nearest exact multiple,
with ties away from zero. The step remains part of the authored recipe; no coarser quantum is
introduced implicitly. Fractional steps stay fractional: Count 1/2 rounded to step 1/3 becomes
Count 2/3. This quantization does not perform the later integer binding conversion.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let value = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::new(1, 2, true)?)?;
let step = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::ONE)?;
assert_eq!(value.checked_round_to(step)?.rational(), FormulaRational::from_signed_integer(-1));
# Ok::<(), Box<dyn std::error::Error>>(())
```

A different step kind returns BuiltinDimension at argument one. A zero step returns a rational
zero-divisor refusal in formula_division, including zero rounded to zero step. Negative steps
specify the same multiples as positive steps for kinds that admit negative values; Count's
completed-value domain continues to forbid negative steps. Both signs round ties away from zero,
and a rounded zero is canonical positive zero. Ratio steps use the same ppm internal units as
the value; their step quotient has no extra ppm scale.

The private bounded workspace rounds the exact quotient before reducing the final multiple.
No intermediate quotient storage/128-bit bound applies: a256-bit quotient may still produce an
admitted final result. Only the complete reduced multiple crosses the128-bit return boundary,
then its scalar domain is checked. Maximum128-bit integer rounded to step2 reports129 numerator
bits; length1,000,000,000 rounded to step6 reports exact out-of-domain1,000,000,002 micrometres.
Signed complete angle turns survive. Source-bearing explicit-quantization provenance and tolerance
rules are still owned by later evaluator integration; this value API does not claim that proof.

### One binding conversion and immutable stored values

FormulaBoundValue converts an admitted completed numeric value into its signed i64 storage form.
It uses the shared unsigned-magnitude half-away rounding primitive, then applies signed storage
limits. This order preserves both endpoints, including i64 MIN, without narrowing a wide positive
literal child before unary negation. Length/area already satisfy their tighter exact integer scalar
bounds; nearest-integer rounding cannot cross those bounds. Count remains nonnegative.

| API | Available behavior |
| --- | --- |
| FormulaBoundValue | Private numeric integer/kind or Boolean payload |
| from_numeric | One binding rounding/storage check with an actual supplied name |
| from_boolean | Store Boolean directly without numeric conversion |
| kind | Inspect the stored numeric or Boolean kind |
| numeric_integer | Returned i64 integer, or None for Boolean |
| boolean | Returned Boolean, or None for numeric kinds |
| FormulaBindingValueError | Borrowed actual name, kind, typed rule and diagnostic family |
| FormulaBindingValueRule | Rounding refusal or signed Storage bounds and exact rounded witness |
| name / rule | Inspect the supplied MachineToken or typed refusal |
| Storage | Inclusive minimum/maximum and measured rounded integer |
| Rounding | Propagate the shared UnitError without substituting a fallback |

```rust
use sc_core::{name::MachineToken, recipe::{FormulaBoundValue, FormulaKind,
    FormulaNumericValue, FormulaRational}};

let name = MachineToken::new("saved_width")?;
let exact = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::new(1, 2, false)?)?;
let stored = FormulaBoundValue::from_numeric(&name, exact)?;
assert_eq!(stored.numeric_integer(), Some(1));
let read = FormulaNumericValue::new(stored.kind(), FormulaRational::from_signed_integer(1))?;
assert_eq!(read.checked_add(read)?.rational(), FormulaRational::from_signed_integer(2));

let condition = FormulaBoundValue::from_boolean(true);
assert_eq!(condition.kind(), FormulaKind::Boolean);
assert_eq!(condition.boolean(), Some(true));
assert_eq!(condition.numeric_integer(), None);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Subsequent reads must use the returned stored integer. Reusing the pre-binding half in this example
would produce the wrong later sum. An exact temporary wider than i64 can cancel before binding;
converting the oversized temporary itself refuses. Angle bindings preserve complete signed turns.
Boolean stays Boolean and has no numeric integer payload.

A Storage refusal is formula_domain and retains the actual borrowed MachineToken, kind, inclusive
storage bounds and exact once-rounded measured integer. The witness keeps full128-bit unsigned
magnitude and sign, so even a value beyond i128's positive range is reported without wrapping.
Count's storage interval is zero through i64 MAX; the other numeric kinds use i64 MIN through MAX.
No namespace mutation or partially stored value occurs. These APIs perform conversion only:
annotation checking, reserved/repeated-name authority, environment publication, source/provenance
and complete statement/recipe execution remain separate integration responsibilities.

Check the binding boundary with its affected native target:

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_binding_value_contract
```

The focused target passes three tests covering all numeric kinds, signed endpoint/tie neighbors,
full-width refusal witnesses, the direct positive literal child beneath unary i64 MIN, raw turns,
returned-integer reads and Boolean separation. Broad/platform verification remains deferred.

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
passes five tests, including 34 rows derived independently with Python Fraction arithmetic. The
primitive and quantity-aware operations are available as described above. Broad/platform work remains .5c.4/pre-push.

The fixture oracle imports no production arithmetic. Reproduce its expected rows with:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/rational_value_oracle.py
```

Its output is retained in crates/sc-core/tests/fixtures/formula_rationals.tsv and consumed by the
native target above. The rows cover full magnitudes, signs, canonical zero, wide cancellation,
zero division and fully reduced numerator/denominator width refusals. This is a focused comparison;
it does not claim exhaustive arithmetic or platform verification.

## Correctly rounded sqrt and hypot

checked_sqrt is available for area and ratio. Area returns length; ratio returns ratio in ppm.
Negative inputs return NegativeSquareRoot with their actual kind and exact value. Other kinds,
including zero length/angle/Count, return BuiltinInputKind with their actual argument/kind. The
named built-in retains the complete wanted signature alternatives. The result is rounded at the function quantum;
there is no intermediate approximation or i64 binding conversion.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let area = FormulaNumericValue::new(FormulaKind::Area, FormulaRational::new(9, 4, false)?)?;
let length = area.checked_sqrt()?;
assert_eq!(length.kind(), FormulaKind::Length);
assert_eq!(length.rational(), FormulaRational::from_signed_integer(2)); // sqrt(9/4) = 1.5, a tie

let quarter = FormulaNumericValue::new(FormulaKind::Ratio, FormulaRational::from_signed_integer(250_000))?;
assert_eq!(quarter.checked_sqrt()?.rational(), FormulaRational::from_signed_integer(500_000));
# Ok::<(), Box<dyn std::error::Error>>(())
```

The recorded root-family algorithm uses exact integers with the existing private bigint dependency.
checked_hypot is also available for two signed lengths. Transcendental functions retain a separate
pending algorithm obligation.

For z=N/D>=0, find k=floor(sqrt(floor(N/D))) by bounded integer bisection. This is floor(sqrt(z))
because comparing an integer square with z is equivalent to comparing it with floor(z). Round up
exactly when 4N>=D(2k+1)²; equality is a half tie and rounds away from zero. This compares the exact
squared midpoint instead of approximating a root. Bisection halves a finite integer interval and
has no numerical convergence guess.

Area sqrt uses its square-micrometre fraction directly. Ratio sqrt multiplies its internal ppm
radicand by1,000,000, so the integer root returns ppm. Negative root operands refuse. Hypot forms
the exact squared-length sum privately and roots it without creating an area value: a sum exceeding
the area domain can still have an admitted rounded length. Only the rounded function result is
checked against its scalar domain; no binding narrowing occurs inside the function.

Input/component domains bound area, ratio and hypot bisection to30,74 and31 iterations respectively.
Hypot's raw numerator/denominator need at most513/512 bits; the conservative midpoint comparison
needs at most576 bits. These are workspace bounds, not additional language limits. Focused
checks include exact half squares and rational neighbors, full128-bit ratio, zero/signed lengths,
Pythagorean cases and squared sums above the area domain. Source-call provenance and broad/platform
proof remain later owned integration work. Every sqrt call, including perfect roots, must retain its
contribution in the later source-bearing evaluator. This source-free value API supplies no such proof.

checked_hypot forms the exact squared sum from its two admitted Length operands and applies the
same integer root rule. Sign is retained in the operands; squaring gives a nonnegative result.
A first non-Length operand returns BuiltinInputKind with argument zero or one, including for zero
values. Only the final rounded Length has a scalar-domain check, with operation hypot and its exact
rounded witness. A zero pair returns canonical zero; there is no hidden area or binding conversion.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let x = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::from_signed_integer(-3))?;
let y = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::from_signed_integer(4))?;
assert_eq!(x.checked_hypot(y)?.rational(), FormulaRational::from_signed_integer(5));
# Ok::<(), Box<dyn std::error::Error>>(())
```

For example, lengths1,000,000,000 and1 micrometre have a squared sum above the Area limit, but
hypot returns the admitted rounded1,000,000,000 micrometres. Two maximum lengths return a scalar
refusal with rounded1,414,213,562, rather than a premature Area refusal. Near-unit fractions with
full128-bit components also work even though their unreduced squared sum needs513/512 bits.
Source-bearing evaluator provenance remains pending for both root methods, including perfect roots.

The pure fixture oracle is already retained at docs/tasks/artifacts/formula_structure/root_value_oracle.py.
It uses Python math.isqrt and Fraction arithmetic;25 expected rows live in
crates/sc-core/tests/fixtures/formula_roots.tsv. The focused native target passes four tests, including
all15 sqrt and ten hypot rows. Exact midpoint neighbors beyond ordinary float
precision, full128-bit ratio, negative inputs and actual kind refusals are covered. Authored half-square,
full-width and over-domain values check the oracle's role; no broad/platform pass is claimed.

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_root_value_contract
```

## Bounded correctly rounded arc_length

checked_arc_length is available for an Angle followed by a Length radius. It returns the nearest
micrometre, preserving signed/full turns and both operand signs. Zero products return canonical
zero. A different kind returns BuiltinInputKind with the actual first wrong argument and kind.
The existing integer dependency and recorded pi certificate establish exact rounding across the
admitted input family; the remaining trigonometric/inverse functions retain separate plans.

```rust
use sc_core::recipe::{FormulaKind, FormulaNumericValue, FormulaRational};

let angle = FormulaNumericValue::new(FormulaKind::Angle, FormulaRational::from_signed_integer(720_000_000))?;
let radius = FormulaNumericValue::new(FormulaKind::Length, FormulaRational::ONE)?;
assert_eq!(angle.checked_arc_length(radius)?.rational(), FormulaRational::from_signed_integer(13));
// Two full turns at radius 1 micrometre: round(4*pi), with no direction modulo.
# Ok::<(), Box<dyn std::error::Error>>(())
```

The complete exact product is rounded once at the function quantum. Excessive returned component
width reports the exact numerator bits; an excessive rounded Length reports operation arc_length,
its actual integer and the signed scalar bounds. There is no premature i64 binding check or
intermediate rounding. Negative radius values follow the current signed numeric product contract.

For operand magnitudes n1/d1 and n2/d2, arc length is A*pi/B with A=n1*n2<2^256 and
B=180,000,000*d1*d2. A half-micrometre output would place pi at a rational number whose reduced
denominator is below2^257. The certificate encloses pi in a small interval strictly inside two
adjacent bounded-denominator fractions. Their determinant is1 and their denominator sum exceeds
2^257; elementary integer arithmetic then excludes every such midpoint from the entire enclosure.
Consequently rounding either endpoint gives the same answer as rounding the true value for every
admitted input. Zero is handled exactly and signs are restored after magnitude rounding.

This converts the difficult precision question into a finite certificate. Exact alternating Fraction
sums and a different directed-integer arctangent identity independently establish the same640-bit
pi enclosure. Its neighbor/determinant/denominator predicates are retained in
crates/sc-core/tests/fixtures/pi640_certificate.tsv. Reproduce them with:

```bash
python3 -I -B docs/tasks/artifacts/formula_structure/pi_rounding_certificate.py
```

The full derivation is in the numerical fixed-point decision. Runtime work stays within898/924
numerator/denominator bits and uses exact quotient/remainder rounding before result-width and
Length-domain checks. The precision is a checked internal certificate, with no added language limit,
platform libm dependency or empirical convergence cutoff. Source-call provenance and platform/broad
verification remain later integration owners.

The compiled pi limbs are checked against the retained certificate by a native library test that
re-runs its two-identity producer. The independent arc oracle uses a different pi identity and exact
Fraction enclosures. Its23 rows cover signed/full turns, zero, full128-bit components,133-bit
refusals, scalar excess and rational neighbors straddling half-micrometre boundaries. The native
arc target passes two tests; the affected numeric target retains six passing cases. The selected
library run also passed67 unit tests, including the new certificate check. Broad verification
remains deferred. Future focused invocations filter that library check explicitly:

```bash
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --lib compiled_pi_endpoint_matches_the_reproduced_certificate
python3 -I -B scripts/local_environment.py -- cargo test --offline -p sc-core --test formula_arc_value_contract --test formula_numeric_contract
```

## Remaining trig/inverse plan

Sin/cos/tan and atan/atan2 remain unimplemented. For the actual ppm/microdegree quanta,
rational-value restrictions exclude exact rounding midpoints, so converging valid enclosures would
eventually decide each finite admitted input after exact pole/axis/branch handling. This is a
qualitative termination proof. A generic algebraic separation estimate gives an impractical upper
bound; it does not establish actual hard inputs or justify an arbitrary precision cutoff.
The full derivation/sources are in docs/decisions/fact_trig-midpoints-and-resource-bound.md;
G1-SLICE.5d.1.2.2.2 still owns usable all-input resource accounting before implementation.
