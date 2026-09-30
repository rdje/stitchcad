//! The bounded exact rational — the representation of a parameterized reference's position.
//!
//! The normative source is `docs/book/src/spec/ontology.md` §1 ("the parameter is a rational in
//! `[0, 1]` of that edge's own length, so it survives a change of tessellation and a change of units")
//! and §1.1, which recomputes that parameter on every split, merge and reverse. The coupled decision is
//! `docs/decisions/decision_edge-parameter-bounded-exact-rational.md`.
//!
//! This is **not** `sc_units::Ratio`, which is a parts-per-million *fixed point*: it rounds on
//! construction, so recomputing `t/s` in ppm would round on every edit and accumulate drift — exactly
//! what reference stability forbids. A [`Rational`] keeps an integer numerator and denominator and the
//! four operators **never round**, so a parameter survives an unbounded number of edits with no drift.
//!
//! It is **bounded**: the numerator and denominator are `i64`, and arithmetic widens to `i128` for the
//! intermediate product and sum. A result whose reduced form does not fit `i64` is a typed
//! [`UnitError::Overflow`], never a silent wrap or clamp. (The formula language's rational, by contrast,
//! is arbitrary-precision and internal to evaluation — `docs/book/src/spec/formula-language.md` §4.2 —
//! and is a different concern landed by `G1-SLICE.5`.)

use core::cmp::Ordering;
use core::fmt;

use sc_units::UnitError;

/// An exact rational number `numerator / denominator` in reduced canonical form.
///
/// The canonical form is `denominator > 0` and `gcd(|numerator|, denominator) == 1`, maintained by every
/// constructor and operator. Two consequences the identity contract relies on:
///
/// - equality is **structural** (`num == num && den == den`), so a parameter recomputed by a different
///   route compares equal to one recomputed exactly, with no epsilon;
/// - ordering is **exact** cross-multiplication, so the split-point case `t == s` is decided correctly
///   rather than within a tolerance.
///
/// `Default` is [`Rational::ZERO`] (`0/1`), never the invalid `0/0` a field-wise default would produce.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rational {
    num: i64,
    den: i64,
}

impl Default for Rational {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rational {
    /// The rational `0` (canonical form `0/1`).
    pub const ZERO: Self = Self { num: 0, den: 1 };

    /// The rational `1` (canonical form `1/1`).
    pub const ONE: Self = Self { num: 1, den: 1 };

    /// Builds the reduced rational `numerator / denominator`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `denominator` is zero, and [`UnitError::Overflow`] for
    /// the one input whose reduced magnitude does not fit `i64` — `i64::MIN / -1`, which is `2^63`.
    pub fn new(numerator: i64, denominator: i64) -> Result<Self, UnitError> {
        reduce(
            i128::from(numerator),
            i128::from(denominator),
            "Rational::new",
        )
    }

    /// Builds the rational `n / 1`. Infallible: an integer is already in canonical form.
    #[must_use]
    pub const fn from_integer(n: i64) -> Self {
        Self { num: n, den: 1 }
    }

    /// The numerator of the reduced form.
    #[must_use]
    pub const fn numerator(self) -> i64 {
        self.num
    }

    /// The denominator of the reduced form; always positive.
    #[must_use]
    pub const fn denominator(self) -> i64 {
        self.den
    }

    /// Whether this is the zero rational.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.num == 0
    }

    /// Whether `0 ≤ self ≤ 1` — the range an edge parameter must be in (ontology §1).
    ///
    /// Exact and division-free: with a positive denominator, `0 ≤ num/den ≤ 1` is `0 ≤ num ≤ den`.
    #[must_use]
    pub const fn in_unit_interval(self) -> bool {
        self.num >= 0 && self.num <= self.den
    }

    /// The exact sum `self + rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] when the reduced result does not fit `i64`.
    pub fn checked_add(self, rhs: Self) -> Result<Self, UnitError> {
        // n1/d1 + n2/d2 = (n1·d2 + n2·d1) / (d1·d2). For i64 fields the products and their sum are
        // bounded by 2^127 - 2^65 + 2, strictly inside i128, so these intermediates cannot overflow; only
        // the reduced result's return to i64 can, and `reduce` reports that.
        let num =
            i128::from(self.num) * i128::from(rhs.den) + i128::from(rhs.num) * i128::from(self.den);
        let den = i128::from(self.den) * i128::from(rhs.den);
        reduce(num, den, "Rational::checked_add")
    }

    /// The exact difference `self - rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] when the reduced result does not fit `i64`.
    pub fn checked_sub(self, rhs: Self) -> Result<Self, UnitError> {
        let num =
            i128::from(self.num) * i128::from(rhs.den) - i128::from(rhs.num) * i128::from(self.den);
        let den = i128::from(self.den) * i128::from(rhs.den);
        reduce(num, den, "Rational::checked_sub")
    }

    /// The exact product `self * rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] when the reduced result does not fit `i64`.
    pub fn checked_mul(self, rhs: Self) -> Result<Self, UnitError> {
        let num = i128::from(self.num) * i128::from(rhs.num);
        let den = i128::from(self.den) * i128::from(rhs.den);
        reduce(num, den, "Rational::checked_mul")
    }

    /// The exact quotient `self / rhs`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::DivisionByZero`] when `rhs` is zero and [`UnitError::Overflow`] when the
    /// reduced result does not fit `i64`.
    pub fn checked_div(self, rhs: Self) -> Result<Self, UnitError> {
        if rhs.num == 0 {
            return Err(UnitError::DivisionByZero {
                operation: "Rational::checked_div",
            });
        }
        let num = i128::from(self.num) * i128::from(rhs.den);
        let den = i128::from(self.den) * i128::from(rhs.num);
        reduce(num, den, "Rational::checked_div")
    }

    /// The exact negation `-self`.
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] for `i64::MIN / 1`, whose negation does not fit `i64`.
    pub fn checked_neg(self) -> Result<Self, UnitError> {
        Self::ZERO.checked_sub(self)
    }

    /// The exact reflection `1 - self`, which is how reversing an edge maps a parameter `t` to `1 − t`
    /// (ontology §1.1).
    ///
    /// # Errors
    ///
    /// Returns [`UnitError::Overflow`] when the reduced result does not fit `i64`.
    pub fn one_minus(self) -> Result<Self, UnitError> {
        Self::ONE.checked_sub(self)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        // Cross-multiply in i128: with positive denominators, n1/d1 ⋚ n2/d2 iff n1·d2 ⋚ n2·d1. The
        // products of two i64 fit i128, so this is exact and total — no epsilon, which is what makes the
        // split-point case `t == s` decidable rather than approximate.
        let lhs = i128::from(self.num) * i128::from(other.den);
        let rhs = i128::from(other.num) * i128::from(self.den);
        lhs.cmp(&rhs)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/// Reduces `num / den` to the canonical `i64` form: positive denominator, coprime, sign on the numerator.
///
/// Reduction divides both by their gcd, so a reduced magnitude never exceeds the input magnitude; the
/// only way the result leaves `i64` is the genuine overflow the caller must report (an arithmetic result
/// too large, or `i64::MIN / -1`). Division is exact because the gcd divides both, and negation cannot
/// overflow `i128` because every `num`/`den` reaching here is strictly inside `(-2^127, 2^127)`.
fn reduce(num: i128, den: i128, operation: &'static str) -> Result<Rational, UnitError> {
    if den == 0 {
        return Err(UnitError::DivisionByZero { operation });
    }
    let g = gcd(num.unsigned_abs(), den.unsigned_abs()) as i128;
    let mut n = num / g;
    let mut d = den / g;
    if d < 0 {
        n = -n;
        d = -d;
    }
    let num_i64 = i64::try_from(n).map_err(|_| UnitError::Overflow { operation })?;
    let den_i64 = i64::try_from(d).map_err(|_| UnitError::Overflow { operation })?;
    Ok(Rational {
        num: num_i64,
        den: den_i64,
    })
}

/// The greatest common divisor of two magnitudes, by Euclid's algorithm. `gcd(a, 0) == a`.
const fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::{gcd, Rational};
    use sc_units::UnitError;

    #[test]
    fn construction_reduces_to_canonical_form() {
        let r = Rational::new(2, 4).unwrap();
        assert_eq!((r.numerator(), r.denominator()), (1, 2));
        let neg = Rational::new(3, -9).unwrap();
        assert_eq!(
            (neg.numerator(), neg.denominator()),
            (-1, 3),
            "the sign moves to the numerator and the denominator stays positive"
        );
        let zero = Rational::new(0, 5).unwrap();
        assert_eq!((zero.numerator(), zero.denominator()), (0, 1));
    }

    #[test]
    fn a_zero_denominator_is_a_diagnostic() {
        assert_eq!(
            Rational::new(1, 0).unwrap_err(),
            UnitError::DivisionByZero {
                operation: "Rational::new"
            }
        );
    }

    #[test]
    fn arithmetic_is_exact_and_never_rounds() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 6).unwrap();
        assert_eq!(a.checked_add(b).unwrap(), Rational::new(1, 2).unwrap());
        assert_eq!(a.checked_sub(b).unwrap(), Rational::new(1, 6).unwrap());
        assert_eq!(a.checked_mul(b).unwrap(), Rational::new(1, 18).unwrap());
        assert_eq!(a.checked_div(b).unwrap(), Rational::from_integer(2));
    }

    #[test]
    fn division_by_zero_is_a_diagnostic() {
        let a = Rational::new(1, 2).unwrap();
        assert_eq!(
            a.checked_div(Rational::ZERO).unwrap_err(),
            UnitError::DivisionByZero {
                operation: "Rational::checked_div"
            }
        );
    }

    #[test]
    fn a_result_past_i64_is_an_overflow_not_a_wrap() {
        // Consecutive large denominators are coprime, so the sum's reduced numerator (2^64 - 5) and
        // denominator both leave i64: the diagnostic fires rather than wrapping.
        let a = Rational::new(1, i64::MAX - 1).unwrap();
        let b = Rational::new(1, i64::MAX - 2).unwrap();
        let err = a.checked_add(b).unwrap_err();
        assert!(matches!(err, UnitError::Overflow { .. }), "got {err:?}");
    }

    #[test]
    fn the_one_input_that_overflows_i64_is_reported() {
        // i64::MIN / -1 is +2^63, which does not fit i64.
        let err = Rational::new(i64::MIN, -1).unwrap_err();
        assert!(matches!(err, UnitError::Overflow { .. }), "got {err:?}");
        // i64::MIN / 1 is fine: the value fits, only its negation would not.
        assert_eq!(Rational::new(i64::MIN, 1).unwrap().numerator(), i64::MIN);
        assert!(Rational::from_integer(i64::MIN).checked_neg().is_err());
    }

    #[test]
    fn one_minus_reflects_the_parameter_as_reverse_requires() {
        let t = Rational::new(1, 4).unwrap();
        assert_eq!(t.one_minus().unwrap(), Rational::new(3, 4).unwrap());
        assert_eq!(Rational::ZERO.one_minus().unwrap(), Rational::ONE);
        assert_eq!(Rational::ONE.one_minus().unwrap(), Rational::ZERO);
    }

    #[test]
    fn the_unit_interval_predicate_is_exact_at_its_ends() {
        assert!(Rational::ZERO.in_unit_interval());
        assert!(Rational::ONE.in_unit_interval());
        assert!(Rational::new(1, 2).unwrap().in_unit_interval());
        assert!(!Rational::new(-1, 2).unwrap().in_unit_interval());
        assert!(!Rational::new(3, 2).unwrap().in_unit_interval());
    }

    #[test]
    fn ordering_is_exact_and_total() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 2).unwrap();
        assert!(a < b);
        assert!(b > a);
        // Equal values reached by different routes compare equal — the property reference stability rests on.
        assert_eq!(Rational::new(2, 4).unwrap(), Rational::new(1, 2).unwrap());
        assert_eq!(
            Rational::new(2, 4)
                .unwrap()
                .cmp(&Rational::new(1, 2).unwrap()),
            core::cmp::Ordering::Equal
        );
    }

    #[test]
    fn display_shows_the_exact_value() {
        assert_eq!(Rational::new(1, 2).unwrap().to_string(), "1/2");
        assert_eq!(Rational::from_integer(3).to_string(), "3");
        assert_eq!(Rational::new(-1, 3).unwrap().to_string(), "-1/3");
    }

    #[test]
    fn default_is_zero_not_an_invalid_denominator() {
        let d = Rational::default();
        assert_eq!(d, Rational::ZERO);
        assert_eq!(d.denominator(), 1);
    }

    #[test]
    fn gcd_is_correct_on_coprime_and_shared_factors() {
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(7, 13), 1);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }
}
