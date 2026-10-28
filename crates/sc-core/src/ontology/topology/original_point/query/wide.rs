//! Canonical private unit-interval fractions with guarded integer operands/results.
use super::super::super::Param;
use core::cmp::Ordering;
use num_bigint::BigUint;
use sc_units::UnitError;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Wide {
    pub(super) num: BigUint,
    pub(super) den: BigUint,
}
#[derive(Clone, Copy)]
pub(super) struct Workspace {
    pub(super) component_bits: u64,
    pub(super) integer_bits: u64,
    pub(super) operation: &'static str,
}
impl Workspace {
    fn admit(self, bits: u64) -> Result<(), UnitError> {
        if bits > self.integer_bits {
            return Err(UnitError::DomainExceeded {
                operation: self.operation,
                kind: "integer workspace bits",
                value: i128::from(bits),
                limit: i128::from(self.integer_bits),
            });
        }
        Ok(())
    }
    fn mul(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(a.bits().checked_add(b.bits()).ok_or(UnitError::Overflow {
            operation: self.operation,
        })?)?;
        Ok(a * b)
    }
    fn compare_parts(
        self,
        left_num: &BigUint,
        left_den: &BigUint,
        right_num: &BigUint,
        right_den: &BigUint,
    ) -> Result<Ordering, UnitError> {
        self.admit(
            left_num
                .bits()
                .max(left_den.bits())
                .max(right_num.bits())
                .max(right_den.bits()),
        )?;
        if left_den == &BigUint::from(0u8) || right_den == &BigUint::from(0u8) {
            return Err(UnitError::DivisionByZero {
                operation: self.operation,
            });
        }
        let left = self.mul(left_num, right_den)?;
        let right = self.mul(right_num, left_den)?;
        Ok(left.cmp(&right))
    }
    pub(super) fn compare(self, left: &Wide, right: &Wide) -> Result<Ordering, UnitError> {
        self.compare_parts(&left.num, &left.den, &right.num, &right.den)
    }
    pub(super) fn compare_reference(
        self,
        value: &Wide,
        bound: Param,
    ) -> Result<Ordering, UnitError> {
        let reference = bound.as_rational();
        let (num, den): (i64, i64) = (reference.numerator(), reference.denominator());
        // Param already supplies a canonical positive-denominator unit fraction. Avoid normalize
        // here:normalization itself uses this exact comparison to enforce its unit-domain invariant.
        let reference = Wide {
            num: self.scalar(num.unsigned_abs())?,
            den: self.scalar(den.unsigned_abs())?,
        };
        self.compare(value, &reference)
    }
    fn add(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(
            a.bits()
                .max(b.bits())
                .checked_add(1)
                .ok_or(UnitError::Overflow {
                    operation: self.operation,
                })?,
        )?;
        Ok(a + b)
    }
    fn sub(self, a: &BigUint, b: &BigUint) -> Result<BigUint, UnitError> {
        self.admit(a.bits().max(b.bits()))?;
        if a < b {
            return Err(UnitError::Overflow {
                operation: self.operation,
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
                operation: self.operation,
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
                operation: self.operation,
                kind: "original coordinate component bits",
                value: i128::from(bits),
                limit: i128::from(self.component_bits),
            });
        }
        let ordering = self.compare_reference(&value, Param::END)?;
        if ordering == Ordering::Greater {
            return Err(UnitError::DomainExceeded {
                operation: self.operation,
                kind: "original coordinate comparison with unit end",
                value: 1,
                limit: 0,
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
            operation: self.operation,
        })?;
        self.compare_parts(
            &value.num,
            &value.den,
            &self.scalar(first)?,
            &self.scalar(total)?,
        )
    }
    pub(super) fn merge(
        self,
        value: Wide,
        first: u64,
        second: u64,
        on_second: bool,
    ) -> Result<Wide, UnitError> {
        let total = first.checked_add(second).ok_or(UnitError::Overflow {
            operation: self.operation,
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

#[cfg(test)]
mod contracts {
    use super::{BigUint, Wide, Workspace};
    #[test]
    fn normalization_checks_actual_unit_order_and_zero_denominator() {
        let workspace = Workspace {
            component_bits: 63,
            integer_bits: 254,
            operation: "Workspace::normalize",
        };
        let valid = workspace
            .normalize(BigUint::from(1u8), BigUint::from(2u8))
            .expect("unit fraction");
        assert_eq!(
            workspace
                .compare_reference(&valid, crate::ontology::Param::END)
                .expect("actual comparison"),
            core::cmp::Ordering::Less
        );
        assert_eq!(
            workspace.normalize(BigUint::from(2u8), BigUint::from(1u8)),
            Err(sc_units::UnitError::DomainExceeded {
                operation: "Workspace::normalize",
                kind: "original coordinate comparison with unit end",
                value: 1,
                limit: 0,
            })
        );
        let invalid = Wide {
            num: BigUint::from(1u8),
            den: BigUint::from(0u8),
        };
        assert_eq!(
            workspace.compare(&valid, &invalid),
            Err(sc_units::UnitError::DivisionByZero {
                operation: "Workspace::normalize",
            })
        );
        assert_eq!(
            workspace.normalize(BigUint::from(1u8), BigUint::from(0u8)),
            Err(sc_units::UnitError::DivisionByZero {
                operation: "Workspace::normalize"
            })
        );
    }
}
