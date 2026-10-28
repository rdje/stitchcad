//! Sealed actual original-notch interval,without amount evaluation or physical correspondence.
use super::{NotchContextLimits, NotchSourceValidation};
use crate::ontology::{
    EdgeRange, EdgeRef, EntityId, Notch, OriginalPointAdmission, OriginalPointAdmissionError,
    OriginalPointProjection, OriginalPointProjectionError, SeamSideId,
};
use core::{cmp::Ordering, fmt};
use sc_units::UnitError;
const OPERATION: &str = "NotchSourceValidation::original_context";

/// Actual original-context refusal;complete ambiguous/coincident projections retain their frames.
#[derive(Debug)]
pub enum OriginalNotchContextIssue<'a> {
    /// Combined per-point candidate budget plus one retained unique result cannot be represented.
    CandidateBoundOverflow {
        /// Actual explicit per-point candidate limit.
        candidates: usize,
    },
    /// Actual named Notch current/source/integer admission cause,before wide projection.
    Admission {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Original fixed admission refusal.
        issue: Box<OriginalPointAdmissionError>,
    },
    /// Actual inverse failure,without any partial inverse candidates.
    Projection {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Original source/held/current/operation/quota/numeric cause.
        issue: OriginalPointProjectionError,
    },
    /// Completed inverse has more than one coordinate/direction candidate;none is selected.
    Ambiguous {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Complete bounded actual candidate set and admitted frame evidence.
        projection: Box<OriginalPointProjection<'a>>,
    },
    /// A projection does not bind to the actual expected immutable source frame.
    Frame {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Actual original distribution-side edge.
        expected_source: EdgeRef,
        /// Actual projection,with its own source/creation/current/candidate evidence.
        projection: Box<OriginalPointProjection<'a>>,
    },
    /// Unique original coordinate lies outside the actual full original side domain.
    OutsideOriginal {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Actual original full side domain.
        domain: EdgeRange,
        /// Actual unique wide coordinate,without bounded view narrowing.
        projection: Box<OriginalPointProjection<'a>>,
    },
    /// Numeric bound comparison refusal retains the operand and actual cause.
    NumericBound {
        /// Actual named Notch identity.
        notch: EntityId,
        /// Actual numeric/quota cause,also the standard error source.
        cause: UnitError,
        /// Actual source-bound comparison operand.
        projection: Box<OriginalPointProjection<'a>>,
    },
    /// Two projections do not satisfy the private common-frame/uniqueness prerequisites.
    PairFrame {
        /// Actual original distribution-side edge.
        expected_source: EdgeRef,
        /// Actual named start projection.
        start: Box<OriginalPointProjection<'a>>,
        /// Actual named end projection.
        end: Box<OriginalPointProjection<'a>>,
    },
    /// Numeric pair comparison refusal retains both actual operands and concrete cause.
    NumericPair {
        /// Actual numeric/quota cause,also the standard error source.
        cause: UnitError,
        /// Actual named start projection.
        start: Box<OriginalPointProjection<'a>>,
        /// Actual named end projection.
        end: Box<OriginalPointProjection<'a>>,
    },
    /// Distinct current Notches have the same exact original coordinate,no positive interval.
    CoincidentOriginal {
        /// Actual named start projection.
        start: Box<OriginalPointProjection<'a>>,
        /// Actual named end projection.
        end: Box<OriginalPointProjection<'a>>,
    },
}
#[derive(Debug)]
struct Failure<'a> {
    source_span: EntityId,
    side: SeamSideId,
    start: EntityId,
    end: EntityId,
    issue: OriginalNotchContextIssue<'a>,
}
/// Original source/named IDs and actual context refusal,with no accepted partial context.
/// Projection evidence borrows the actual immutable ledger for the diagnostic's lifetime.
#[derive(Debug)]
pub struct OriginalNotchContextError<'a>(Box<Failure<'a>>);
impl<'a> OriginalNotchContextError<'a> {
    /// Actual retained original span identity.
    #[must_use]
    pub const fn source_span(&self) -> EntityId {
        self.0.source_span
    }
    /// Actual original distribution side.
    #[must_use]
    pub const fn distribution_side(&self) -> SeamSideId {
        self.0.side
    }
    /// Actual start/end IDs in named order.
    #[must_use]
    pub const fn notch_ids(&self) -> (EntityId, EntityId) {
        (self.0.start, self.0.end)
    }
    /// Complete actual refusal and bounded evidence.
    #[must_use]
    pub const fn issue(&self) -> &OriginalNotchContextIssue<'a> {
        &self.0.issue
    }
}
impl fmt::Display for OriginalNotchContextError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "original notch context source {} {:?} {:?}: {:?}",
            self.source_span(),
            self.distribution_side(),
            self.notch_ids(),
            self.issue()
        )
    }
}
impl std::error::Error for OriginalNotchContextError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.issue() {
            OriginalNotchContextIssue::Admission { notch: _, issue } => Some(issue.as_ref()),
            OriginalNotchContextIssue::Projection { notch: _, issue } => Some(issue),
            OriginalNotchContextIssue::NumericBound {
                notch: _,
                cause,
                projection: _,
            }
            | OriginalNotchContextIssue::NumericPair {
                cause,
                start: _,
                end: _,
            } => Some(cause),
            OriginalNotchContextIssue::CandidateBoundOverflow { candidates: _ }
            | OriginalNotchContextIssue::Ambiguous {
                notch: _,
                projection: _,
            }
            | OriginalNotchContextIssue::Frame {
                notch: _,
                expected_source: _,
                projection: _,
            }
            | OriginalNotchContextIssue::OutsideOriginal {
                notch: _,
                domain: _,
                projection: _,
            }
            | OriginalNotchContextIssue::PairFrame {
                expected_source: _,
                start: _,
                end: _,
            }
            | OriginalNotchContextIssue::CoincidentOriginal { start: _, end: _ } => None,
        }
    }
}

/// Actual original-notch interval bound to validated immutable source inputs and unique projections.
/// Named order and mathematical lower/upper order remain distinct. No amount evaluation,quantity,
/// whole Design/other graph or physical readiness follows. No dependency integer type escapes.
///
/// Private fields prevent fabrication:
/// ```compile_fail
/// use sc_core::ontology::{OriginalNotchContext, OriginalPointProjection, NotchSourceValidation};
/// fn forge<'a>(validation:NotchSourceValidation<'a>,start:OriginalPointProjection<'a>,
///     end:OriginalPointProjection<'a>)->OriginalNotchContext<'a> {
///     OriginalNotchContext { validation,start,end,start_is_lower:true,candidate_item_bound:2 }
/// }
/// ```
/// Its source borrow cannot escape:
/// ```compile_fail
/// fn escape<'a>(source:sc_core::ontology::NotchSourceValidation<'a>)
///     -> sc_core::ontology::OriginalNotchContext<'static> {
///     source.original_context().unwrap()
/// }
/// ```
pub struct OriginalNotchContext<'a> {
    validation: NotchSourceValidation<'a>,
    start: OriginalPointProjection<'a>,
    end: OriginalPointProjection<'a>,
    start_is_lower: bool,
    candidate_item_bound: usize,
}
impl fmt::Debug for OriginalNotchContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            validation,
            start,
            end,
            start_is_lower,
            candidate_item_bound,
        } = self;
        f.debug_struct("OriginalNotchContext")
            .field("source_span", &validation.admission().source().span())
            .field("side", &validation.admission().distribution_side())
            .field("notches", &validation.admission().notch_ids())
            .field("start", start)
            .field("end", end)
            .field("start_is_lower", start_is_lower)
            .field("candidate_item_bound", candidate_item_bound)
            .finish()
    }
}
impl<'a> OriginalNotchContext<'a> {
    /// Actual local structural validation and complete admitted inputs.
    #[must_use]
    pub const fn validation(&self) -> NotchSourceValidation<'a> {
        self.validation
    }
    /// Actual semantic Notch objects in original named start/end order.
    #[must_use]
    pub const fn notches(&self) -> (&'a Notch, &'a Notch) {
        self.validation.named_notches()
    }
    /// Named start's complete one-candidate projection with actual held/current/source frames.
    #[must_use]
    pub const fn start(&self) -> &OriginalPointProjection<'a> {
        &self.start
    }
    /// Named end's complete one-candidate projection with actual held/current/source frames.
    #[must_use]
    pub const fn end(&self) -> &OriginalPointProjection<'a> {
        &self.end
    }
    /// Mathematical lower original projection,without swapping named Notch identities.
    #[must_use]
    pub const fn lower(&self) -> &OriginalPointProjection<'a> {
        if self.start_is_lower {
            &self.start
        } else {
            &self.end
        }
    }
    /// Mathematical upper original projection,without swapping named Notch identities.
    #[must_use]
    pub const fn upper(&self) -> &OriginalPointProjection<'a> {
        if self.start_is_lower {
            &self.end
        } else {
            &self.start
        }
    }
    /// Checked per-point candidate budget plus one retained unique result;not an allocator/CPU cap.
    #[must_use]
    pub const fn candidate_item_bound(&self) -> usize {
        self.candidate_item_bound
    }
}
#[derive(Clone, Copy)]
struct Builder<'a> {
    validation: NotchSourceValidation<'a>,
    domain: EdgeRange,
}
impl<'a> Builder<'a> {
    fn fail(self, issue: OriginalNotchContextIssue<'a>) -> OriginalNotchContextError<'a> {
        let admission = self.validation.admission();
        let (start, end) = admission.notch_ids();
        OriginalNotchContextError(Box::new(Failure {
            source_span: admission.source().span(),
            side: admission.distribution_side(),
            start,
            end,
            issue,
        }))
    }
    fn admit(
        self,
        notch: &Notch,
    ) -> Result<OriginalPointAdmission<'a>, OriginalNotchContextError<'a>> {
        let admission = self.validation.admission();
        let anchor = notch.definition().anchor;
        admission
            .ledger()
            .admit_original_point(
                anchor.edge,
                anchor.param,
                self.domain.edge(),
                admission.limits().original_point(),
            )
            .map_err(|issue| {
                self.fail(OriginalNotchContextIssue::Admission {
                    notch: notch.id(),
                    issue: Box::new(issue),
                })
            })
    }
    fn frame(
        self,
        notch: EntityId,
        projection: OriginalPointProjection<'a>,
    ) -> OriginalNotchContextError<'a> {
        self.fail(OriginalNotchContextIssue::Frame {
            notch,
            expected_source: self.domain.edge(),
            projection: Box::new(projection),
        })
    }
    fn verify(
        self,
        notch: EntityId,
        projection: OriginalPointProjection<'a>,
    ) -> Result<OriginalPointProjection<'a>, OriginalNotchContextError<'a>> {
        let ledger = self.validation.admission().ledger();
        if !projection.frame_matches(ledger, self.domain.edge()) {
            return Err(self.frame(notch, projection));
        }
        if projection.unique().is_none() {
            return Err(self.fail(OriginalNotchContextIssue::Ambiguous {
                notch,
                projection: Box::new(projection),
            }));
        }
        let mut orders = [Ordering::Equal; 2];
        for (slot, bound) in orders
            .iter_mut()
            .zip([self.domain.from(), self.domain.to()])
        {
            match projection.compare_original_bound(ledger, self.domain.edge(), bound, OPERATION) {
                Ok(Some(order)) => *slot = order,
                Ok(None) => return Err(self.frame(notch, projection)),
                Err(cause) => {
                    return Err(self.fail(OriginalNotchContextIssue::NumericBound {
                        notch,
                        cause,
                        projection: Box::new(projection),
                    }))
                }
            }
        }
        let [lower, upper] = orders;
        if lower == Ordering::Less || upper == Ordering::Greater {
            return Err(self.fail(OriginalNotchContextIssue::OutsideOriginal {
                notch,
                domain: self.domain,
                projection: Box::new(projection),
            }));
        }
        Ok(projection)
    }
}
impl<'a> NotchSourceValidation<'a> {
    /// Bind actual named Notches to one distinct in-domain original interval without bounded views.
    ///
    /// Checks combined C+1 cardinality before projection,admits both actual points before wide work,
    /// then requires actual immutable ledger/source-creation frame and one coordinate/direction per
    /// named Notch. Compare exact originals and full source bounds without selecting ambiguity.
    ///
    /// # Errors
    /// Retains original admission/projection/numeric causes and complete ambiguous/coincident evidence.
    /// No partial context,amount evaluation,physical matching or implicit reference choice occurs.
    pub fn original_context(
        self,
    ) -> Result<OriginalNotchContext<'a>, OriginalNotchContextError<'a>> {
        let admission = self.admission();
        let builder = Builder {
            validation: self,
            domain: admission.source().side(admission.distribution_side()).range,
        };
        let limits: NotchContextLimits = admission.limits();
        let candidates = limits.original_point().candidates().get();
        let candidate_item_bound = candidates.checked_add(1).ok_or_else(|| {
            builder.fail(OriginalNotchContextIssue::CandidateBoundOverflow { candidates })
        })?;
        let (first, second) = self.named_notches();
        let first_admission = builder.admit(first)?;
        let second_admission = builder.admit(second)?;
        let project = |notch: EntityId, point: OriginalPointAdmission<'a>| {
            point.project().map_err(|issue| {
                builder.fail(OriginalNotchContextIssue::Projection { notch, issue })
            })
        };
        let start = builder.verify(first.id(), project(first.id(), first_admission)?)?;
        let end = builder.verify(second.id(), project(second.id(), second_admission)?)?;
        let ordering = match start.compare_projected(
            &end,
            admission.ledger(),
            builder.domain.edge(),
            OPERATION,
        ) {
            Ok(Some(order)) => order,
            Ok(None) => {
                return Err(builder.fail(OriginalNotchContextIssue::PairFrame {
                    expected_source: builder.domain.edge(),
                    start: Box::new(start),
                    end: Box::new(end),
                }))
            }
            Err(cause) => {
                return Err(builder.fail(OriginalNotchContextIssue::NumericPair {
                    cause,
                    start: Box::new(start),
                    end: Box::new(end),
                }))
            }
        };
        if ordering == Ordering::Equal {
            return Err(builder.fail(OriginalNotchContextIssue::CoincidentOriginal {
                start: Box::new(start),
                end: Box::new(end),
            }));
        }
        Ok(OriginalNotchContext {
            validation: self,
            start,
            end,
            start_is_lower: ordering == Ordering::Less,
            candidate_item_bound,
        })
    }
}
