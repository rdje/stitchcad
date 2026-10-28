//! Original-domain checks and exact cumulative endpoint differences over admitted actual context.
use super::NotchEaseAdmission;
use crate::ontology::{EdgeAnchor, EdgeRange, EdgeRef, EntityId, Param, SeamDirection, SeamSideId};
use core::fmt;
use sc_units::{Length, UnitError};

/// Actual original distribution frame,bounds,context or concrete quantity arithmetic refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NotchEaseAmountError {
    /// Boundary does not use the actual full original distribution-side edge.
    SourceFrame {
        /// Actual full original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Required original edge.
        expected: EdgeRef,
        /// Actual supplied boundary.
        at: EdgeAnchor,
    },
    /// Interval does not use the actual full original distribution-side edge.
    IntervalSource {
        /// Actual full original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Required original edge.
        expected: EdgeRef,
        /// Actual supplied interval.
        interval: EdgeRange,
    },
    /// Boundary lies outside the closed full original side domain,not the clamped notch interval.
    Outside {
        /// Actual full original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Full original side domain.
        range: EdgeRange,
        /// Actual original-frame position.
        at: Param,
    },
    /// Stored projections fail the private common-frame/unique/distinct preconditions.
    /// The admission retains the actual sealed context and complete projections for inspection.
    ContextFrame {
        /// Actual full original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Required original edge.
        expected: EdgeRef,
        /// Actual named start/end Notch identities,without swapping them.
        notches: (EntityId, EntityId),
    },
    /// Actual arithmetic/quota cause,also the standard error source.
    Numeric {
        /// Actual full original source identity.
        source_span: EntityId,
        /// Actual distribution side.
        side: SeamSideId,
        /// Concrete numeric cause with the actual operation.
        issue: UnitError,
    },
}
impl fmt::Display for NotchEaseAmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "original-notch quantity: {self:?}")
    }
}
impl std::error::Error for NotchEaseAmountError {
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
            }
            | Self::ContextFrame {
                source_span: _,
                side: _,
                expected: _,
                notches: _,
            } => None,
        }
    }
}
impl NotchEaseAdmission<'_> {
    fn numeric(self, issue: UnitError) -> NotchEaseAmountError {
        let admission = self.context.validation().admission();
        NotchEaseAmountError::Numeric {
            source_span: admission.source().span(),
            side: admission.distribution_side(),
            issue,
        }
    }
    fn boundary(self, at: EdgeAnchor) -> Result<EdgeRange, NotchEaseAmountError> {
        let admission = self.context.validation().admission();
        let side = admission.distribution_side();
        let source_span = admission.source().span();
        let range = admission.source().side(side).range;
        if at.edge != range.edge() {
            return Err(NotchEaseAmountError::SourceFrame {
                source_span,
                side,
                expected: range.edge(),
                at,
            });
        }
        if at.param < range.from() || at.param > range.to() {
            return Err(NotchEaseAmountError::Outside {
                source_span,
                side,
                range,
                at: at.param,
            });
        }
        Ok(range)
    }
    fn reversed(self) -> bool {
        let admission = self.context.validation().admission();
        admission.distribution_side() == SeamSideId::B
            && admission.source().direction() == SeamDirection::Opposite
    }
    /// Original signed cumulative amount in canonical A traversal order.
    /// Supply a boundary in the full original distribution-side edge/domain. Ascending progress
    /// clamps outside the exact original notch interval;B/Opposite complements it before shared
    /// half-away rounding. Original wide coordinates are never narrowed. No reference choice,
    /// symbolic value,physical correspondence or current whole Design proof is supplied.
    ///
    /// # Errors
    /// Original edge and closed domain are checked before query integer allocation,even for zero
    /// ease. Retains actual source/side/position,private context refusal or concrete numeric cause.
    /// ```
    /// use sc_core::ontology::{NotchEaseAdmission,NotchEaseAmountError,EdgeAnchor};
    /// fn query(actual:NotchEaseAdmission<'_>,at:EdgeAnchor)->Result<sc_units::Length,NotchEaseAmountError>{
    ///     actual.cumulative_at(at)
    /// }
    /// ```
    pub fn cumulative_at(self, at: EdgeAnchor) -> Result<Length, NotchEaseAmountError> {
        let range = self.boundary(at)?;
        let admission = self.context.validation().admission();
        self.context
            .lower()
            .notch_cumulative_at(
                self.context.upper(),
                at.param,
                self.parent,
                self.reversed(),
                self.integer_bits,
            )
            .map_err(|issue| self.numeric(issue))?
            .ok_or(NotchEaseAmountError::ContextFrame {
                source_span: admission.source().span(),
                side: admission.distribution_side(),
                expected: range.edge(),
                notches: admission.notch_ids(),
            })
    }
    /// Difference of original rounded cumulative boundaries,using canonical A order on either side.
    /// A retained child never becomes a new source or renormalizes the original notch interval.
    /// Both full original edge/domain endpoints validate before either cumulative query.
    ///
    /// # Errors
    /// Preserves the actual interval/frame/bounds or concrete numeric cause;no partial amount.
    pub fn amount_in(self, interval: EdgeRange) -> Result<Length, NotchEaseAmountError> {
        let admission = self.context.validation().admission();
        let side = admission.distribution_side();
        let expected = admission.source().side(side).range.edge();
        if interval.edge() != expected {
            return Err(NotchEaseAmountError::IntervalSource {
                source_span: admission.source().span(),
                side,
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
        self.boundary(from)?;
        self.boundary(to)?;
        let lower = self.cumulative_at(from)?;
        let upper = self.cumulative_at(to)?;
        if self.reversed() {
            lower.checked_sub(upper)
        } else {
            upper.checked_sub(lower)
        }
        .map_err(|issue| self.numeric(issue))
    }
}
