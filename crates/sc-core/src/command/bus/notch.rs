//! Actual semantic Notch candidate over a separately staged immutable identity reserve.
use super::{CommandBus, CommandPreviewError};
use crate::{
    command::{AddNotch, CommandAuthoritySource, DesignRevision},
    design::{DesignGarment, DesignReferenceHost, DesignSewing},
    ontology::{EntityId, IdGenerator, Notch, SewingLandmark},
};

/// Exact proposed added semantic Notch at an unchanged base revision; no publication capability.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandNotchPreview {
    design: EntityId,
    revision: DesignRevision,
    notch: Notch,
}
impl CommandNotchPreview {
    /// Actual inspected Design identity.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }
    /// Exact unchanged base revision.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
    /// Proposed actual added semantic mark, including its exact anchor and deferred profile bindings.
    /// Its identity is tentative until a separately authorized atomic publication; no reservation token.
    #[must_use]
    pub const fn notch(&self) -> &Notch {
        &self.notch
    }
}
impl<H: DesignReferenceHost + Clone + PartialEq, A: CommandAuthoritySource> CommandBus<H, A> {
    pub(super) fn preview_notch(
        &self,
        addition: AddNotch,
    ) -> Result<CommandNotchPreview, CommandPreviewError<H::Error>> {
        let structure = self.state.inputs().structure();
        let piece = structure
            .pieces()
            .pieces()
            .iter()
            .find(|piece| piece.id() == addition.piece())
            .ok_or(CommandPreviewError::MissingPiece {
                piece: addition.piece(),
                index: 0,
            })?;
        let mut identities = self.identities.stage();
        let identity = identities
            .next_id()
            .map_err(CommandPreviewError::Identity)?;
        let notch = Notch::new(
            addition.definition_with_id(identity),
            piece,
            structure.topology(),
        )
        .map_err(CommandPreviewError::Notch)?;
        let mut garment = self.state.definition().clone();
        let mut sewing = garment.sewing.definition().clone();
        sewing.landmarks.push(SewingLandmark::Notch(notch));
        garment.sewing =
            DesignSewing::new(sewing).map_err(|e| CommandPreviewError::Sewing(Box::new(e)))?;
        let candidate =
            DesignGarment::new(garment).map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        candidate
            .validate_current()
            .map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        super::difference::unchanged_context(&self.state, &candidate)
            .map_err(CommandPreviewError::Difference)?;
        if structure.pieces() != candidate.inputs().structure().pieces() {
            return Err(CommandPreviewError::Difference(
                super::CommandDifferenceError::Pieces,
            ));
        }
        let notch = super::difference::added_notch(
            self.state.sewing().landmarks(),
            candidate.sewing().landmarks(),
            addition,
            identity,
        )
        .map_err(CommandPreviewError::Difference)?;
        Ok(CommandNotchPreview {
            design: candidate.inputs().id(),
            revision: candidate.inputs().revision().clone(),
            notch,
        })
    }
}
