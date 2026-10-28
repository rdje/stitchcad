//! Original-source cumulative quantization; no graph edits or physical sewing proof.
use super::{CommandSeamSplitBinding, SplitSeamSpan};
use crate::ontology::{
    EaseAmount, EaseDistribution, EdgeRange, EdgeRef, EntityId, Param, RangeProgressError,
    Rational, SeamDirection, SeamSideId, SeamSpan,
};
use core::fmt;
use sc_units::{round::div_round_half_away_from_zero, Length, UnitError};

/// Original source or arithmetic refusal from the uniform explicit allocation helper.
/// Distribution/amount variants describe helper eligibility, not an invalid sewing graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandUniformEaseError {
    /// Retained original-source portions require their separate original-root quantity query.
    RetainedPortion {
        /// Actual requested span identity.
        span: EntityId,
        /// Retained original source identity.
        source_span: EntityId,
    },
    /// Weighted allocation requires its separate cumulative distribution implementation.
    Weighted {
        /// Original correspondence identity.
        span: EntityId,
        /// Actual declared distribution side.
        side: SeamSideId,
    },
    /// Notch-bounded allocation requires the actual named landmark context.
    BetweenNotches {
        /// Original correspondence identity.
        span: EntityId,
        /// Actual declared distribution side.
        side: SeamSideId,
        /// Actual first named notch.
        start: EntityId,
        /// Actual second named notch.
        end: EntityId,
    },
    /// The original symbolic amount is retained without inventing a resolved value.
    Amount {
        /// Original correspondence identity.
        span: EntityId,
        /// Actual original formula/profile source.
        amount: EaseAmount,
    },
    /// Original range, position and actual normalization refusal.
    Progress {
        /// Original correspondence identity.
        span: EntityId,
        /// Named side whose normalization refused.
        side: SeamSideId,
        /// Concrete original cause, also the standard error source.
        issue: Box<RangeProgressError>,
    },
    /// Uniform cuts do not have equal progress in the declared endpoint correspondence.
    CutProgress {
        /// Original correspondence identity.
        span: EntityId,
        /// Normalized ascending A progress.
        a: Rational,
        /// Normalized ascending B progress, before direction adjustment.
        b: Rational,
        /// Actual original endpoint correspondence.
        direction: SeamDirection,
    },
    /// A query interval uses a different original source edge.
    IntervalSource {
        /// Original correspondence identity.
        span: EntityId,
        /// Original A edge required by this query.
        expected: EdgeRef,
        /// Unchanged requested interval.
        interval: EdgeRange,
    },
    /// Original shared quantity-arithmetic cause.
    Numeric {
        /// Original correspondence identity.
        span: EntityId,
        /// Concrete arithmetic refusal, also the standard error source.
        issue: UnitError,
    },
}

impl fmt::Display for CommandUniformEaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RetainedPortion { span, source_span } => write!(
                f,
                "span {span} uses retained ease source {source_span};direct allocation is inapplicable"
            ),
            Self::Weighted { span, side } => {
                write!(f,
                "seam {span} uses weighted ease on {side:?}; uniform allocation is inapplicable")
            }
            Self::BetweenNotches {
                span,
                side,
                start,
                end,
            } => write!(
                f,
                "seam {span} uses notch-bounded ease on {side:?} between {start} and {end}"
            ),
            Self::Amount { span, amount } => {
                write!(f,
                "seam {span} retains symbolic ease {amount:?}; explicit allocation needs a value")
            }
            Self::Progress { span, side, issue } => {
                write!(f, "seam {span} ease progress on {side:?}: {issue}")
            }
            Self::CutProgress {
                span,
                a,
                b,
                direction,
            } => {
                write!(f,
                "seam {span} uniform cuts have incompatible progress A={a}, B={b}, {direction:?}")
            }
            Self::IntervalSource {
                span,
                expected,
                interval,
            } => write!(
                f,
                "seam {span} ease query needs original A source {expected:?}, got {interval:?}"
            ),
            Self::Numeric { span, issue } => write!(f, "seam {span} ease allocation: {issue}"),
        }
    }
}

impl std::error::Error for CommandUniformEaseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Progress {
                span: _,
                side: _,
                issue,
            } => Some(issue.as_ref()),
            Self::Numeric { span: _, issue } => Some(issue),
            Self::RetainedPortion {
                span: _,
                source_span: _,
            }
            | Self::Weighted { span: _, side: _ }
            | Self::BetweenNotches {
                span: _,
                side: _,
                start: _,
                end: _,
            }
            | Self::Amount { span: _, amount: _ }
            | Self::CutProgress {
                span: _,
                a: _,
                b: _,
                direction: _,
            }
            | Self::IntervalSource {
                span: _,
                expected: _,
                interval: _,
            } => None,
        }
    }
}

/// Borrowed original uniform explicit source with conserved, A-ordered cut amounts.
/// Queries always use the original source domain and cumulative quantized parent amount.
/// This is descriptive allocation evidence, not child lineage, currentness or execution authority.
///
/// Direct construction is refused even with correctly typed source data:
/// ```compile_fail
/// use sc_core::{command::{CommandUniformEasePartition, SplitSeamSpan},
///     ontology::{Rational, SeamSpan}};
/// use sc_units::Length;
/// fn forge<'a>(request: &'a SplitSeamSpan, span: &'a SeamSpan,
///     parent: Length, progress: Rational, first: Length, second: Length)
///     -> CommandUniformEasePartition<'a> {
///     CommandUniformEasePartition { request, span, parent, progress, first, second }
/// }
/// ```
/// The original owners must outlive the view:
/// ```compile_fail
/// use sc_core::command::{CommandSeamSplitBinding, CommandUniformEasePartition};
/// fn escape<'a>(binding: &CommandSeamSplitBinding<'a>) -> CommandUniformEasePartition<'static> {
///     binding.uniform_explicit_ease().unwrap()
/// }
/// ```
#[derive(Clone, Copy)]
pub struct CommandUniformEasePartition<'a> {
    request: &'a SplitSeamSpan,
    span: &'a SeamSpan,
    parent: Length,
    progress: Rational,
    first: Length,
    second: Length,
}

impl fmt::Debug for CommandUniformEasePartition<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            request: _,
            span,
            parent: _,
            progress,
            first: _,
            second: _,
        } = self;
        f.debug_struct("CommandUniformEasePartition")
            .field("span", &span.id())
            .field("cut_progress", progress)
            .finish_non_exhaustive()
    }
}

impl<'a> CommandUniformEasePartition<'a> {
    /// Exact original request.
    #[must_use]
    pub const fn request(&self) -> &'a SplitSeamSpan {
        self.request
    }

    /// Actual original graph member, retaining physical copies, domains and source intent.
    #[must_use]
    pub const fn span(&self) -> &'a SeamSpan {
        self.span
    }

    /// Original signed explicit amount, never a rounded child total used as a new source.
    #[must_use]
    pub const fn parent_amount(&self) -> Length {
        self.parent
    }

    /// Exact original A-domain cut progress; B was checked using declared direction.
    #[must_use]
    pub const fn cut_progress(&self) -> Rational {
        self.progress
    }

    /// Amount of the first pair in A-lower/upper order.
    #[must_use]
    pub const fn first_amount(&self) -> Length {
        self.first
    }

    /// Original parent minus the first amount, conserving its signed quantum sum exactly.
    #[must_use]
    pub const fn second_amount(&self) -> Length {
        self.second
    }

    /// Quantized cumulative original allocation at a position in the original closed A domain.
    /// Stored edge parameters denote fractions of their own edge length, not curve polynomial t.
    /// # Errors
    /// Retains original bounds/normalization and shared quantity-arithmetic refusals.
    pub fn cumulative_at_a(&self, at: Param) -> Result<Length, CommandUniformEaseError> {
        let progress = self
            .span
            .definition()
            .a
            .range
            .relative_progress(at)
            .map_err(|issue| CommandUniformEaseError::Progress {
                span: self.span.id(),
                side: SeamSideId::A,
                issue: Box::new(issue),
            })?;
        quantize(self.span.id(), self.parent, progress)
    }

    /// Difference original cumulative endpoint amounts for an original-A-frame subinterval.
    /// It never repartitions rounded children. The query does not create a child or certify lineage.
    /// # Errors
    /// Checks original edge first, then lower and upper endpoint bounds/arithmetic in that order.
    pub fn amount_in_a(&self, interval: EdgeRange) -> Result<Length, CommandUniformEaseError> {
        let expected = self.span.definition().a.range.edge();
        if interval.edge() != expected {
            return Err(CommandUniformEaseError::IntervalSource {
                span: self.span.id(),
                expected,
                interval,
            });
        }
        let lower = self.cumulative_at_a(interval.from())?;
        let upper = self.cumulative_at_a(interval.to())?;
        upper
            .checked_sub(lower)
            .map_err(|issue| CommandUniformEaseError::Numeric {
                span: self.span.id(),
                issue,
            })
    }
}

impl<'a> CommandSeamSplitBinding<'a> {
    /// Allocate the actual uniform explicit source using original cumulative boundaries.
    /// Same-direction normalized cuts must agree; Opposite compares A with one minus ascending B.
    /// The two amounts follow the existing A-ordered interval pairs, including negative/zero ease.
    /// No current validation, new identity, child graph or bus effect is supplied.
    ///
    /// ```
    /// use sc_core::command::{CommandSeamSplitBinding, CommandUniformEaseError,
    ///     CommandUniformEasePartition};
    /// fn allocate<'a>(binding: &CommandSeamSplitBinding<'a>)
    ///     -> Result<CommandUniformEasePartition<'a>, CommandUniformEaseError> {
    ///     let allocation = binding.uniform_explicit_ease()?;
    ///     assert_eq!(allocation.first_amount().checked_add(allocation.second_amount()),
    ///         Ok(allocation.parent_amount()));
    ///     Ok(allocation)
    /// }
    /// ```
    /// # Errors
    /// Checks distribution eligibility, then explicit source, A/B normalization, directed cut
    /// compatibility and shared quantity arithmetic. All refusals retain actual source context.
    pub fn uniform_explicit_ease(
        &self,
    ) -> Result<CommandUniformEasePartition<'a>, CommandUniformEaseError> {
        let span = self.span();
        let source = span.definition();
        if let Some(portion) = source.ease.portion() {
            return Err(CommandUniformEaseError::RetainedPortion {
                span: span.id(),
                source_span: portion.source().span(),
            });
        }
        match &source.ease.declaration().distribution {
            EaseDistribution::Uniform => (),
            EaseDistribution::Weighted { side, regions: _ } => {
                return Err(CommandUniformEaseError::Weighted {
                    span: span.id(),
                    side: *side,
                })
            }
            EaseDistribution::BetweenNotches { side, start, end } => {
                return Err(CommandUniformEaseError::BetweenNotches {
                    span: span.id(),
                    side: *side,
                    start: *start,
                    end: *end,
                })
            }
        }
        let parent = match source.ease.declaration().amount {
            EaseAmount::Explicit(amount) => amount,
            amount @ (EaseAmount::Formula(_) | EaseAmount::Profile(_)) => {
                return Err(CommandUniformEaseError::Amount {
                    span: span.id(),
                    amount,
                })
            }
        };
        let progress = |side, range: EdgeRange| {
            range
                .relative_progress(self.request().cut(side).param)
                .map_err(|issue| CommandUniformEaseError::Progress {
                    span: span.id(),
                    side,
                    issue: Box::new(issue),
                })
        };
        let a = progress(SeamSideId::A, source.a.range)?;
        let b = progress(SeamSideId::B, source.b.range)?;
        let directed_b = match source.direction {
            SeamDirection::Same => b,
            SeamDirection::Opposite => {
                b.one_minus()
                    .map_err(|issue| CommandUniformEaseError::Numeric {
                        span: span.id(),
                        issue,
                    })?
            }
        };
        if a != directed_b {
            return Err(CommandUniformEaseError::CutProgress {
                span: span.id(),
                a,
                b,
                direction: source.direction,
            });
        }
        let first = quantize(span.id(), parent, a)?;
        let second =
            parent
                .checked_sub(first)
                .map_err(|issue| CommandUniformEaseError::Numeric {
                    span: span.id(),
                    issue,
                })?;
        Ok(CommandUniformEasePartition {
            request: self.request(),
            span,
            parent,
            progress: a,
            first,
            second,
        })
    }
}

fn quantize(
    span: EntityId,
    parent: Length,
    progress: Rational,
) -> Result<Length, CommandUniformEaseError> {
    // Both factors are i64 and progress is in [0,1], so the product fits i128. Retain the
    // original exact denominator and invoke the single shared half-away rule once per boundary.
    let numerator = i128::from(parent.as_micrometres()) * i128::from(progress.numerator());
    div_round_half_away_from_zero(numerator, i128::from(progress.denominator()))
        .and_then(Length::from_micrometres)
        .map_err(|issue| CommandUniformEaseError::Numeric { span, issue })
}
