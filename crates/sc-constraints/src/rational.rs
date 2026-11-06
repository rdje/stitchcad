//! Exact rationals. Every operation that could overflow says so instead of rounding.
use std::cmp::Ordering;
use std::fmt;

/// An exact fraction in lowest terms with a positive denominator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rational {
    num: i128,
    den: i128,
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a.abs()
}

impl Rational {
    /// The value zero.
    pub const ZERO: Rational = Rational { num: 0, den: 1 };

    /// A fraction from authored numbers. `None` when the denominator is zero.
    pub fn new(num: i64, den: i64) -> Option<Rational> {
        Rational::reduced(i128::from(num), i128::from(den))
    }

    /// A whole number.
    pub fn int(value: i64) -> Rational {
        Rational {
            num: i128::from(value),
            den: 1,
        }
    }

    fn reduced(num: i128, den: i128) -> Option<Rational> {
        if den == 0 || num == i128::MIN || den == i128::MIN {
            return None;
        }
        let divisor = gcd(num, den);
        let (num, den) = (num.checked_div(divisor)?, den.checked_div(divisor)?);
        if den < 0 {
            Some(Rational {
                num: num.checked_neg()?,
                den: den.checked_neg()?,
            })
        } else {
            Some(Rational { num, den })
        }
    }

    /// The numerator in lowest terms.
    pub fn numerator(self) -> i128 {
        self.num
    }

    /// The denominator in lowest terms, always positive.
    pub fn denominator(self) -> i128 {
        self.den
    }

    /// Sum, or `None` when the exact result does not fit.
    pub fn checked_add(self, other: Rational) -> Option<Rational> {
        let num = self
            .num
            .checked_mul(other.den)?
            .checked_add(other.num.checked_mul(self.den)?)?;
        Rational::reduced(num, self.den.checked_mul(other.den)?)
    }

    /// Product, or `None` when the exact result does not fit.
    pub fn checked_mul(self, other: Rational) -> Option<Rational> {
        Rational::reduced(
            self.num.checked_mul(other.num)?,
            self.den.checked_mul(other.den)?,
        )
    }

    /// Negation. Values never hold `i128::MIN`, so this cannot overflow.
    pub fn checked_neg(self) -> Option<Rational> {
        Some(Rational {
            num: self.num.checked_neg()?,
            den: self.den,
        })
    }

    /// Exact order, or `None` when the comparison does not fit.
    pub fn checked_cmp(self, other: Rational) -> Option<Ordering> {
        Some(
            self.num
                .checked_mul(other.den)?
                .cmp(&other.num.checked_mul(self.den)?),
        )
    }

    /// The sign as an ordering against zero. Never overflows.
    pub fn sign(self) -> Ordering {
        self.num.cmp(&0)
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
