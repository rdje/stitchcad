//! Canonical private unit-interval fractions with guarded integer operands/results.
use super::super::super::Param;
use core::cmp::Ordering;
use num_bigint::BigUint;
use sc_units::UnitError;

const OPERATION: &str = "OriginalPointAdmission::project";
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Wide {
    pub(super) num: BigUint,
    pub(super) den: BigUint,
}
#[derive(Clone, Copy)]
pub(super) struct Workspace {
    pub(super) component_bits: u64,
    pub(super) integer_bits: u64,
}
impl Workspace {
    fn admit(self, bits: u64) -> Result<(), UnitError> {
        if bits > self.integer_bits {
            return Err(UnitError::DomainExceeded {
                operation: OPERATION,
                kind: "integer workspace bits",
                value: i128::from(bits),
                limit: i128::from(self.integer_bits),
            });
        }
        Ok(())
    }
    fn mul(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(a.bits().checked_add(b.bits()).ok_or(UnitError::Overflow {
            operation: OPERATION,
        })?)?;
        Ok(a * b)
    }
    fn add(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(
            a.bits()
                .max(b.bits())
                .checked_add(1)
                .ok_or(UnitError::Overflow {
                    operation: OPERATION,
                })?,
        )?;
        Ok(a + b)
    }
    fn sub(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(a.bits().max(b.bits()))?;
        if a < b {
            return Err(UnitError::Overflow {
                operation: OPERATION,
            });
        }
        Ok(a - b)
    }
    fn scalar(self, value: u64) -> Result<BigUint, UnitError> {
        self.admit(u64::from(u64::BITS - value.leading_zeros()))?;
        Ok(BigUint::from(value))
    }
    fn normalize(self, num: BigUint, den: BigUint) -> Result<Wide, UnitError> {
        self.admit(num.bits().max(den.bits()))?;
        if den == BigUint::from(0u8) {
            return Err(UnitError::DivisionByZero {
                operation: OPERATION,
            });
        }
        let (mut left, mut right) = (num.clone(), den.clone());
        while right != BigUint::from(0u8) {
            let remainder = &left % &right;
            left = right;
            right = remainder;
        }
        let value = Wide {
            num: num / &left,
            den: den / left,
        };
        let bits = value.num.bits().max(value.den.bits());
        if bits > self.component_bits {
            return Err(UnitError::DomainExceeded {
                operation: OPERATION,
                kind: "original coordinate component bits",
                value: i128::from(bits),
                limit: i128::from(self.component_bits),
            });
        }
        Ok(value)
    }
    pub(super) fn reference(self, param: Param) -> Result<Wide, UnitError> {
        let value = param.as_rational();
        // Explicit i64 field views guard the 63-bit reference proof against a representation change.
        let (num, den): (i64, i64) = (value.numerator(), value.denominator());
        self.normalize(
            self.scalar(num.unsigned_abs())?,
            self.scalar(den.unsigned_abs())?,
        )
    }
    pub(super) fn endpoint(self, end: bool) -> Result<Wide, UnitError> {
        self.normalize(self.scalar(u64::from(end))?, self.scalar(1)?)
    }
    pub(super) fn reverse(self, value: Wide) -> Result<Wide, UnitError> {
        self.normalize(self.sub(&value.den, &value.num)?, value.den)
    }
    pub(super) fn split(self, value: Wide, at: Param, second: bool) -> Result<Wide, UnitError> {
        let split = self.reference(at)?;
        let num = if second {
            let rest = self.sub(&split.den, &split.num)?;
            self.add(
                &self.mul(&value.num, &rest)?,
                &self.mul(&split.num, &value.den)?,
            )?
        } else {
            self.mul(&value.num, &split.num)?
        };
        self.normalize(num, self.mul(&value.den, &split.den)?)
    }
    pub(super) fn offset(self, value: Wide, from: Param, to: Param) -> Result<Wide, UnitError> {
        let low = self.reference(from)?;
        let high = self.reference(to)?;
        let extent = self.sub(
            &self.mul(&high.num, &low.den)?,
            &self.mul(&low.num, &high.den)?,
        )?;
        let shifted = self.mul(&self.mul(&low.num, &value.den)?, &high.den)?;
        let num = self.add(&self.mul(&value.num, &extent)?, &shifted)?;
        let den = self.mul(&self.mul(&value.den, &high.den)?, &low.den)?;
        self.normalize(num, den)
    }
    pub(super) fn merge_side(
        self,
        value: &Wide,
        first: u64,
        second: u64,
    ) -> Result<Ordering, UnitError> {
        let total = first.checked_add(second).ok_or(UnitError::Overflow {
            operation: OPERATION,
        })?;
        Ok(self
            .mul(&value.num, &self.scalar(total)?)?
            .cmp(&self.mul(&value.den, &self.scalar(first)?)?))
    }
    pub(super) fn merge(
        self,
        value: Wide,
        first: u64,
        second: u64,
        on_second: bool,
    ) -> Result<Wide, UnitError> {
        let total = first.checked_add(second).ok_or(UnitError::Overflow {
            operation: OPERATION,
        })?;
        let scaled = self.mul(&value.num, &self.scalar(total)?)?;
        let (num, divisor) = if on_second {
            (
                self.sub(&scaled, &self.mul(&value.den, &self.scalar(first)?)?)?,
                second,
            )
        } else {
            (scaled, first)
        };
        self.normalize(num, self.mul(&value.den, &self.scalar(divisor)?)?)
    }
}
