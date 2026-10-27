//! Actual private Material candidates; public reports contain no model or publication capability.
use super::{CommandPreview, CommandPreviewError, PreparationBase, PreparedCommand};
use crate::{
    command::{AssignMaterial, DesignRevision},
    design::{DesignGarment, DesignInputs, DesignReferenceHost, DesignSewing, DesignStructure},
    ontology::{EntityId, MaterialAssignment, PieceInventory},
};

/// One actual changed Piece assignment, in original request-selection order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMaterialChange {
    pub(super) piece: EntityId,
    pub(super) before: MaterialAssignment,
    pub(super) after: MaterialAssignment,
}
impl CommandMaterialChange {
    /// Original selected Piece identity.
    #[must_use]
    pub const fn piece(&self) -> EntityId {
        self.piece
    }
    /// Exact original assignment, including unresolved reason text.
    #[must_use]
    pub const fn before(&self) -> &MaterialAssignment {
        &self.before
    }
    /// Exact proposed assignment, without inferred physical properties or label text.
    #[must_use]
    pub const fn after(&self) -> &MaterialAssignment {
        &self.after
    }
}

/// Nonpublishing report from an actual validated private AssignMaterial candidate.
/// The candidate model is discarded; this report cannot be submitted as a commit token.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMaterialPreview {
    design: EntityId,
    revision: DesignRevision,
    changes: Vec<CommandMaterialChange>,
}
impl CommandMaterialPreview {
    /// Actual represented Design.
    #[must_use]
    pub const fn design(&self) -> EntityId {
        self.design
    }
    /// Exact inspected base revision, unchanged by preparation.
    #[must_use]
    pub const fn revision(&self) -> &DesignRevision {
        &self.revision
    }
    /// Actual semantic assignment changes; exact no-ops are omitted without reordering targets.
    #[must_use]
    pub fn changes(&self) -> &[CommandMaterialChange] {
        &self.changes
    }
}

impl<H: DesignReferenceHost + Clone + PartialEq> PreparationBase<'_, H> {
    pub(super) fn prepare_material(
        &self,
        assignment: &AssignMaterial,
    ) -> Result<PreparedCommand<H>, CommandPreviewError<H::Error>> {
        let originals = self.state.inputs().structure().pieces().pieces();
        for (index, identity) in assignment.pieces().pieces().iter().copied().enumerate() {
            originals
                .iter()
                .find(|piece| piece.id() == identity)
                .ok_or(CommandPreviewError::MissingPiece {
                    piece: identity,
                    index,
                })?;
        }
        let pieces = originals
            .iter()
            .map(|original| {
                if assignment.pieces().pieces().contains(&original.id()) {
                    original
                        .with_material_assignment(assignment.material().clone())
                        .map_err(|issue| CommandPreviewError::Piece {
                            piece: original.id(),
                            issue,
                        })
                } else {
                    Ok(original.clone())
                }
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut garment = self.state.definition().clone();
        let mut sewing = garment.sewing.definition().clone();
        let mut inputs = sewing.inputs.definition().clone();
        let mut structure = inputs.structure.definition().clone();
        structure.pieces =
            PieceInventory::new(pieces).map_err(|e| CommandPreviewError::Pieces(Box::new(e)))?;
        inputs.structure = DesignStructure::new(structure)
            .map_err(|e| CommandPreviewError::Structure(Box::new(e)))?;
        sewing.inputs =
            DesignInputs::new(inputs).map_err(|e| CommandPreviewError::Inputs(Box::new(e)))?;
        garment.sewing =
            DesignSewing::new(sewing).map_err(|e| CommandPreviewError::Sewing(Box::new(e)))?;
        let state =
            DesignGarment::new(garment).map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        state
            .validate_current()
            .map_err(|e| CommandPreviewError::Candidate(Box::new(e)))?;
        super::difference::unchanged_context(self.state, &state)
            .map_err(CommandPreviewError::Difference)?;
        if self.state.sewing().landmarks() != state.sewing().landmarks() {
            return Err(CommandPreviewError::Difference(
                super::CommandDifferenceError::Landmarks,
            ));
        }
        let changes = super::difference::material_changes(
            originals,
            state.inputs().structure().pieces().pieces(),
            assignment,
        )
        .map_err(CommandPreviewError::Difference)?;
        let report = CommandMaterialPreview {
            design: state.inputs().id(),
            revision: state.inputs().revision().clone(),
            changes,
        };
        Ok(PreparedCommand {
            state,
            report: CommandPreview::Material(report),
        })
    }
}
