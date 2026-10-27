//! Actual private Material candidates; public reports contain no model or publication capability.
use super::{CommandBus, CommandBusError};
use crate::{
    command::{
        CommandAuthoritySource, CommandAuthorizationMode, CommandId, CommandRequest,
        DesignRevision, MutationPayload,
    },
    design::{
        DesignGarment, DesignGarmentEnvelopeError, DesignGarmentError, DesignInputs,
        DesignInputsError, DesignReferenceHost, DesignSewing, DesignSewingError, DesignStructure,
        DesignStructureError,
    },
    ontology::{EntityId, MaterialAssignment, PieceError, PieceInventory, PieceInventoryError},
};
use core::fmt;

/// One actual changed Piece assignment, in original request-selection order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandMaterialChange {
    piece: EntityId,
    before: MaterialAssignment,
    after: MaterialAssignment,
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

/// Original preparation failures, without partial publication or a substitute model.
#[derive(Debug)]
pub enum CommandPreviewError<E> {
    /// Authentication, request binding or unavailable-handler cause.
    Command(Box<CommandBusError>),
    /// Original explicit execution-envelope cause before source/operation work.
    Envelope(Box<DesignGarmentEnvelopeError>),
    /// Original complete current source cause before copying a candidate.
    Current(Box<DesignGarmentError<E>>),
    /// Actual selection names no Piece; index is zero-based in the request.
    MissingPiece {
        /// Original requested identity.
        piece: EntityId,
        /// Original selection index.
        index: usize,
    },
    /// Original local immutable Piece-replacement cause.
    Piece {
        /// Original selected identity.
        piece: EntityId,
        /// Original local validation cause.
        issue: PieceError,
    },
    /// Original whole pair-consistent inventory cause.
    Pieces(Box<PieceInventoryError>),
    /// Original structural ownership/copy-plan cause.
    Structure(Box<DesignStructureError>),
    /// Original current registry/input cause, including a missing assigned Material.
    Inputs(Box<DesignInputsError<E>>),
    /// Original sewing/root ownership cause.
    Sewing(Box<DesignSewingError<E>>),
    /// Original candidate garment/current-source cause.
    Candidate(Box<DesignGarmentError<E>>),
}
impl<E: fmt::Display> fmt::Display for CommandPreviewError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(e) => write!(f, "preview request: {e}"),
            Self::Envelope(e) => write!(f, "preview envelope: {e}"),
            Self::Current(e) => write!(f, "preview current sources: {e}"),
            Self::MissingPiece { piece, index } => {
                write!(f, "preview Piece {piece} absent at selection {index}")
            }
            Self::Piece { piece, issue } => write!(f, "preview Piece {piece}: {issue}"),
            Self::Pieces(e) => write!(f, "preview Piece inventory: {e}"),
            Self::Structure(e) => write!(f, "preview structure: {e}"),
            Self::Inputs(e) => write!(f, "preview inputs: {e}"),
            Self::Sewing(e) => write!(f, "preview sewing: {e}"),
            Self::Candidate(e) => write!(f, "preview candidate: {e}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for CommandPreviewError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Command(e) => Some(e.as_ref()),
            Self::Envelope(e) => Some(e.as_ref()),
            Self::Current(e) | Self::Candidate(e) => Some(e.as_ref()),
            Self::MissingPiece { .. } => None,
            Self::Piece { issue, .. } => Some(issue),
            Self::Pieces(e) => Some(e.as_ref()),
            Self::Structure(e) => Some(e.as_ref()),
            Self::Inputs(e) => Some(e.as_ref()),
            Self::Sewing(e) => Some(e.as_ref()),
        }
    }
}

// No constructor, model or replacement handle is exposed outside the bus module.
struct MaterialCandidate<H> {
    state: DesignGarment<H>,
    changes: Vec<CommandMaterialChange>,
}
impl<H: DesignReferenceHost> MaterialCandidate<H> {
    fn into_report(self) -> CommandMaterialPreview {
        CommandMaterialPreview {
            design: self.state.inputs().id(),
            revision: self.state.inputs().revision().clone(),
            changes: self.changes,
        }
    }
}

impl<H: DesignReferenceHost + Clone, A: CommandAuthoritySource> CommandBus<H, A> {
    /// Current inspect-preview subset, independent of executable/committable handlers.
    #[must_use]
    pub const fn is_preview_available(&self, command: CommandId) -> bool {
        matches!(command, CommandId::AssignMaterial)
    }

    /// Prepare one actual AssignMaterial candidate under fresh inspect authority.
    /// Original model, revision, labels, references and identity allocation remain unchanged. The
    /// whole current garment and candidate registry/source validators run; this is no geometry,
    /// resolved physical-property, whole-topology or release certificate. No command is published.
    /// # Errors
    /// Refuses authorization/context first, unsupported preview, explicit envelope/current sources,
    /// missing targets, then original candidate construction/current-registry causes.
    pub fn preview(
        &self,
        request: &CommandRequest,
    ) -> Result<CommandMaterialPreview, CommandPreviewError<H::Error>> {
        self.authorize_bound(request, CommandAuthorizationMode::Preview)
            .map_err(|e| CommandPreviewError::Command(Box::new(e)))?;
        let CommandRequest::Mutation(mutation) = request else {
            return Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable {
                    command: request.command_id(),
                },
            )));
        };
        let MutationPayload::AssignMaterial(assignment) = mutation.payload() else {
            return Err(CommandPreviewError::Command(Box::new(
                CommandBusError::HandlerUnavailable {
                    command: request.command_id(),
                },
            )));
        };
        self.state
            .require_in_scope()
            .map_err(|e| CommandPreviewError::Envelope(Box::new(e)))?;
        self.state
            .validate_current()
            .map_err(|e| CommandPreviewError::Current(Box::new(e)))?;
        let originals = self.state.inputs().structure().pieces().pieces();
        let mut selected = Vec::new();
        for (index, identity) in assignment.pieces().pieces().iter().copied().enumerate() {
            let original = originals
                .iter()
                .find(|piece| piece.id() == identity)
                .ok_or(CommandPreviewError::MissingPiece {
                    piece: identity,
                    index,
                })?;
            selected.push(original);
        }
        let mut changes = Vec::new();
        for original in selected {
            if &original.definition().material != assignment.material() {
                changes.push(CommandMaterialChange {
                    piece: original.id(),
                    before: original.definition().material.clone(),
                    after: assignment.material().clone(),
                });
            }
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
        Ok(MaterialCandidate { state, changes }.into_report())
    }
}
