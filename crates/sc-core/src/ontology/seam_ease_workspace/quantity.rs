//! Exact density integration in the admitted original-source workspace.
use super::WeightedEaseAdmission;
use crate::ontology::{
    EdgeAnchor, EdgeRange, EdgeRef, EntityId, Param, Rational, SeamDirection, SeamSideId,
    WeightedEaseRegion,
};
use core::fmt;
use num_bigint::BigUint;
use sc_units::{round::round_magnitude_from_half_comparison, Length, UnitError};

/// Actual original distribution-side frame,bounds or arithmetic refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightedEaseAmountError {
    /// Boundary uses a different edge from the original declared density side.
    SourceFrame {
        /// Original source identity.
        source_span: EntityId,
        /// Actual density side.
        side: SeamSideId,
        /// Original edge frame.
        expected: EdgeRef,
        /// Actual supplied boundary.
        at: EdgeAnchor,
    },
    /// Interval uses a different original density-side edge.
    IntervalSource {
        /// Original source identity.
        source_span: EntityId,
        /// Actual density side.
        side: SeamSideId,
        /// Original edge frame.
        expected: EdgeRef,
        /// Actual requested interval.
        interval: EdgeRange,
    },
    /// Boundary lies outside the closed original density-side domain.
    Outside {
        /// Original source identity.
        source_span: EntityId,
        /// Actual density side.
        side: SeamSideId,
        /// Complete original domain.
        range: EdgeRange,
        /// Actual source-frame position.
        at: Param,
    },
    /// Concrete quantity-arithmetic refusal,also the standard error source.
    Numeric {
        /// Original source identity.
        source_span: EntityId,
        /// Actual density side.
        side: SeamSideId,
        /// Original numeric cause.
        issue: UnitError,
    },
}
impl fmt::Display for WeightedEaseAmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceFrame {
                source_span,
                side,
                expected,
                at,
            } => write!(
                f,
                "ease source {source_span} {side:?} needs {expected:?},got {at:?}"
            ),
            Self::IntervalSource {
                source_span,
                side,
                expected,
                interval,
            } => write!(
                f,
                "ease source {source_span} {side:?} needs {expected:?},got {interval:?}"
            ),
            Self::Outside {
                source_span,
                side,
                range,
                at,
            } => write!(
                f,
                "ease source {source_span} {side:?} position {at} is outside {range:?}"
            ),
            Self::Numeric {
                source_span,
                side,
                issue,
            } => {
                write!(f, "ease source {source_span} {side:?} quantity: {issue}")
            }
        }
    }
}
impl std::error::Error for WeightedEaseAmountError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Numeric {
                source_span: _,
                side: _,
                issue,
            } => Some(issue),
            Self::SourceFrame {
                source_span: _,
                side: _,
                expected: _,
                at: _,
            }
            | Self::IntervalSource {
                source_span: _,
                side: _,
                expected: _,
                interval: _,
            }
            | Self::Outside {
                source_span: _,
                side: _,
                range: _,
                at: _,
            } => None,
        }
    }
}
impl WeightedEaseAdmission<'_> {
    fn numeric(self, issue: UnitError) -> WeightedEaseAmountError {
        WeightedEaseAmountError::Numeric {
            source_span: self.source.span(),
            side: self.side,
            issue,
        }
    }
    fn validate_boundary(self, at: EdgeAnchor) -> Result<EdgeRange, WeightedEaseAmountError> {
        let range = self.source.side(self.side).range;
        if at.edge != range.edge() {
            return Err(WeightedEaseAmountError::SourceFrame {
                source_span: self.source.span(),
                side: self.side,
                expected: range.edge(),
                at,
            });
        }
        if at.param < range.from() || at.param > range.to() {
            return Err(WeightedEaseAmountError::Outside {
                source_span: self.source.span(),
                side: self.side,
                range,
                at: at.param,
            });
        }
        Ok(range)
    }
    fn reverses_density(self) -> bool {
        self.side == SeamSideId::B && self.source.direction() == SeamDirection::Opposite
    }
    /// Quantized original weighted cumulative amount in canonical A traversal order.
    /// Supply a boundary in the original declared density side's edge frame. B/Opposite complements
    /// the exact density fraction before rounding. This pure intent query grants no currentness,
    /// physical correspondence or command authority. Every call uses the complete original density.
    ///
    /// # Errors
    /// Checks original edge and closed bounds before integer allocation;retains actual numeric causes.
    ///
    /// ```
    /// use sc_core::ontology::{EdgeAnchor, WeightedEaseAdmission, WeightedEaseAmountError};
    /// use sc_units::Length;
    /// fn query(actual: WeightedEaseAdmission<'_>, at: EdgeAnchor)
    ///     -> Result<Length, WeightedEaseAmountError> {
    ///     actual.cumulative_at(at)
    /// }
    /// ```
    pub fn cumulative_at(self, at: EdgeAnchor) -> Result<Length, WeightedEaseAmountError> {
        let range = self.validate_boundary(at)?;
        let (q_n, q_d) = normalized(range, at.param).map_err(|issue| self.numeric(issue))?;
        let (mut prefix, denominator) =
            cumulative(self.regions, &q_n, &q_d).map_err(|issue| self.numeric(issue))?;
        if self.reverses_density() {
            // Positive original densities and a bounded query prove prefix<=denominator. The
            // integration helper checks that invariant before this exact unsigned subtraction.
            prefix = &denominator - prefix;
        }
        let numerator = prefix * self.parent.as_micrometres().unsigned_abs();
        let quotient = &numerator / &denominator;
        let remainder = numerator % &denominator;
        let floor = u128::try_from(&quotient).map_err(|_| self.numeric(overflow()))?;
        let magnitude = round_magnitude_from_half_comparison(
            floor,
            remainder.cmp(&(&denominator - &remainder)),
        )
        .map_err(|issue| self.numeric(issue))?;
        let magnitude = i64::try_from(magnitude).map_err(|_| self.numeric(overflow()))?;
        let signed = if self.parent.as_micrometres() < 0 {
            magnitude
                .checked_neg()
                .ok_or_else(|| self.numeric(overflow()))?
        } else {
            magnitude
        };
        Length::from_micrometres(signed).map_err(|issue| self.numeric(issue))
    }
    /// Original weighted allocation for an interval on the declared density side.
    /// Differences original quantized cumulative boundaries,using canonical A order even on an
    /// Opposite B domain. A child interval never becomes a new source or normalizes its own mass.
    ///
    /// # Errors
    /// Checks original interval edge and both closed bounds before any integer allocation,then
    /// returns actual boundary/numeric refusals. This does not assert a paired physical interval.
    pub fn amount_in(self, interval: EdgeRange) -> Result<Length, WeightedEaseAmountError> {
        let expected = self.source.side(self.side).range.edge();
        if interval.edge() != expected {
            return Err(WeightedEaseAmountError::IntervalSource {
                source_span: self.source.span(),
                side: self.side,
                expected,
                interval,
            });
        }
        let from = EdgeAnchor {
            edge: expected,
            param: interval.from(),
        };
        let to = EdgeAnchor {
            edge: expected,
            param: interval.to(),
        };
        self.validate_boundary(from)?;
        self.validate_boundary(to)?;
        let lower = self.cumulative_at(from)?;
        let upper = self.cumulative_at(to)?;
        if self.reverses_density() {
            lower.checked_sub(upper)
        } else {
            upper.checked_sub(lower)
        }
        .map_err(|issue| self.numeric(issue))
    }
}

fn overflow() -> UnitError {
    UnitError::Overflow {
        operation: "WeightedEaseAdmission::cumulative_at",
    }
}
fn denominator_zero() -> UnitError {
    UnitError::DivisionByZero {
        operation: "WeightedEaseAdmission::cumulative_at",
    }
}
fn parts(at: Param) -> (u128, u128) {
    let (n, d) = reference_magnitudes(at.as_rational());
    (u128::from(n), u128::from(d))
}
fn reference_magnitudes(value: Rational) -> (u64, u64) {
    // Only nonnegative positions/positive validated densities reach this query. Explicit field
    // types make a future reference-width change fail compilation until admission is re-derived.
    let numerator: i64 = value.numerator();
    let denominator: i64 = value.denominator();
    (numerator.unsigned_abs(), denominator.unsigned_abs())
}
fn normalized(range: EdgeRange, at: Param) -> Result<(BigUint, BigUint), UnitError> {
    let (a, ad) = parts(at);
    let (f, fd) = parts(range.from());
    let (t, td) = parts(range.to());
    // All factors have <=63 bits. These products/differences fit u128;the complete ratio
    // may need189 bits. Canceling fd algebraically avoids an unnecessary fourth field factor.
    let offset = (a * fd).checked_sub(f * ad).ok_or_else(overflow)?;
    let extent = (t * fd).checked_sub(f * td).ok_or_else(overflow)?;
    if extent == 0 {
        return Err(denominator_zero());
    }
    Ok((BigUint::from(offset) * td, BigUint::from(extent) * ad))
}
fn mass(region: WeightedEaseRegion) -> Result<(BigUint, BigUint), UnitError> {
    let (f, fd) = parts(region.from);
    let (t, td) = parts(region.to);
    let (weight_n, weight_d) = reference_magnitudes(region.weight);
    let width = (t * fd).checked_sub(f * td).ok_or_else(overflow)?;
    let numerator = BigUint::from(width) * weight_n;
    let denominator = BigUint::from(td) * fd * weight_d;
    if denominator == BigUint::from(0_u8) {
        return Err(denominator_zero());
    }
    Ok((numerator, denominator))
}
fn cumulative(
    regions: &[WeightedEaseRegion],
    q_n: &BigUint,
    q_d: &BigUint,
) -> Result<(BigUint, BigUint), UnitError> {
    // Two passes,without a region/mass clone or a retained BigUint array. Common D has <=189N
    // bits;total T <=N*2^(189N),prefix P<=q_d*T. Each positive mass-term multiplication's
    // intermediate is no larger than its complete term. Admission reserves189N+ceil(log2N)+219 bits,
    // including original query normalization and the later30-bit source multiplier.
    let mut common = BigUint::from(1_u8);
    for region in regions {
        common *= mass(*region)?.1;
    }
    let mut total = BigUint::from(0_u8);
    let mut prefix = BigUint::from(0_u8);
    for region in regions {
        let (n, d) = mass(*region)?;
        let factor = &common / d;
        let contribution = n * &factor;
        total += &contribution;
        let (t, td) = parts(region.to);
        if q_n * td >= q_d * t {
            prefix += &contribution * q_d;
        } else {
            let (f, fd) = parts(region.from);
            let at = q_n * fd;
            let start = q_d * f;
            if at > start {
                let (weight_n, _) = reference_magnitudes(region.weight);
                prefix += (at - start) * weight_n * td * factor;
            }
        }
    }
    let denominator = q_d * total;
    if denominator == BigUint::from(0_u8) {
        return Err(denominator_zero());
    }
    if prefix > denominator {
        return Err(overflow());
    }
    Ok((prefix, denominator))
}
