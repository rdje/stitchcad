//! Private actual-model view and concrete staged identity cursor, without caller-pluggable execution.
use super::{identity::IdentityStage, CommandBusError, CommandPreviewError, PreparedCommand};
use crate::{
    command::{CommandId, MutationPayload},
    design::{DesignGarment, DesignReferenceHost},
};

pub(super) const fn handler_available(command: CommandId) -> bool {
    matches!(
        command,
        CommandId::ChangeMeasurement | CommandId::AssignMaterial | CommandId::AddNotch
    )
}

// Only bus internals can construct this view from their actual original or validated staged owner.
// It borrows immutable model data; it contains no replacement capability or authority provider.
pub(super) struct PreparationBase<'a, H> {
    pub(super) state: &'a DesignGarment<H>,
}
impl<H: DesignReferenceHost + Clone + PartialEq> PreparationBase<'_, H> {
    pub(super) fn prepare_payload(
        &self,
        payload: &MutationPayload,
        identities: &mut IdentityStage<'_>,
        observer: &dyn crate::command::CommandProgressObserver,
        member: Option<(usize, CommandId)>,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        let command = payload.command_id();
        if !handler_available(command) {
            return Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable { command },
            )));
        }
        use crate::command::CommandProgressPhase as Phase;
        super::operation::checkpoint(observer, Phase::CurrentValidation, 0, 1, member)?;
        self.state
            .require_in_scope()
            .map_err(|e| CommandPreviewError::Envelope(Box::new(e)))?;
        self.state
            .validate_current()
            .map_err(|e| CommandPreviewError::Current(Box::new(e)))?;
        super::operation::checkpoint(observer, Phase::CurrentValidation, 1, 1, member)?;
        super::operation::checkpoint(observer, Phase::CandidateValidation, 0, 1, member)?;
        let prepared = match payload {
            MutationPayload::ChangeMeasurement(replacement) => {
                self.prepare_measurement(replacement)
            }
            MutationPayload::AssignMaterial(assignment) => self.prepare_material(assignment),
            MutationPayload::AddNotch(addition) => self.prepare_notch(*addition, identities),
            _ => Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable { command },
            ))),
        }?;
        super::operation::checkpoint(observer, Phase::CandidateValidation, 1, 1, member)?;
        Ok(prepared)
    }
}
