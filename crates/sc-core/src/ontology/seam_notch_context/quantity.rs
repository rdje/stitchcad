//! Separate arithmetic admission over an actual sealed original-notch interval.
use super::OriginalNotchContext;
use crate::ontology::{EaseAmount, EntityId, Param, SeamSideId};
use core::{fmt, num::NonZeroU64};
use sc_units::{Length, MAX_LENGTH_UM};

/// Explicit positive integer operand/result limit for original-notch quantities;no default.
/// Context dependency/candidate admission remains separate. This is not an allocator or CPU cap.
/// ```compile_fail
/// let limits:sc_core::ontology::NotchEaseLimits=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotchEaseLimits {
    integer_bits: NonZeroU64,
}
impl NotchEaseLimits {
    /// Choose the quantity integer allowance explicitly.
    #[must_use]
    pub const fn new(integer_bits: NonZeroU64) -> Self {
        Self { integer_bits }
    }
    /// Actual caller-selected integer width limit.
    #[must_use]
    pub const fn integer_bits(self) -> NonZeroU64 {
        self.integer_bits
    }
}

/// Actual source amount or separate quantity-reservation refusal;no query integers allocated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchEaseWorkspaceError {
    /// Actual full original source still has a symbolic amount;no value is substituted.
    Amount {
        /// Original source span.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Actual formula/profile declaration.
        amount: EaseAmount,
    },
    /// Conservative quantity width exceeds the independently selected allowance.
    IntegerBits {
        /// Original source span.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Largest admitted original coordinate component bound.
        component_bits: u64,
        /// Conservative quantity reservation,not the minimal width for these values.
        required: u64,
        /// Actual caller-selected quantity allowance.
        limit: u64,
    },
    /// Bound arithmetic cannot be represented;no query workspace was allocated.
    BoundOverflow {
        /// Original source span.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Largest admitted original coordinate component bound.
        component_bits: u64,
    },
}
impl fmt::Display for NotchEaseWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Amount { source_span, side, amount } => write!(f,
                "notch ease source {source_span} {side:?} retains symbolic amount {amount:?}"),
            Self::IntegerBits { source_span, side, component_bits, required, limit } => write!(f,
                "notch ease source {source_span} {side:?} with {component_bits}-bit originals reserves {required} integer bits;limit {limit}"),
            Self::BoundOverflow { source_span, side, component_bits } => write!(f,
                "notch ease source {source_span} {side:?} quantity bound overflows for {component_bits}-bit originals"),
        }
    }
}
impl std::error::Error for NotchEaseWorkspaceError {}

/// Sealed quantity admission borrowing the actual original-notch context and signed source literal.
/// Uses only fixed-width checked bound arithmetic;does not copy projections or allocate query integers.
/// The earlier context's inverse/comparison allowance does not license quantity arithmetic.
///
/// Private fields prevent fabrication:
/// ```compile_fail
/// use sc_core::ontology::{NotchEaseAdmission,NotchEaseLimits,OriginalNotchContext};
/// fn forge<'a>(context:&'a OriginalNotchContext<'a>,limits:NotchEaseLimits)->NotchEaseAdmission<'a>{
///     NotchEaseAdmission {context,parent:sc_units::Length::ZERO,integer_bits:1,limits}
/// }
/// ```
/// Its context borrow cannot escape:
/// ```compile_fail
/// use sc_core::ontology::{NotchEaseAdmission,NotchEaseLimits,OriginalNotchContext};
/// fn escape<'a>(context:&'a OriginalNotchContext<'a>,limits:NotchEaseLimits)
///     ->NotchEaseAdmission<'static>{context.admit_explicit(limits).unwrap()}
/// ```
#[derive(Clone, Copy)]
pub struct NotchEaseAdmission<'a> {
    context: &'a OriginalNotchContext<'a>,
    parent: Length,
    integer_bits: u64,
    limits: NotchEaseLimits,
}
impl fmt::Debug for NotchEaseAdmission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            context,
            parent,
            integer_bits,
            limits,
        } = self;
        let admission = context.validation().admission();
        f.debug_struct("NotchEaseAdmission")
            .field("source_span", &admission.source().span())
            .field("side", &admission.distribution_side())
            .field("notches", &admission.notch_ids())
            .field("parent", parent)
            .field("integer_bits", integer_bits)
            .field("limits", limits)
            .finish()
    }
}
impl<'a> NotchEaseAdmission<'a> {
    /// Actual sealed context and complete source/Notch/projection evidence.
    #[must_use]
    pub const fn context(self) -> &'a OriginalNotchContext<'a> {
        self.context
    }
    /// Full original signed literal,not a rounded child amount.
    #[must_use]
    pub const fn parent_amount(self) -> Length {
        self.parent
    }
    /// Separate conservative quantity integer reservation.
    #[must_use]
    pub const fn integer_bit_bound(self) -> u64 {
        self.integer_bits
    }
    /// Actual selected allowance retained by value.
    #[must_use]
    pub const fn limits(self) -> NotchEaseLimits {
        self.limits
    }
}
impl OriginalNotchContext<'_> {
    /// Admit explicit original-notch quantities without allocating query integers.
    /// Check the actual amount first,then independently reserve2W+63+b integer bits,whereW is
    /// the largest admitted original coordinate component bound andb the maximum Length magnitude
    /// width. This covers clamped progress construction,orientation and source multiplication.
    /// Quantity methods remain a subsequent implementation step.
    ///
    /// # Errors
    /// Preserves actual symbolic amount first,then checked bound-overflow or selected-limit refusal.
    pub fn admit_explicit(
        &self,
        limits: NotchEaseLimits,
    ) -> Result<NotchEaseAdmission<'_>, NotchEaseWorkspaceError> {
        let admission = self.validation().admission();
        let source_span = admission.source().span();
        let side = admission.distribution_side();
        let parent = match admission.source().declaration().amount {
            EaseAmount::Explicit(value) => value,
            amount @ (EaseAmount::Formula(_) | EaseAmount::Profile(_)) => {
                return Err(NotchEaseWorkspaceError::Amount {
                    source_span,
                    side,
                    amount,
                });
            }
        };
        let component_bits = self
            .start()
            .admission()
            .component_bit_bound()
            .max(self.end().admission().component_bit_bound());
        let integer_bits =
            quantity_bit_bound(component_bits).ok_or(NotchEaseWorkspaceError::BoundOverflow {
                source_span,
                side,
                component_bits,
            })?;
        if integer_bits > limits.integer_bits.get() {
            return Err(NotchEaseWorkspaceError::IntegerBits {
                source_span,
                side,
                component_bits,
                required: integer_bits,
                limit: limits.integer_bits.get(),
            });
        }
        Ok(NotchEaseAdmission {
            context: self,
            parent,
            integer_bits,
            limits,
        })
    }
}

fn quantity_bit_bound(component_bits: u64) -> Option<u64> {
    let reference = Param::END.as_rational();
    let _: (i64, i64) = (reference.numerator(), reference.denominator());
    let reference_bits = u64::from(i64::BITS - 1);
    let max_length: i64 = MAX_LENGTH_UM;
    let source_bits = u64::from(i64::BITS - max_length.leading_zeros());
    // With L=ln/ld,U=un/ud and query xn/xd (<=63-bit fields),interior CDF is
    // ((xn*ld-ln*xd)*ud)/((un*ld-ln*ud)*xd). Each field needs <=2W+63 bits.
    // Clamping/complementing cannot increase this;source multiplication adds at mostb bits.
    component_bits
        .checked_mul(2)?
        .checked_add(reference_bits)?
        .checked_add(source_bits)
}

#[cfg(test)]
mod tests {
    use super::quantity_bit_bound;
    #[test]
    fn original_notch_quantity_bound_matches_independent_checked_rows() {
        for row in include_str!("../../../tests/fixtures/notch_quantity_workspace.tsv")
            .lines()
            .filter(|row| !row.starts_with('#'))
        {
            let (component, result) = row.split_once('\t').expect("two frozen fields");
            let component = component.parse().expect("component count");
            let expected = if result == "Overflow" {
                None
            } else {
                Some(result.parse().expect("integer count"))
            };
            assert_eq!(quantity_bit_bound(component), expected, "{row}");
        }
    }
}
