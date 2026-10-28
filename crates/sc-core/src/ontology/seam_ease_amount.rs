//! Uniform explicit quantities from original cumulative boundaries,without graph or physical effects.
use super::{
    EaseAmount, EaseDistribution, EdgeRange, EdgeRef, EntityId, Param, RangeProgressError,
    Rational, SeamDirection, SeamEasePortion, SeamEaseSource, SeamSideId,
};
use core::fmt;
use sc_units::{round::div_round_half_away_from_zero, Length, UnitError};

/// Actual original source eligibility,normalization,correspondence or quantity refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeamEaseAmountError {
    /// Weighted intent requires its cumulative distribution implementation.
    Weighted {
        /// Original source identity.
        source_span: EntityId,
        /// Actual declared side.
        side: SeamSideId,
    },
    /// Notch-bounded intent requires actual landmark/allocation context.
    BetweenNotches {
        /// Original source identity.
        source_span: EntityId,
        /// Actual declared side.
        side: SeamSideId,
        /// Actual first notch identity.
        start: EntityId,
        /// Actual second notch identity.
        end: EntityId,
    },
    /// Actual original symbolic amount,without a fabricated resolved value.
    Amount {
        /// Original source identity.
        source_span: EntityId,
        /// Actual formula/profile reference.
        amount: EaseAmount,
    },
    /// Original source interval/position and concrete normalization refusal.
    Progress {
        /// Original source identity.
        source_span: EntityId,
        /// Original named side.
        side: SeamSideId,
        /// Actual range/progress cause,also the standard error source.
        issue: Box<RangeProgressError>,
    },
    /// Portion endpoints do not correspond under the original uniform declaration.
    Correspondence {
        /// Original source identity.
        source_span: EntityId,
        /// Actual original-frame A endpoint.
        at_a: Param,
        /// Actual corresponding original-frame B endpoint.
        at_b: Param,
        /// Normalized ascending original A progress.
        a: Rational,
        /// Normalized ascending original B progress,before direction adjustment.
        b: Rational,
        /// Actual original endpoint correspondence.
        direction: SeamDirection,
    },
    /// Query interval names a different original A source edge.
    IntervalSource {
        /// Original source identity.
        source_span: EntityId,
        /// Original A source edge.
        expected: EdgeRef,
        /// Actual requested interval.
        interval: EdgeRange,
    },
    /// Actual shared quantity-arithmetic cause.
    Numeric {
        /// Original source identity.
        source_span: EntityId,
        /// Concrete arithmetic refusal,also the standard error source.
        issue: UnitError,
    },
}
impl fmt::Display for SeamEaseAmountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Weighted { source_span, side } => {
                write!(f, "ease source {source_span} is weighted on {side:?}")
            }
            Self::BetweenNotches { source_span, side, start, end } => write!(
                f,
                "ease source {source_span} is notch-bounded on {side:?} between {start} and {end}"
            ),
            Self::Amount { source_span, amount } => write!(
                f,
                "ease source {source_span} retains symbolic amount {amount:?}"
            ),
            Self::Progress { source_span, side, issue } => {
                write!(f, "ease source {source_span} {side:?} progress: {issue}")
            }
            Self::Correspondence { source_span, at_a, at_b, a, b, direction } => write!(
                f,
                "ease source {source_span} endpoints A={at_a} ({a}), B={at_b} ({b}) do not correspond under {direction:?}"
            ),
            Self::IntervalSource { source_span, expected, interval } => write!(
                f,
                "ease source {source_span} needs A frame {expected:?}, got {interval:?}"
            ),
            Self::Numeric { source_span, issue } => {
                write!(f, "ease source {source_span} amount: {issue}")
            }
        }
    }
}
impl std::error::Error for SeamEaseAmountError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Progress {
                source_span: _,
                side: _,
                issue,
            } => Some(issue.as_ref()),
            Self::Numeric {
                source_span: _,
                issue,
            } => Some(issue),
            Self::Weighted {
                source_span: _,
                side: _,
            }
            | Self::BetweenNotches {
                source_span: _,
                side: _,
                start: _,
                end: _,
            }
            | Self::Amount {
                source_span: _,
                amount: _,
            }
            | Self::Correspondence {
                source_span: _,
                at_a: _,
                at_b: _,
                a: _,
                b: _,
                direction: _,
            }
            | Self::IntervalSource {
                source_span: _,
                expected: _,
                interval: _,
            } => None,
        }
    }
}
impl SeamEaseSource {
    fn uniform_explicit_total(&self) -> Result<Length, SeamEaseAmountError> {
        match &self.declaration().distribution {
            EaseDistribution::Uniform => (),
            EaseDistribution::Weighted { side, regions: _ } => {
                return Err(SeamEaseAmountError::Weighted {
                    source_span: self.span(),
                    side: *side,
                })
            }
            EaseDistribution::BetweenNotches { side, start, end } => {
                return Err(SeamEaseAmountError::BetweenNotches {
                    source_span: self.span(),
                    side: *side,
                    start: *start,
                    end: *end,
                })
            }
        }
        match self.declaration().amount {
            EaseAmount::Explicit(amount) => Ok(amount),
            amount @ (EaseAmount::Formula(_) | EaseAmount::Profile(_)) => {
                Err(SeamEaseAmountError::Amount {
                    source_span: self.span(),
                    amount,
                })
            }
        }
    }
    fn progress(&self, side: SeamSideId, at: Param) -> Result<Rational, SeamEaseAmountError> {
        self.side(side)
            .range
            .relative_progress(at)
            .map_err(|issue| SeamEaseAmountError::Progress {
                source_span: self.span(),
                side,
                issue: Box::new(issue),
            })
    }
    fn quantize(&self, total: Length, progress: Rational) -> Result<Length, SeamEaseAmountError> {
        quantize_ease_boundary(total, progress).map_err(|issue| SeamEaseAmountError::Numeric {
            source_span: self.span(),
            issue,
        })
    }
    /// Quantized cumulative uniform explicit amount in the original closed A domain.
    /// The original signed source is used once per boundary,with the shared rounding rule.
    /// # Errors
    /// Checks distribution then amount eligibility,followed by original bounds/numeric causes.
    pub fn uniform_explicit_cumulative_at_a(
        &self,
        at: Param,
    ) -> Result<Length, SeamEaseAmountError> {
        let total = self.uniform_explicit_total()?;
        self.quantize(total, self.progress(SeamSideId::A, at)?)
    }
    /// Difference original cumulative amounts for an original-A-frame subinterval.
    /// This query does not create a portion or assert paired physical correspondence.
    /// # Errors
    /// Checks original edge first,then source eligibility and lower/upper original bounds.
    pub fn uniform_explicit_amount_in_a(
        &self,
        interval: EdgeRange,
    ) -> Result<Length, SeamEaseAmountError> {
        let expected = self.side(SeamSideId::A).range.edge();
        if interval.edge() != expected {
            return Err(SeamEaseAmountError::IntervalSource {
                source_span: self.span(),
                expected,
                interval,
            });
        }
        let total = self.uniform_explicit_total()?;
        let lower = self.quantize(total, self.progress(SeamSideId::A, interval.from())?)?;
        let upper = self.quantize(total, self.progress(SeamSideId::A, interval.to())?)?;
        upper
            .checked_sub(lower)
            .map_err(|issue| SeamEaseAmountError::Numeric {
                source_span: self.span(),
                issue,
            })
    }
}
impl SeamEasePortion {
    /// Uniform explicit allocation from original cumulative endpoint differences.
    /// Both directed endpoints must correspond in the original full A/B domains. Rounded child
    /// totals never become new sources;negative and zero amounts preserve the same rule.
    /// # Errors
    /// Checks source eligibility,complete A then B normalization,lower-A then upper-A directed
    /// correspondence,and shared quantity arithmetic. No currentness or physical proof follows.
    ///
    /// ```
    /// use sc_core::ontology::{SeamEaseAmountError, SeamEasePortion};
    /// use sc_units::Length;
    /// fn amount(actual: &SeamEasePortion) -> Result<Length, SeamEaseAmountError> {
    ///     actual.uniform_explicit_amount()
    /// }
    /// ```
    pub fn uniform_explicit_amount(&self) -> Result<Length, SeamEaseAmountError> {
        let source = self.source();
        let total = source.uniform_explicit_total()?;
        let a = self.side(SeamSideId::A).range;
        let b = self.side(SeamSideId::B).range;
        let a_lower = source.progress(SeamSideId::A, a.from())?;
        let a_upper = source.progress(SeamSideId::A, a.to())?;
        let b_lower = source.progress(SeamSideId::B, b.from())?;
        let b_upper = source.progress(SeamSideId::B, b.to())?;
        let pairs = match source.direction() {
            SeamDirection::Same => [
                (a.from(), b.from(), a_lower, b_lower),
                (a.to(), b.to(), a_upper, b_upper),
            ],
            SeamDirection::Opposite => [
                (a.from(), b.to(), a_lower, b_upper),
                (a.to(), b.from(), a_upper, b_lower),
            ],
        };
        for (at_a, at_b, a, b) in pairs {
            let directed_b = match source.direction() {
                SeamDirection::Same => b,
                SeamDirection::Opposite => {
                    b.one_minus()
                        .map_err(|issue| SeamEaseAmountError::Numeric {
                            source_span: source.span(),
                            issue,
                        })?
                }
            };
            if a != directed_b {
                return Err(SeamEaseAmountError::Correspondence {
                    source_span: source.span(),
                    at_a,
                    at_b,
                    a,
                    b,
                    direction: source.direction(),
                });
            }
        }
        let lower = source.quantize(total, a_lower)?;
        let upper = source.quantize(total, a_upper)?;
        upper
            .checked_sub(lower)
            .map_err(|issue| SeamEaseAmountError::Numeric {
                source_span: source.span(),
                issue,
            })
    }
}

pub(crate) fn quantize_ease_boundary(
    parent: Length,
    progress: Rational,
) -> Result<Length, UnitError> {
    // Each factor is i64,so the exact product fits i128. Normalized progress is in [0,1].
    let numerator = i128::from(parent.as_micrometres()) * i128::from(progress.numerator());
    div_round_half_away_from_zero(numerator, i128::from(progress.denominator()))
        .and_then(Length::from_micrometres)
}
