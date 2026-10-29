//! Complete actual bounded query/refusal evidence;no partial accepted context.
use crate::ontology::{
    EntityId, RangeCoverage, RangeCoverageError, RangePortion, SeamSideId, SewingError,
    UniquePointResolution,
};
use core::fmt;

/// Actual bounded interval query location;indices retain authored Piece traversal provenance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopContextRange {
    /// Canonical held Piece edge,flattened boundary/hole/construction order.
    PieceEdge {
        /// Actual owning Piece.
        piece: EntityId,
        /// Zero-based authored edge position.
        index: usize,
    },
    /// Complete actual target side.
    TargetSide(SeamSideId),
    /// Named side of first A-ordered pair.
    FirstPair(SeamSideId),
    /// Named side of second A-ordered pair.
    SecondPair(SeamSideId),
}
/// Actual held side boundary queried without any reference-choice tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopContextBoundary {
    /// Original target's lower bound.
    Start,
    /// Actual requested cut.
    Cut,
    /// Original target's upper bound.
    End,
}
/// Derived logical collection guarded before append/insert under already admitted bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandStopContextPopulation {
    /// Complete actual copies.
    Copies,
    /// Complete actual Pieces.
    Pieces,
    /// Every actual semantic mark,including unused entries.
    Landmarks,
    /// All Piece live/repair interval portions,without sampling.
    OwnedPortions,
}
/// Concrete source/current/frame/ownership/query/derived refusal with complete admitted evidence.
#[derive(Debug, PartialEq, Eq)]
pub enum CommandStopContextIssue {
    /// Original actual copy/Piece/identity/self-overlap source refusal.
    Structural(SewingError),
    /// Actual semantic mark's held Piece is absent.
    MissingLandmarkPiece {
        /// Actual mark identity.
        landmark: EntityId,
        /// Held Piece identity.
        piece: EntityId,
    },
    /// Actual mark requires a choice/repair or has unknown source;no candidate is chosen.
    LandmarkPoint {
        /// Actual mark identity.
        landmark: EntityId,
        /// Complete fixed held/current outcome.
        resolution: UniquePointResolution,
    },
    /// Actual unique current mark lies outside its Piece's surviving intervals.
    LandmarkOutsidePiece {
        /// Actual mark identity.
        landmark: EntityId,
        /// Actual Piece owner.
        piece: EntityId,
        /// Actual held/current point evidence.
        resolution: UniquePointResolution,
    },
    /// Actual target endpoint/cut requires a choice/repair or has unknown source.
    Boundary {
        /// Actual named side.
        side: SeamSideId,
        /// Actual lower/cut/upper query.
        boundary: CommandStopContextBoundary,
        /// Complete actual held/current point outcome.
        resolution: UniquePointResolution,
    },
    /// Actual interval quota/count/input refusal;no accepted partial coverage is returned.
    CoverageLimit {
        /// Actual query location.
        reference: CommandStopContextRange,
        /// Original complete query refusal and held interval.
        issue: RangeCoverageError,
    },
    /// Actual positive target/pair interval retains lost/unknown/numeric repair evidence.
    Coverage {
        /// Actual query location.
        reference: CommandStopContextRange,
        /// Complete bounded coverage,including every repair.
        evidence: RangeCoverage,
    },
    /// Current interval is not covered by the actual copy's Piece surviving interval union.
    OutsidePiece {
        /// Actual interval query location.
        reference: CommandStopContextRange,
        /// Actual current Piece owner.
        piece: EntityId,
        /// Complete actual query evidence,without guessed reassignment.
        evidence: RangeCoverage,
    },
    /// Growing a derived population exceeds its pre-admitted bound or count representation.
    DerivedItems {
        /// Actual population.
        population: CommandStopContextPopulation,
        /// Actual attempted owner.
        owner: EntityId,
        /// Already retained population.
        retained: usize,
        /// One requested row.
        requested: usize,
        /// Exact pre-admitted conservative cap.
        limit: usize,
    },
}
#[derive(Debug, PartialEq, Eq)]
struct Failure {
    target_span: EntityId,
    issue: CommandStopContextIssue,
}
/// Actual target and complete original bounded context refusal;no partial successful context.
#[derive(Debug, PartialEq, Eq)]
pub struct CommandStopContextError(Box<Failure>);
impl CommandStopContextError {
    pub(super) fn new(target_span: EntityId, issue: CommandStopContextIssue) -> Self {
        Self(Box::new(Failure { target_span, issue }))
    }
    /// Actual consuming target,distinct from any retained original ease source.
    #[must_use]
    pub const fn target_span(&self) -> EntityId {
        self.0.target_span
    }
    /// Complete actual refusal,including raw choice/repair/coverage evidence.
    #[must_use]
    pub const fn issue(&self) -> &CommandStopContextIssue {
        &self.0.issue
    }
}
impl fmt::Display for CommandStopContextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "command stop target {} current context: {:?}",
            self.target_span(),
            self.issue()
        )
    }
}
impl std::error::Error for CommandStopContextError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.issue() {
            CommandStopContextIssue::Structural(issue) => Some(issue),
            CommandStopContextIssue::CoverageLimit {
                reference: _,
                issue,
            } => Some(issue),
            CommandStopContextIssue::LandmarkPoint {
                landmark: _,
                resolution,
            }
            | CommandStopContextIssue::LandmarkOutsidePiece {
                landmark: _,
                piece: _,
                resolution,
            }
            | CommandStopContextIssue::Boundary {
                side: _,
                boundary: _,
                resolution,
            } => resolution
                .numeric_cause()
                .map(|cause| cause as &dyn std::error::Error),
            CommandStopContextIssue::Coverage {
                reference: _,
                evidence,
            }
            | CommandStopContextIssue::OutsidePiece {
                reference: _,
                piece: _,
                evidence,
            } => evidence.portions().iter().find_map(|part| match part {
                RangePortion::Resolved(_) => None,
                RangePortion::Unresolved(repair) => repair
                    .numeric_cause()
                    .map(|cause| cause as &dyn std::error::Error),
            }),
            CommandStopContextIssue::MissingLandmarkPiece {
                landmark: _,
                piece: _,
            }
            | CommandStopContextIssue::DerivedItems {
                population: _,
                owner: _,
                retained: _,
                requested: _,
                limit: _,
            } => None,
        }
    }
}
