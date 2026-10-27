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
pub use material::{CommandMaterialChange, CommandMaterialPreview, CommandPreviewError};

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
/// it. Construction claims no permission, source freshness or physical/geometry readiness. Only
/// GetPieceList executes at this stage; AssignMaterial can prepare a private inspect-preview.
/// Mutation publication and histories remain absent.
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
pub struct CommandBus<H, A> {
    state: DesignGarment<H>,
    authority: A,
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
    #[must_use]
    pub const fn new(state: DesignGarment<H>, authority: A) -> Self {
        Self { state, authority }
    }

    /// Actual executable subset, separate from the full typed command catalog.
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
