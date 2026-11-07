//! Replacing the drafting document: an authored edit with a revision, an actor and an undo, which the core holds and does not read.
use super::{CommandPreview, CommandPreviewError, PreparationBase, PreparedCommand};
use crate::{
    command::{ChangeDrafting, DesignRevision},
    design::{DesignGarment, DesignResourceSource},
    ontology::EntityId,
};

/// What replacing the drafting document would do.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandDraftingPreview {
    /// The design.
    pub design: EntityId,
    /// The revision it would become.
    pub revision: DesignRevision,
    /// Whether the document differs from the current one.
    pub changed: bool,
}

impl<H: DesignResourceSource + Clone + PartialEq> PreparationBase<'_, H> {
    pub(super) fn prepare_drafting(
        &self,
        change: &ChangeDrafting,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        let mut garment = self.state.definition().clone();
        let changed = garment.drafting != change.document();
        change.document().clone_into(&mut garment.drafting);
        let state =
            DesignGarment::new(garment).map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        state
            .validate_current()
            .map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        let report = CommandDraftingPreview {
            design: state.inputs().id(),
            revision: state.inputs().revision().clone(),
            changed,
        };
        Ok(PreparedCommand {
            state,
            report: CommandPreview::Drafting(report),
        })
    }
}
