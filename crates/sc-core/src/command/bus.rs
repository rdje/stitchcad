//! First executable command path over a privately owned actual garment and trusted authority source.
use super::{
    authorize_command, CommandAuthoritySource, CommandAuthorizationError, CommandAuthorizationMode,
    CommandId, CommandRequest, DesignRevision, QueryPayload,
};
use crate::{
    design::{DesignGarment, DesignReferenceHost},
    ontology::{EntityId, Piece},
};
use core::fmt;

mod material;
pub use material::{CommandMaterialChange, CommandMaterialPreview};
mod error;
pub use error::CommandPreviewError;
mod identity;
pub use identity::CommandIdentityError;
mod notch;
pub use notch::CommandNotchPreview;
mod difference;
pub use difference::CommandDifferenceError;
mod measurement;
pub use measurement::{CommandLengthChange, CommandMeasurementPreview};
mod preparation;
use preparation::PreparationBase;
mod operation;
pub use operation::CommandControlError;
mod group;
pub use group::{CommandGroupPreview, CommandGroupPreviewError, CommandGroupPreviewStep};
mod group_difference;
pub use group_difference::CommandGroupDifference;
mod execution;
pub use execution::{
    CommandAuditEntry, CommandAuditSequence, CommandGroupCommit, CommandGroupCommitError,
    CommandGroupCommitResult, CommandRevisionChange,
};
mod history;
pub use history::{
    CommandHistoryAuditEntry, CommandHistoryChange, CommandHistoryError, CommandHistoryResult,
};

/// Actual inspect-preview subset; reports carry semantic changes, never an owned candidate model.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandPreview {
    /// Exact bound canonical length replacement; no recipe evaluation result.
    Measurement(CommandMeasurementPreview),
    /// Exact changed Piece assignments in original selection order.
    Material(CommandMaterialPreview),
    /// Exact proposed added semantic Notch with tentative staged identity.
    Notch(CommandNotchPreview),
}

// Preparation retains the actual validated state and its derived report together. Only the bus
// module can hold this object; callers receive a report after the private model has been discarded.
struct PreparedCommand<H> {
    state: DesignGarment<H>,
    report: CommandPreview,
}
impl<H> PreparedCommand<H> {
    fn into_parts(self) -> (DesignGarment<H>, CommandPreview) {
        (self.state, self.report)
    }
    fn into_report(self) -> CommandPreview {
        let (state, report) = self.into_parts();
        drop(state);
        report
    }
}

/// Actual authored Piece inventory at one authorized exact revision; no geometry/readiness verdict.
/// Values borrow the private bus owner, so they cannot outlive it or become a publication token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandPieceList<'a> {
    design: EntityId,
    revision: &'a DesignRevision,
    pieces: &'a [Piece],
}
impl<'a> CommandPieceList<'a> {
    /// Actual containing Design, after authorization and exact request binding.
    #[must_use]
    pub const fn design(self) -> EntityId {
        self.design
    }
    /// Original represented revision; this read does not advance it.
    #[must_use]
    pub const fn revision(self) -> &'a DesignRevision {
        self.revision
    }
    /// Original complete authored order, including multiplicity, pairing and Material assignment.
    /// Unresolved assignments remain unresolved; held repair state remains inspectable.
    #[must_use]
    pub const fn pieces(self) -> &'a [Piece] {
        self.pieces
    }
}

/// Implemented execution results only; catalog membership is independent of handler availability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandResult<'a> {
    /// Authorized observation of the actual original Piece inventory.
    PieceList(CommandPieceList<'a>),
}

/// Structured refusal from the current executable bus subset; no partial state is published.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandBusError {
    /// Original core authentication/actor/grant/scope cause, before inspecting owned state.
    Authorization(CommandAuthorizationError),
    /// The authorized request targets another Design than this bus owns.
    DesignMismatch {
        /// Payload-derived command identity.
        command: CommandId,
        /// Original requested Design.
        requested: EntityId,
        /// Actual private owner's Design.
        current: EntityId,
    },
    /// The exact requested revision differs; future counters are refused as well as stale ones.
    RevisionMismatch {
        /// Payload-derived command identity.
        command: CommandId,
        /// Original exact requested counter, never narrowed to a storage integer.
        requested: DesignRevision,
        /// Actual represented counter.
        current: DesignRevision,
    },
    /// Typed input exists, but this command has no executable handler yet.
    HandlerUnavailable {
        /// Exact payload-derived identity, without treating a refusal as execution success.
        command: CommandId,
    },
}
impl fmt::Display for CommandBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "command bus refused: {self:?}")
    }
}
impl std::error::Error for CommandBusError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Authorization(issue) => Some(issue),
            Self::DesignMismatch { .. }
            | Self::RevisionMismatch { .. }
            | Self::HandlerUnavailable { .. } => None,
        }
    }
}

/// Private actual garment ownership and executable command dispatch over a trusted host dependency.
///
/// The application installs the authenticated authority provider; submitted requests cannot replace
/// it. Construction claims no permission, source freshness or physical/geometry readiness.
/// The execute entry point supports GetPieceList; ChangeMeasurement, AssignMaterial and AddNotch prepare
/// private metadata inspect-previews, individually or in ordered atomic-group previews.
/// Recipe evaluation remains separate.
/// Explicit finite edit capacity enables atomic metadata-group publication and in-memory replay/audit.
/// Explicit history bounds enable actual whole-model undo/redo and an independent action audit.
/// Persistence, resource/work bounds and full integration remain separate implementation work.
/// There is no model getter, mutable borrow, state replacement or caller-minted commit capability.
///
/// ```compile_fail
/// use sc_core::{command::CommandBus, design::DesignGarment};
/// fn replace<H, A>(bus: &mut CommandBus<H, A>, other: DesignGarment<H>) {
///     bus.state = other;
/// }
/// ```
/// ```compile_fail
/// use sc_core::{command::{CommandBus, CommandAuthoritySource, CommandRequest, CommandResult},
///     design::DesignReferenceHost};
/// fn retain<H: DesignReferenceHost, A: CommandAuthoritySource>(
///     bus: &CommandBus<H, A>, request: &CommandRequest
/// ) -> Option<CommandResult<'static>> {
///     bus.execute(request).ok()
/// }
/// ```
/// ```compile_fail
/// use sc_core::command::CommandBus;
/// fn replace_reserved_ids<H, A>(bus: &mut CommandBus<H, A>) {
///     bus.identities = Default::default();
/// }
/// ```
/// ```compile_fail
/// use sc_core::{command::{CommandBus, CommandAuthoritySource, CommandRequest},
///     design::DesignReferenceHost};
/// fn take_prepared<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource>(
///     bus: &CommandBus<H, A>, request: &CommandRequest,
/// ) {
///     let prepared = bus.prepare(request);
/// }
/// ```
/// ```compile_fail
/// use sc_core::{command::{CommandBus, CommandAuthoritySource, CommandAuthorizationMode,
///     CommandMutationGroup}, design::DesignReferenceHost};
/// fn take_prepared_group<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource>(
///     bus: &CommandBus<H, A>, group: &CommandMutationGroup,
/// ) {
///     let prepared = bus.prepare_group(group, CommandAuthorizationMode::Execute);
/// }
/// ```
/// ```compile_fail
/// use sc_core::command::CommandBus;
/// fn erase_replay_and_audit<H, A>(bus: &mut CommandBus<H, A>) {
///     bus.journal = None;
/// }
/// ```
/// ```compile_fail
/// use sc_core::command::CommandBus;
/// fn erase_history_and_action_audit<H, A>(bus: &mut CommandBus<H, A>) {
///     bus.history = None;
/// }
/// ```
/// ```compile_fail
/// use sc_core::command::CommandBus;
/// fn reset_operation_policy<H, A>(bus: &mut CommandBus<H, A>) {
///     bus.operation_limits = None;
/// }
/// ```
pub struct CommandBus<H, A> {
    state: DesignGarment<H>,
    authority: A,
    identities: identity::IdentityReserve,
    journal: Option<execution::CommandJournal>,
    history: Option<history::History<H>>,
    operation_limits: Option<crate::command::CommandOperationLimits>,
}

impl<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Implemented inspect-preview subset with installed operation policy; supplies no authority.
    #[must_use]
    pub const fn is_preview_available(&self, command: CommandId) -> bool {
        self.operation_limits.is_some() && preparation::handler_available(command)
    }

    /// Freshly authorize inspect, bind the actual Design/revision and prepare a private candidate.
    /// The original model, revision and reserved identities remain unchanged on success or refusal.
    /// Candidates are discarded; reports are no geometry, physical-property or release certificate.
    /// # Errors
    /// Supported metadata first requires installed policy and pure submitted-input admission. Then
    /// authority/binding, handler and original envelope/current source checks precede target lookup
    /// and actual candidate construction. No progress or private source access precedes admission.
    pub fn preview(
        &self,
        request: &CommandRequest,
    ) -> Result<CommandPreview, CommandPreviewError<H::Error>> {
        self.prepare(request).map(PreparedCommand::into_report)
    }

    fn prepare(
        &self,
        request: &CommandRequest,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        self.prepare_with_progress(request, &())
    }

    /// Preview with actual phase-local progress and cooperative pre-effect cancellation.
    /// Explicit installed operation limits still apply; the observer receives no model or authority.
    /// # Errors
    /// Preserves original preparation causes and typed setup/input/cancellation refusals.
    pub fn preview_with_progress(
        &self,
        request: &CommandRequest,
        observer: &dyn crate::command::CommandProgressObserver,
    ) -> Result<CommandPreview, CommandPreviewError<H::Error>> {
        self.prepare_with_progress(request, observer)
            .map(PreparedCommand::into_report)
    }

    fn prepare_with_progress(
        &self,
        request: &CommandRequest,
        observer: &dyn crate::command::CommandProgressObserver,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        if let CommandRequest::Mutation(mutation) = request {
            if preparation::handler_available(mutation.command_id()) {
                self.admit_mutation(mutation)?;
            }
        }
        self.authorize_bound(request, CommandAuthorizationMode::Preview)
            .map_err(|e| CommandPreviewError::Command(Box::new(e)))?;
        let CommandRequest::Mutation(mutation) = request else {
            return Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable {
                    command: request.command_id(),
                },
            )));
        };
        if !preparation::handler_available(mutation.command_id()) {
            return Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable {
                    command: mutation.command_id(),
                },
            )));
        }
        use crate::command::CommandProgressPhase as Phase;
        operation::checkpoint(observer, Phase::Authorized, 1, 1, None)?;
        operation::checkpoint(observer, Phase::SnapshotBound, 1, 1, None)?;
        let member = Some((0, mutation.command_id()));
        operation::checkpoint(observer, Phase::Preparing, 0, 1, member)?;
        let base = PreparationBase { state: &self.state };
        let mut identities = self.identities.stage();
        let prepared =
            base.prepare_payload(mutation.payload(), &mut identities, observer, member)?;
        operation::checkpoint(observer, Phase::Preparing, 1, 1, member)?;
        Ok(prepared)
    }
}
impl<H, A> fmt::Debug for CommandBus<H, A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Logging the owner must not expose a model or authenticated session/grant inventory.
        f.debug_struct("CommandBus").finish_non_exhaustive()
    }
}
impl<H: DesignReferenceHost, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Install the original immutable model and trusted application authority dependency.
    /// This moves the sole owner without reconstructing held references or certifying execution.
    /// The identity reserve is empty; creation previews refuse exhaustion until configured through
    /// `with_identities` at construction. Costly metadata paths also require install_operation_limits;
    /// authorized Piece inspection needs no policy. Material previews allocate no identities.
    #[must_use]
    pub fn new(state: DesignGarment<H>, authority: A) -> Self {
        Self {
            state,
            authority,
            identities: identity::IdentityReserve::default(),
            journal: None,
            history: None,
            operation_limits: None,
        }
    }

    /// Install explicit unused identities reserved by the trusted application generator.
    /// Production clock/entropy and deterministic replay stay outside command inputs. The reserve
    /// is private; previews borrow a separate cursor and cannot call or advance an external source.
    /// The application must ensure global nonreuse across buses/restarts; this local check covers
    /// the actual garment and reserve. An empty reserve is valid but creation reports exhaustion.
    /// Install operation limits separately before costly metadata previews/effects.
    /// # Errors
    /// Refuses non-increasing supplied order, then the first identity already owned by this garment.
    pub fn with_identities(
        state: DesignGarment<H>,
        authority: A,
        identities: Vec<EntityId>,
    ) -> Result<Self, CommandIdentityError> {
        let identities = identity::IdentityReserve::new(identities, &state)?;
        Ok(Self {
            state,
            authority,
            identities,
            journal: None,
            history: None,
            operation_limits: None,
        })
    }

    /// Implemented `execute` entry-point subset, separate from `commit_group` and the full catalog.
    #[must_use]
    pub const fn is_available(&self, command: CommandId) -> bool {
        matches!(command, CommandId::GetPieceList)
    }

    /// Authorize afresh, bind the actual Design/revision, then execute the implemented read.
    /// No model/cache/revision/identity-allocation state changes on success or refusal. This metadata
    /// query intentionally permits inspection of unresolved assignments and held topology repairs;
    /// it neither runs geometry nor returns a current-source, envelope or release certificate.
    /// # Errors
    /// Returns the original authorization cause, wrong Design, revision mismatch, then unavailable
    /// handler in that order. No mutation is implemented or partially applied by this entry point.
    pub fn execute(&self, request: &CommandRequest) -> Result<CommandResult<'_>, CommandBusError> {
        self.authorize_bound(request, CommandAuthorizationMode::Execute)?;
        let command = request.command_id();
        let inputs = self.state.inputs();
        match request {
            CommandRequest::Query(query)
                if matches!(query.payload(), QueryPayload::GetPieceList) =>
            {
                Ok(CommandResult::PieceList(CommandPieceList {
                    design: inputs.id(),
                    revision: inputs.revision(),
                    pieces: inputs.structure().pieces().pieces(),
                }))
            }
            _ => Err(CommandBusError::HandlerUnavailable { command }),
        }
    }

    fn authorize_bound(
        &self,
        request: &CommandRequest,
        mode: CommandAuthorizationMode,
    ) -> Result<(), CommandBusError> {
        authorize_command(request, mode, &self.authority)
            .map_err(CommandBusError::Authorization)?;
        let command = request.command_id();
        let inputs = self.state.inputs();
        if request.design() != inputs.id() {
            return Err(CommandBusError::DesignMismatch {
                command,
                requested: request.design(),
                current: inputs.id(),
            });
        }
        if request.expected_revision() != inputs.revision() {
            return Err(CommandBusError::RevisionMismatch {
                command,
                requested: request.expected_revision().clone(),
                current: inputs.revision().clone(),
            });
        }
        Ok(())
    }
}
