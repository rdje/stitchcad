//! Trusted immutable operation policy and data-only pre-effect checkpoints.
use super::CommandBus;
use crate::command::{
    CommandId, CommandMutation, CommandMutationGroup, CommandOperationDemand,
    CommandOperationLimitError, CommandOperationLimits, CommandProgress, CommandProgressDecision,
    CommandProgressError, CommandProgressObserver, CommandProgressPhase,
};
use core::{fmt, num::NonZeroUsize};

/// Original setup/admission/cancellation refusal, before a metadata effect is published.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandControlError {
    /// Trusted composition root has not installed explicit operation policy.
    LimitsNotInstalled,
    /// Installed policy is immutable for this owner's lifetime, including after accepted retries.
    LimitsAlreadyInstalled,
    /// Original submitted metadata demand exceeds host policy.
    Input(CommandOperationLimitError),
    /// Host requested cancellation at this actual pre-effect checkpoint.
    Cancelled {
        /// Exact phase-local counts and original member context observed before refusing.
        progress: CommandProgress,
    },
    /// Internal progress producer supplied an empty phase rather than a positive denominator.
    EmptyProgress {
        /// Actual attempted named phase.
        phase: CommandProgressPhase,
    },
    /// Original invalid phase-fraction cause; never clip or fabricate a denominator.
    Progress(CommandProgressError),
}
impl fmt::Display for CommandControlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "operation control refused: {self:?}")
    }
}
impl std::error::Error for CommandControlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(issue) => Some(issue),
            Self::Progress(issue) => Some(issue),
            _ => None,
        }
    }
}

impl<H, A> CommandBus<H, A> {
    /// Install host-chosen metadata policy exactly once; neither requests nor observers can replace it.
    /// Read-only Piece inspection needs no policy. Costly metadata previews/effects require this setup.
    /// This changes no Design, grant, revision, reserve, audit or history and confers no authority.
    /// # Errors
    /// Returns LimitsAlreadyInstalled, retaining the original policy unchanged.
    pub fn install_operation_limits(
        &mut self,
        limits: CommandOperationLimits,
    ) -> Result<(), CommandControlError> {
        if self.operation_limits.is_some() {
            return Err(CommandControlError::LimitsAlreadyInstalled);
        }
        self.operation_limits = Some(limits);
        Ok(())
    }
    pub(super) fn admit_mutation(
        &self,
        mutation: &CommandMutation,
    ) -> Result<CommandOperationDemand, CommandControlError> {
        self.operation_limits
            .as_ref()
            .ok_or(CommandControlError::LimitsNotInstalled)?
            .check_mutation(mutation)
            .map_err(CommandControlError::Input)
    }
    pub(super) fn admit_group(
        &self,
        group: &CommandMutationGroup,
    ) -> Result<CommandOperationDemand, CommandControlError> {
        self.operation_limits
            .as_ref()
            .ok_or(CommandControlError::LimitsNotInstalled)?
            .check_group(group)
            .map_err(CommandControlError::Input)
    }
}

pub(super) fn checkpoint(
    observer: &dyn CommandProgressObserver,
    phase: CommandProgressPhase,
    completed: usize,
    total: usize,
    member: Option<(usize, CommandId)>,
) -> Result<(), CommandControlError> {
    let total = NonZeroUsize::new(total).ok_or(CommandControlError::EmptyProgress { phase })?;
    let progress = CommandProgress::new(phase, completed, total, member)
        .map_err(CommandControlError::Progress)?;
    match observer.observe(&progress) {
        CommandProgressDecision::Continue => Ok(()),
        CommandProgressDecision::Cancel => Err(CommandControlError::Cancelled { progress }),
    }
}
pub(super) fn group_checkpoint<E>(
    group: &CommandMutationGroup,
    observer: &dyn CommandProgressObserver,
    phase: CommandProgressPhase,
    completed: usize,
    total: usize,
    member: Option<(usize, CommandId)>,
) -> Result<(), super::CommandGroupPreviewError<E>> {
    checkpoint(observer, phase, completed, total, member)
        .map_err(|issue| super::CommandGroupPreviewError::control(group.id(), issue))
}
