//! Explicit endpoint/stop-preserving seam refit intent;no fitted curve or editing authority.
use crate::ontology::{EntityId, SeamSideId};
use core::fmt;
use sc_units::Tolerance;
mod copies;
pub use copies::{
    BoundTruingCopyInventory, TruingCopyInventoryError, TruingCopyInventoryLimits,
    TruingCopyInventoryRefusal,
};
mod roles;
pub use roles::{BoundTruingEdgeRoles, TruingEdgeRoleBindingError, TruingEdgeRoleBindingRefusal};
mod manifest;
pub use manifest::{
    AdmittedTruingEdges, TruingEdgeCandidate, TruingEdgeManifest, TruingEdgeManifestError,
    TruingEdgeManifestLimits, TruingEdgeManifestRefusal, TruingOutputEdge,
};
mod domain;
pub use domain::{
    BoundTruingSourceDomain, TruingSourceDomainEdge, TruingSourceDomainError,
    TruingSourceDomainLimits, TruingSourceDomainRefusal,
};
mod coverage;
pub use coverage::{
    CoveredTruingSourceDomain, TruingSourceCoverageError, TruingSourceCoverageLimits,
    TruingSourceCoverageRefusal,
};
mod outcome;
pub use outcome::{
    RequiredTruingOutcome, TruingOutcomeError, TruingOutcomeIntent, TruingOutcomeRefusal,
};
mod binding;
mod piece_source;
pub use piece_source::{
    BoundExactTruingPieceSource, TruingPieceOccurrence, TruingPieceOccurrenceField,
    TruingPieceOccurrenceLocation, TruingPieceSourceError, TruingPieceSourceLimits,
    TruingPieceSourceRefusal,
};
mod directions;
pub use directions::{
    AdmittedTruingDirections, TruingDirectionAssignment, TruingDirectionError,
    TruingDirectionLimits, TruingDirectionManifest, TruingDirectionRefusal, TruingDirectionSource,
    TruingDirectionalOutput, TruingParameterDirection,
};
mod outside;
pub use outside::{
    AdmittedTruingOutsideRoles, TruingOutsideIntent, TruingOutsideOutput, TruingOutsidePart,
    TruingOutsideRoleError, TruingOutsideRoleLimits, TruingOutsideRoleRefusal, TruingOutsideRoles,
};
mod evidence;
pub use evidence::{
    RequiredTruingEvidence, TruingContextSources, TruingEffectSources, TruingEvidenceKind,
    TruingEvidencePolicy, TruingEvidenceRequirement, TruingFitSources,
};
mod locks;
pub use binding::{BoundConstrainedTruing, TruingBindingError, TruingBindingRefusal};
pub use locks::{
    RequiredTruingLocks, TruingEndpoint, TruingEndpointRequirement, TruingLockError,
    TruingLockLimits, TruingLockRefusal, TruingStopRequirement,
};

/// Required quantitative length-fit source;the actual tolerance remains borrowed and unchanged.
/// Source-registry membership,derivation truth and fit/error-budget evidence remain separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingFitProvider<'a> {
    /// Actual requirement/profile source identity,not an invented formula tolerance slot.
    pub source: EntityId,
    /// Original quantitative content,including class,limit and requirement derivation.
    pub tolerance: &'a Tolerance,
}

/// Required existing input role;an operation cannot supply one of its own prerequisites.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TruingInputRole {
    /// Canonical SewingGraph span.
    Span,
    /// Pattern Piece to refit,not its selected physical copy.
    Piece,
    /// Requirement/profile source for the geometric length-fit budget.
    FitTolerance,
}
/// Definite local source cycle;complete program namespaces and dependency graphs remain separate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TruingInputError {
    /// Original operation identity,also the refused prerequisite identity.
    pub operation: EntityId,
    /// First self-supplied role in span,Piece,tolerance order.
    pub role: TruingInputRole,
}
impl fmt::Display for TruingInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "truing {} supplies its own {:?} input",
            self.operation, self.role
        )
    }
}
impl std::error::Error for TruingInputError {}

/// Deliberate refit of one span side's pattern Piece,preserving both endpoint positions and stops.
/// The type explicitly selects this constrained policy;it grants no single-copy fork,endpoint
/// movement,ease replacement,target length,geometry or command authority. Infeasible fits must refuse.
/// ```compile_fail
/// let edit:sc_core::construction::ConstrainedSeamTruing=Default::default();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConstrainedSeamTruing {
    operation: EntityId,
    span: EntityId,
    adjusted_side: SeamSideId,
    piece: EntityId,
    fit_tolerance: EntityId,
}
impl ConstrainedSeamTruing {
    /// Retain all original sources and explicit adjusted side without evaluating them.
    /// # Errors
    /// Refuses first self-span,then self-Piece,then self-tolerance. Equal external identities
    /// are retained;current namespace,source existence and complete dependency checks are separate.
    pub fn new(
        operation: EntityId,
        span: EntityId,
        adjusted_side: SeamSideId,
        piece: EntityId,
        fit_tolerance: EntityId,
    ) -> Result<Self, TruingInputError> {
        for (role, source) in [
            (TruingInputRole::Span, span),
            (TruingInputRole::Piece, piece),
            (TruingInputRole::FitTolerance, fit_tolerance),
        ] {
            if source == operation {
                return Err(TruingInputError { operation, role });
            }
        }
        Ok(Self {
            operation,
            span,
            adjusted_side,
            piece,
            fit_tolerance,
        })
    }
    /// Original creating operation identity;not a newly issued command identity.
    #[must_use]
    pub const fn operation(&self) -> EntityId {
        self.operation
    }
    /// Required existing canonical span id.
    #[must_use]
    pub const fn span_id(&self) -> EntityId {
        self.span
    }
    /// Side whose canonical pattern Piece may be refitted.
    #[must_use]
    pub const fn adjusted_side(&self) -> SeamSideId {
        self.adjusted_side
    }
    /// Explicit target pattern Piece,shared by all its physical copies.
    #[must_use]
    pub const fn piece_id(&self) -> EntityId {
        self.piece
    }
    /// Required sourced length-fit budget;no implicit EPSgeo or physical tolerance.
    #[must_use]
    pub const fn fit_tolerance_source(&self) -> EntityId {
        self.fit_tolerance
    }
}
