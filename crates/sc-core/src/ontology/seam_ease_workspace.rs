//! Explicit borrowed admission for original weighted quantities,before integer workspace allocation.
use super::{
    EaseAmount, EaseDistribution, EntityId, SeamEaseSource, SeamSideId, WeightedEaseRegion,
};
use core::{fmt, num::NonZeroU64, num::NonZeroUsize};
use sc_units::{Length, MAX_LENGTH_UM};

mod quantity;
pub use quantity::WeightedEaseAmountError;

/// Explicit positive limits for one original weighted-source quantity workspace.
/// No default is supplied. These limits bound region count and conservative integer operand/result
/// width,not allocator overhead,CPU time,current Design authority or physical correspondence.
///
/// ```compile_fail
/// use sc_core::ontology::WeightedEaseLimits;
/// let limits: WeightedEaseLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeightedEaseLimits {
    regions: NonZeroUsize,
    integer_bits: NonZeroU64,
}
impl WeightedEaseLimits {
    /// Choose both limits explicitly;the positive types exclude a zero budget.
    #[must_use]
    pub const fn new(regions: NonZeroUsize, integer_bits: NonZeroU64) -> Self {
        Self {
            regions,
            integer_bits,
        }
    }
    /// Maximum number of original density regions,including regions outside a queried prefix.
    #[must_use]
    pub const fn regions(self) -> NonZeroUsize {
        self.regions
    }
    /// Maximum conservative integer width for the complete original-domain query.
    #[must_use]
    pub const fn integer_bits(self) -> NonZeroU64 {
        self.integer_bits
    }
}

/// Original source eligibility or a caller-selected workspace limit refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightedEaseWorkspaceError {
    /// Actual source is uniform,rather than a weighted declaration.
    Uniform {
        /// Original source identity.
        source_span: EntityId,
    },
    /// Actual notch-bounded source needs its separate original-frame context.
    BetweenNotches {
        /// Original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Actual first named notch.
        start: EntityId,
        /// Actual second named notch.
        end: EntityId,
    },
    /// Actual weighted source retains an unresolved formula/profile amount.
    Amount {
        /// Original source identity.
        source_span: EntityId,
        /// Actual unresolved declaration,without a fabricated value.
        amount: EaseAmount,
    },
    /// Complete original density population exceeds the selected count limit.
    Regions {
        /// Original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Complete original region count.
        required: usize,
        /// Actual caller-selected limit.
        limit: usize,
    },
    /// Conservative integer reservation exceeds the selected width limit.
    IntegerBits {
        /// Original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Complete original region count.
        regions: usize,
        /// Derived conservative reservation,not a minimal necessary width.
        required: u64,
        /// Actual caller-selected limit.
        limit: u64,
    },
    /// Region-count-derived bound cannot be represented;no integer workspace was allocated.
    BoundOverflow {
        /// Original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Complete original region count.
        regions: usize,
    },
}
impl fmt::Display for WeightedEaseWorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uniform { source_span } => write!(f, "ease source {source_span} is uniform"),
            Self::BetweenNotches { source_span, side, start, end } => write!(
                f,"ease source {source_span} is notch-bounded on {side:?} between {start} and {end}"
            ),
            Self::Amount { source_span, amount } => {
                write!(f,"ease source {source_span} retains symbolic amount {amount:?}")
            }
            Self::Regions { source_span, side, required, limit } => write!(
                f,"ease source {source_span} {side:?} needs {required} regions;limit {limit}"
            ),
            Self::IntegerBits { source_span, side, regions, required, limit } => write!(
                f,"ease source {source_span} {side:?} with {regions} regions reserves {required} integer bits;limit {limit}"
            ),
            Self::BoundOverflow { source_span, side, regions } => write!(
                f,"ease source {source_span} {side:?} workspace bound overflows for {regions} regions"
            ),
        }
    }
}
impl std::error::Error for WeightedEaseWorkspaceError {}

/// Sealed admission borrowing the actual weighted explicit original source and complete density.
/// Admission checks distribution,amount,region count,then integer reservation in that order.
/// Success performs fixed-width arithmetic and borrows only;it neither clones regions nor allocates
/// arbitrary-precision integers. Subsequent quantity computation remains a separate operation.
/// This mathematical source snapshot is not fresh Design or geometry validation.
///
/// Fields cannot be used to forge admission:
/// ```compile_fail
/// use sc_core::ontology::{SeamEaseSource, SeamSideId, WeightedEaseAdmission, WeightedEaseLimits,
///     WeightedEaseRegion};
/// use sc_units::Length;
/// fn forge<'a>(source: &'a SeamEaseSource, regions: &'a [WeightedEaseRegion],
///     limits: WeightedEaseLimits) -> WeightedEaseAdmission<'a> {
///     WeightedEaseAdmission { source, side: SeamSideId::A, regions, parent: Length::ZERO,
///         integer_bits: 1, limits }
/// }
/// ```
/// Its source borrow cannot escape:
/// ```compile_fail
/// use sc_core::ontology::{SeamEaseSource, WeightedEaseAdmission, WeightedEaseLimits};
/// fn escape(source: &SeamEaseSource, limits: WeightedEaseLimits) -> WeightedEaseAdmission<'static> {
///     source.admit_weighted_explicit(limits).unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct WeightedEaseAdmission<'a> {
    source: &'a SeamEaseSource,
    side: SeamSideId,
    regions: &'a [WeightedEaseRegion],
    parent: Length,
    integer_bits: u64,
    limits: WeightedEaseLimits,
}
impl fmt::Debug for WeightedEaseAdmission<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Exhaustive fields plus Copy prevent a later owning population from hiding here.
        let Self {
            source,
            side,
            regions,
            parent,
            integer_bits,
            limits,
        } = self;
        f.debug_struct("WeightedEaseAdmission")
            .field("source_span", &source.span())
            .field("side", side)
            .field("regions", &regions.len())
            .field("parent", parent)
            .field("integer_bits", integer_bits)
            .field("limits", limits)
            .finish()
    }
}
impl<'a> WeightedEaseAdmission<'a> {
    /// Actual original source,not a rounded child declaration.
    #[must_use]
    pub const fn source(self) -> &'a SeamEaseSource {
        self.source
    }
    /// Actual side on which the original normalized density is declared.
    #[must_use]
    pub const fn distribution_side(self) -> SeamSideId {
        self.side
    }
    /// Complete original region count;no prefix or child truncates the denominator population.
    #[must_use]
    pub const fn region_count(self) -> usize {
        self.regions.len()
    }
    /// Actual original signed literal;this is not an inherited target's allocation.
    #[must_use]
    pub const fn parent_amount(self) -> Length {
        self.parent
    }
    /// Conservative reserved integer width;admission may refuse even when specific values are small.
    #[must_use]
    pub const fn integer_bit_bound(self) -> u64 {
        self.integer_bits
    }
    /// Actual caller-selected limits retained by value.
    #[must_use]
    pub const fn limits(self) -> WeightedEaseLimits {
        self.limits
    }
}
impl SeamEaseSource {
    /// Admit an explicit original weighted source before allocating an integer query workspace.
    ///
    /// # Errors
    /// Returns actual nonweighted/symbolic intent first,then complete population/bit-limit refusals.
    /// No default budget,no truncated density and no arbitrary-precision computation is used.
    pub fn admit_weighted_explicit(
        &self,
        limits: WeightedEaseLimits,
    ) -> Result<WeightedEaseAdmission<'_>, WeightedEaseWorkspaceError> {
        let source_span = self.span();
        let (side, regions) = match &self.declaration().distribution {
            EaseDistribution::Uniform => {
                return Err(WeightedEaseWorkspaceError::Uniform { source_span })
            }
            EaseDistribution::BetweenNotches { side, start, end } => {
                return Err(WeightedEaseWorkspaceError::BetweenNotches {
                    source_span,
                    side: *side,
                    start: *start,
                    end: *end,
                });
            }
            EaseDistribution::Weighted { side, regions } => (*side, regions.as_slice()),
        };
        let parent = match self.declaration().amount {
            EaseAmount::Explicit(value) => value,
            amount @ (EaseAmount::Formula(_) | EaseAmount::Profile(_)) => {
                return Err(WeightedEaseWorkspaceError::Amount {
                    source_span,
                    amount,
                });
            }
        };
        if regions.len() > limits.regions.get() {
            return Err(WeightedEaseWorkspaceError::Regions {
                source_span,
                side,
                required: regions.len(),
                limit: limits.regions.get(),
            });
        }
        let integer_bits = weighted_workspace_bit_bound(regions.len()).ok_or(
            WeightedEaseWorkspaceError::BoundOverflow {
                source_span,
                side,
                regions: regions.len(),
            },
        )?;
        if integer_bits > limits.integer_bits.get() {
            return Err(WeightedEaseWorkspaceError::IntegerBits {
                source_span,
                side,
                regions: regions.len(),
                required: integer_bits,
                limit: limits.integer_bits.get(),
            });
        }
        Ok(WeightedEaseAdmission {
            source: self,
            side,
            regions,
            parent,
            integer_bits,
            limits,
        })
    }
}

fn weighted_workspace_bit_bound(regions: usize) -> Option<u64> {
    let regions = u64::try_from(regions).ok()?;
    let log_regions = u64::from(u64::BITS - regions.checked_sub(1)?.leading_zeros());
    let reference_bits = u64::from(i64::BITS - 1);
    let region_bits = reference_bits.checked_mul(3)?;
    let query_bits = region_bits;
    let max_length: i64 = MAX_LENGTH_UM;
    let source_bits = u64::from(i64::BITS - max_length.leading_zeros());
    // Common D<2^(189N),total T<N*2^(189N),prefix P<=Q*T. Original-domain Q needs
    // <=189 bits;source magnitude needs30. This reserves a conservative construction,not the
    // minimal width for the actual density. No BigUint allocation occurs to derive admission.
    region_bits
        .checked_mul(regions)?
        .checked_add(log_regions)?
        .checked_add(query_bits)?
        .checked_add(source_bits)
}

#[cfg(test)]
mod tests {
    use super::weighted_workspace_bit_bound as bound;

    #[test]
    fn conservative_bound_uses_checked_population_arithmetic() {
        assert_eq!(bound(1), Some(408));
        assert_eq!(bound(2), Some(598));
        assert_eq!(bound(3), Some(788));
        assert_eq!(bound(4), Some(977));
        assert_eq!(bound(0), None);
        if usize::BITS >= 64 {
            assert_eq!(bound(usize::MAX), None);
        }
    }
}
