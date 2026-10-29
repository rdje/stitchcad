//! Bounded positive-interval evidence,without resolving either endpoint's choice tree.
use super::{
    known_source, repair, step_into, Direction, EdgeRange, IdentityLedger, OrphaningEdit,
    RangeIssue, RangePortion, ResolvedRange,
};
use crate::ontology::{EntityId, PointJournalAdmissionIssue, UniquePointLimits};
use core::{fmt, num::NonZeroUsize};
use sc_units::UnitError;

/// Explicit complete journal/nested-input and peak portion limits;no default is supplied.
/// The portion budget counts both fold queues and the active input,including transient output.
/// Thus even a one-portion identity fold needs two slots while emitting its replacement.
/// These are logical populations,not allocator,RSS,CPU or whole Design authority certificates.
///
/// ```compile_fail
/// use sc_core::ontology::RangeCoverageLimits;
/// let limits: RangeCoverageLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeCoverageLimits {
    current: UniquePointLimits,
    portions: NonZeroUsize,
}
impl RangeCoverageLimits {
    /// Choose all positive limits explicitly,before input lookup or derived collection allocation.
    #[must_use]
    pub const fn new(current: UniquePointLimits, portions: NonZeroUsize) -> Self {
        Self { current, portions }
    }
    /// Complete journal and aggregate declared-edge/offset-fragment input limits.
    #[must_use]
    pub const fn current(self) -> UniquePointLimits {
        self.current
    }
    /// Peak simultaneously retained logical portions,including an active input and emitted output.
    #[must_use]
    pub const fn portions(self) -> NonZeroUsize {
        self.portions
    }
}

/// Actual input-admission or derived collection-growth refusal;no partial coverage is returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeCoverageIssue {
    /// Original whole-journal admission outcome;no point mapping or source lookup has occurred.
    Input(PointJournalAdmissionIssue),
    /// Emitting a single fixed-size portion would exceed the peak population.
    Portions {
        /// Actual mapping operation.
        operation: EntityId,
        /// Existing queued portions plus the active input.
        retained: usize,
        /// One next output,checked before its collection allocation.
        requested: usize,
        /// Caller-selected peak limit.
        limit: usize,
    },
    /// Peak count addition cannot be represented;no next portion was appended.
    CountOverflow {
        /// Actual mapping operation.
        operation: EntityId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Failure {
    held: EdgeRange,
    issue: RangeCoverageIssue,
}

/// Fixed original range and admission refusal;no journal or partial portion collection is retained.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RangeCoverageError(Box<Failure>);
impl RangeCoverageError {
    /// Actual caller-held original range.
    #[must_use]
    pub const fn held(&self) -> EdgeRange {
        self.0.held
    }
    /// Concrete original input or collection refusal.
    #[must_use]
    pub const fn issue(&self) -> &RangeCoverageIssue {
        &self.0.issue
    }
}
impl fmt::Display for RangeCoverageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "range coverage {:?}: {:?}", self.held(), self.issue())
    }
}
impl std::error::Error for RangeCoverageError {}

/// Complete ordered positive-interval evidence from one bounded journal observation.
/// Coverage says nothing about endpoint uniqueness,copy/Piece ownership or physical geometry.
/// A missing or deleted interval remains an explicit repair,including its actual numeric cause.
#[derive(PartialEq, Eq)]
pub struct RangeCoverage {
    held: EdgeRange,
    portions: Vec<RangePortion>,
}
impl fmt::Debug for RangeCoverage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { held, portions } = self;
        f.debug_struct("RangeCoverage")
            .field("held", held)
            .field("portions", &portions.len())
            .finish()
    }
}
impl RangeCoverage {
    // Closed consuming view:move all admitted evidence without cloning or dropping repairs.
    pub(crate) fn into_parts(self) -> (EdgeRange, Vec<RangePortion>) {
        let Self { held, portions } = self;
        (held, portions)
    }
    /// Original caller-held range.
    #[must_use]
    pub const fn held(&self) -> EdgeRange {
        self.held
    }
    /// All live portions and repairs,in the held range's forward traversal order.
    #[must_use]
    pub fn portions(&self) -> &[RangePortion] {
        &self.portions
    }
    /// Every positive-length portion survives;endpoint uniqueness remains a separate query.
    #[must_use]
    pub fn has_full_coverage(&self) -> bool {
        self.portions
            .iter()
            .all(|part| matches!(part, RangePortion::Resolved(_)))
    }
}

enum StepFailure {
    Numeric(UnitError),
    Admission(RangeCoverageIssue),
}
impl From<UnitError> for StepFailure {
    fn from(cause: UnitError) -> Self {
        Self::Numeric(cause)
    }
}

fn emit(
    next: &mut Vec<RangePortion>,
    pending: usize,
    operation: EntityId,
    limit: usize,
    part: RangePortion,
) -> Result<(), RangeCoverageIssue> {
    let overflow = || RangeCoverageIssue::CountOverflow { operation };
    let retained = pending
        .checked_add(next.len())
        .and_then(|count| count.checked_add(1))
        .ok_or_else(overflow)?;
    let total = retained.checked_add(1).ok_or_else(overflow)?;
    if total > limit {
        return Err(RangeCoverageIssue::Portions {
            operation,
            retained,
            requested: 1,
            limit,
        });
    }
    next.push(part);
    Ok(())
}

impl IdentityLedger {
    /// Fold all positive intervals with explicit input and peak collection limits.
    ///
    /// Uses the same exact per-edit mapping as the legacy whole-range resolver,without either
    /// endpoint point query. A required endpoint choice therefore cannot allocate its full tree.
    /// Unknown sources and actual deleted/offset-gap/numeric failures remain coverage repairs.
    /// A failed numeric mapping discards every emission from that input before retaining one repair.
    /// No registration or authored reference is changed;repeat after edits.
    ///
    /// # Errors
    /// Refuses whole-journal admission first,then collection growth before each append.
    /// Returns no partial coverage on a limit or count-overflow refusal.
    pub fn resolve_range_coverage(
        &self,
        held: EdgeRange,
        limits: RangeCoverageLimits,
    ) -> Result<RangeCoverage, RangeCoverageError> {
        let failure = |issue| RangeCoverageError(Box::new(Failure { held, issue }));
        limits
            .current
            .admit_journal(self)
            .map_err(|issue| failure(RangeCoverageIssue::Input(issue)))?;
        let original = ResolvedRange {
            range: held,
            direction: Direction::Original,
        };
        // Positive limit admits this single initial item. The source lookup precedes collection allocation.
        let mut work = if known_source(self, held.edge()) {
            vec![RangePortion::Resolved(original)]
        } else {
            return Ok(RangeCoverage {
                held,
                portions: vec![repair(held, original, RangeIssue::UnknownSource)],
            });
        };
        for entry in self.journal() {
            work.reverse();
            let mut next = Vec::new();
            while let Some(part) = work.pop() {
                let pending = work.len();
                let start = next.len();
                let mut push = |part| {
                    emit(
                        &mut next,
                        pending,
                        entry.operation(),
                        limits.portions.get(),
                        part,
                    )
                };
                let result = match part {
                    RangePortion::Unresolved(_) => push(part).map(|()| false),
                    RangePortion::Resolved(current) => {
                        match step_into(held, current, entry, |part| {
                            push(part).map_err(StepFailure::Admission)
                        }) {
                            Err(StepFailure::Numeric(cause)) => {
                                // Preserve the legacy all-or-repair semantics for each input.
                                next.truncate(start);
                                emit(
                                    &mut next,
                                    pending,
                                    entry.operation(),
                                    limits.portions.get(),
                                    repair(
                                        held,
                                        current,
                                        RangeIssue::Orphaned(OrphaningEdit::RecomputationFailed {
                                            operation: entry.operation(),
                                            edge: current.range.edge(),
                                            cause,
                                        }),
                                    ),
                                )
                                .map(|()| false)
                            }
                            Err(StepFailure::Admission(issue)) => Err(issue),
                            Ok(reverse) => Ok(reverse),
                        }
                    }
                };
                if result.map_err(failure)? {
                    // Iterators reverse only this input's emitted suffix,without unchecked indexing,
                    // extra collection storage or changing the mapper's first numeric failure.
                    let mut output = next.iter_mut().skip(start);
                    while let (Some(first), Some(last)) = (output.next(), output.next_back()) {
                        core::mem::swap(first, last);
                    }
                }
            }
            work = next;
        }
        Ok(RangeCoverage {
            held,
            portions: work,
        })
    }
}
