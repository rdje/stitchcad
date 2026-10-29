//! Typed inputs, core authorization and metadata operations over private actual ownership.
//!
//! Catalog membership declares a command; it does not make a handler available or grant authority.
//! In-process metadata read/preview/group commit/undo are implemented. Other handlers, transport
//! authentication, persistence and workflow parity remain owned.

mod approval;
mod artifact;
mod authorization;
mod bus;
mod construction;
mod context;
mod control;
mod evaluation;
mod grading_selection;
mod grant;
mod group;
mod group_authorization;
mod history;
mod payload;
mod pdf_print;
mod plot;
mod print;
mod query;
mod release;
mod request;
mod resource;
pub use resource::{
    charge_command_history_request_resources, charge_command_mutation_group_resources,
};
mod scope;
mod seam_capture;
mod seam_ease;
mod seam_quantity;
mod seam_source;
mod seam_split;
mod seam_stops;
mod snapshot;
mod tech_pack;
mod workflow;
pub use approval::{ApprovalScopeTarget, ApprovePackage};
pub use artifact::{ArtifactPayload, CommandArtifact, DxfGrading, DxfTarget, ExportDxf};
pub use authorization::{
    authorize_command, CommandAuthoritySnapshot, CommandAuthoritySource, CommandAuthorization,
    CommandAuthorizationError, CommandAuthorizationMode, CommandGrantRefusal,
    CommandGrantRejection,
};
pub use bus::{
    charge_command_audit_entry_resources, charge_command_group_commit_resources,
    charge_command_group_commit_result_resources, charge_command_group_preview_resources,
    charge_command_history_audit_entry_resources, charge_command_history_result_resources,
    charge_command_preview_resources, CommandAuditEntry, CommandAuditSequence, CommandBus,
    CommandBusError, CommandControlError, CommandDifferenceError, CommandGroupCommit,
    CommandGroupCommitError, CommandGroupCommitResult, CommandGroupDifference,
    CommandGroupDiscardedStep, CommandGroupPreview, CommandGroupPreviewError,
    CommandGroupPreviewStep, CommandHistoryAuditEntry, CommandHistoryChange, CommandHistoryError,
    CommandHistoryResult, CommandIdentityError, CommandLengthChange, CommandMaterialChange,
    CommandMaterialPreview, CommandMeasurementPreview, CommandNotchPreview, CommandPieceList,
    CommandPreview, CommandPreviewError, CommandResult, CommandRevisionChange,
};
pub use construction::{AddNotch, CloseDart, SplitSeamSpan};
pub use context::{
    CommandActorClaim, CommandGroupId, CommandIdempotencyKey, CommandMutationContext,
    DesignRevision,
};
pub use control::{
    CommandOperationDemand, CommandOperationLimitError, CommandOperationLimits, CommandProgress,
    CommandProgressDecision, CommandProgressError, CommandProgressObserver, CommandProgressPhase,
};
pub use evaluation::{
    CommandEvaluation, EvaluateInstance, EvaluationPayload, EvaluationProfile, GradeSize,
    RegenerationChart, ResolveProfile, SizeMemberTarget,
};
pub use grading_selection::{GradedSizeSelection, GradedSizeSelectionError};
pub use grant::{AgentApprovalGrantError, CommandGrant, CommandGrantStatus};
pub use group::{CommandMutationGroup, CommandMutationGroupError};
pub use group_authorization::{
    authorize_mutation_group, CommandGroupAuthorization, CommandGroupAuthorizationError,
    CommandGroupGrantRejection,
};
pub use history::{CommandHistoryDirection, CommandHistoryRequest, CommandHistoryRequestError};
pub use payload::{
    AssignMaterial, AssignMaterialError, ChangeMeasurement, CommandMutation, MutationPayload,
    PieceSelection, PieceSelectionError,
};
pub use pdf_print::{
    PdfLayout, PdfOrientation, PdfPrintSettings, PdfPrintSettingsError, PdfTileOrder, PdfTilePaper,
};
pub use plot::ExportPlt;
pub use print::ExportPdf;
pub use query::{CommandQuery, CommandQueryContext, QueryPayload};
pub use release::{CommandRelease, PrepareRelease, PrepareReleaseError, ReleasePayload};
pub use request::CommandRequest;
pub use scope::{
    CommandScope, CommandScopeNarrowingError, CommandScopeRefusal, CommandScopeWindowError,
};
pub use seam_capture::{
    CommandEaseCapture, CommandEaseCaptureAdmission, CommandEaseCaptureCollection,
    CommandEaseCaptureError, CommandEaseCaptureLimits,
};
pub use seam_ease::{CommandUniformEaseError, CommandUniformEasePartition};
pub use seam_quantity::{
    CommandEaseQuantityStage, CommandNotchEaseError, CommandNotchEasePartition,
    CommandWeightedEaseError, CommandWeightedEasePartition,
};
pub use seam_source::{CommandSeamSplitSourceAdmission, CommandSeamSplitSourceError};
pub use seam_split::{CommandSeamSplitBinding, CommandSeamSplitBindingError, CommandSeamSplitPair};
pub use seam_stops::{
    CommandStopChild, CommandStopChildView, CommandStopContextBoundary, CommandStopContextError,
    CommandStopContextIssue, CommandStopContextPopulation, CommandStopContextRange,
    CommandStopCurrentContext, CommandStopLandmarkEvidence, CommandStopMembership,
    CommandStopMembershipError, CommandStopMembershipIssue, CommandStopMemberships,
    CommandStopOwnershipAdmission, CommandStopOwnershipAdmissionError, CommandStopOwnershipLimits,
    CommandStopPairMembership, CommandStopPartitionAdmission, CommandStopPartitionAdmissionError,
    CommandStopPartitionError, CommandStopPartitionLimits, CommandStopPartitions,
    CommandStopPiecePortion,
};
pub use snapshot::CommandSnapshotContext;
pub use tech_pack::{GenerateTechPack, InstanceSelection, InstanceSelectionError};
pub use workflow::{CommandWorkflowId, CommandWorkflowStep};

/// The five command classes fixed by the shared command-layer contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandClass {
    /// Observes canonical state without observable mutation.
    Query,
    /// Changes the authored design through an atomic recipe group.
    Mutation,
    /// Computes a derived instance or profile result.
    Evaluation,
    /// Produces an immutable artifact.
    Artifact,
    /// Prepares or approves a release package.
    Release,
}

impl CommandClass {
    /// Stable contract spelling; independent of localized presentation.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Query => "query",
            Self::Mutation => "mutation",
            Self::Evaluation => "evaluation",
            Self::Artifact => "artifact",
            Self::Release => "release",
        }
    }
}

/// Required permission metadata, not an authenticated grant or an actor-held capability.
/// Approve is human-only; possessing this enum value cannot authorize a release.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandAuthority {
    /// Read-only queries and nonmutating previews.
    Inspect,
    /// Proposal-workflow metadata; native mutation preview separately requires Inspect.
    Propose,
    /// Scoped authored design mutation.
    Commit,
    /// Evaluation, artifacts and release preparation.
    Generate,
    /// Deliberate human approval, never an agent grant.
    Approve,
}

impl CommandAuthority {
    /// Stable contract spelling; no authorization comparison is implied.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Propose => "propose",
            Self::Commit => "commit",
            Self::Generate => "generate",
            Self::Approve => "approve",
        }
    }
}

/// What the contract promises about reversing a command's result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandReversibility {
    /// A read has no changed state, or a produced artifact remains immutable.
    NotApplicable,
    /// One undo restores one complete authored atomic group.
    Yes,
    /// Derived results are discarded and recomputed, not undone.
    Discarded,
    /// Approval withdrawal is a new record rather than an undo.
    No,
}

impl CommandReversibility {
    /// Stable spelling of the normative reversibility vocabulary.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::NotApplicable => "not applicable",
            Self::Yes => "yes",
            Self::Discarded => "discarded",
            Self::No => "no",
        }
    }
}

/// Undo grouping metadata; an actual group and history belong to execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandUndoGranularity {
    /// Queries, derived results, artifacts and release records do not enter design undo.
    NotApplicable,
    /// One complete semantic group, however many entities it affects.
    AtomicGroup,
    /// Each changed measurement has its own group, including in a batched workflow.
    PerMeasurement,
}

/// Stable declared command identities, in the normative catalog's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CommandId {
    /// Inspect pieces and their structural metadata.
    GetPieceList,
    /// Inspect seam differential and ease attribution.
    WalkSeamSpan,
    /// Inspect unknowns affecting a requested artifact.
    GetUncertaintyClosure,
    /// Author one measurement change.
    ChangeMeasurement,
    /// Assign material to selected pieces.
    AssignMaterial,
    /// Split a seam span and resolve its dependent references.
    SplitSeamSpan,
    /// Close a dart as one semantic group.
    CloseDart,
    /// Add a notch to an edge parameter.
    AddNotch,
    /// Evaluate the recipe for one size.
    EvaluateInstance,
    /// Instantiate one size through grading.
    GradeSize,
    /// Resolve a factory profile against its constraints.
    ResolveProfile,
    /// Produce a registered DXF target.
    ExportDxf,
    /// Produce a plotter artifact with its pen map.
    ExportPlt,
    /// Produce PDF pages with their scale square.
    ExportPdf,
    /// Produce a tech pack from canonical content.
    GenerateTechPack,
    /// Prepare a release candidate and manifest.
    PrepareRelease,
    /// Bind deliberate human approval to a package identity.
    ApprovePackage,
}

impl CommandId {
    /// Every declared command, not a list of available execution handlers.
    pub const ALL: [Self; 17] = [
        Self::GetPieceList,
        Self::WalkSeamSpan,
        Self::GetUncertaintyClosure,
        Self::ChangeMeasurement,
        Self::AssignMaterial,
        Self::SplitSeamSpan,
        Self::CloseDart,
        Self::AddNotch,
        Self::EvaluateInstance,
        Self::GradeSize,
        Self::ResolveProfile,
        Self::ExportDxf,
        Self::ExportPlt,
        Self::ExportPdf,
        Self::GenerateTechPack,
        Self::PrepareRelease,
        Self::ApprovePackage,
    ];

    /// Stable command/diagnostic identity, without presentation or transport aliases.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::GetPieceList => "GetPieceList",
            Self::WalkSeamSpan => "WalkSeamSpan",
            Self::GetUncertaintyClosure => "GetUncertaintyClosure",
            Self::ChangeMeasurement => "ChangeMeasurement",
            Self::AssignMaterial => "AssignMaterial",
            Self::SplitSeamSpan => "SplitSeamSpan",
            Self::CloseDart => "CloseDart",
            Self::AddNotch => "AddNotch",
            Self::EvaluateInstance => "EvaluateInstance",
            Self::GradeSize => "GradeSize",
            Self::ResolveProfile => "ResolveProfile",
            Self::ExportDxf => "ExportDxf",
            Self::ExportPlt => "ExportPlt",
            Self::ExportPdf => "ExportPdf",
            Self::GenerateTechPack => "GenerateTechPack",
            Self::PrepareRelease => "PrepareRelease",
            Self::ApprovePackage => "ApprovePackage",
        }
    }

    /// Exact catalog lookup. Unknown names remain absent, without trimming or case folding.
    #[must_use]
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|command| command.token() == token)
    }

    /// The command's declared class, fixed in core metadata rather than by an adapter.
    #[must_use]
    pub const fn class(self) -> CommandClass {
        match self {
            Self::GetPieceList | Self::WalkSeamSpan | Self::GetUncertaintyClosure => {
                CommandClass::Query
            }
            Self::ChangeMeasurement
            | Self::AssignMaterial
            | Self::SplitSeamSpan
            | Self::CloseDart
            | Self::AddNotch => CommandClass::Mutation,
            Self::EvaluateInstance | Self::GradeSize | Self::ResolveProfile => {
                CommandClass::Evaluation
            }
            Self::ExportDxf | Self::ExportPlt | Self::ExportPdf | Self::GenerateTechPack => {
                CommandClass::Artifact
            }
            Self::PrepareRelease | Self::ApprovePackage => CommandClass::Release,
        }
    }

    /// Required permission for issuing this command; a mutation preview separately needs Inspect.
    #[must_use]
    pub const fn required_authority(self) -> CommandAuthority {
        match self {
            Self::ApprovePackage => CommandAuthority::Approve,
            _ => match self.class() {
                CommandClass::Query => CommandAuthority::Inspect,
                CommandClass::Mutation => CommandAuthority::Commit,
                CommandClass::Evaluation | CommandClass::Artifact | CommandClass::Release => {
                    CommandAuthority::Generate
                }
            },
        }
    }

    /// Declared reversibility; this does not execute undo, discard or approval withdrawal.
    #[must_use]
    pub const fn reversibility(self) -> CommandReversibility {
        match self {
            Self::ApprovePackage => CommandReversibility::No,
            _ => match self.class() {
                CommandClass::Mutation => CommandReversibility::Yes,
                CommandClass::Evaluation => CommandReversibility::Discarded,
                CommandClass::Query | CommandClass::Artifact | CommandClass::Release => {
                    CommandReversibility::NotApplicable
                }
            },
        }
    }

    /// Design-history grouping required when a mutation handler is implemented.
    #[must_use]
    pub const fn undo_granularity(self) -> CommandUndoGranularity {
        match self {
            Self::ChangeMeasurement => CommandUndoGranularity::PerMeasurement,
            _ => match self.class() {
                CommandClass::Mutation => CommandUndoGranularity::AtomicGroup,
                _ => CommandUndoGranularity::NotApplicable,
            },
        }
    }
}
